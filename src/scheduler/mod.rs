use spin_sleep::SpinSleeper;
use std::time::{Duration, Instant};
pub struct Scheduler {
    target_delta: Duration,
    sleeper: SpinSleeper,
    last_loop_end: Instant,
}

impl Scheduler {
    pub fn fixed_hz(target_hz: f32) -> Self {
        let target_delta = Duration::from_secs_f32(1.0 / target_hz);
        Scheduler {
            target_delta,
            sleeper: SpinSleeper::default(),
            last_loop_end: Instant::now(),
        }
    }

    pub fn loop_forever(&mut self, mut task: impl FnMut()) -> ! {
        loop {
            task();
            self.sleep_until_next_frame();
        }
    }

    pub fn loop_while_true(&mut self, mut task: impl FnMut() -> bool) {
        loop {
            if task() {
                break;
            }
            self.sleep_until_next_frame();
        }
    }

    pub fn loop_until_err<T>(
        &mut self,
        mut task: impl FnMut() -> Result<T, Box<dyn std::error::Error>>,
    ) -> Box<dyn std::error::Error> {
        loop {
            match task() {
                Ok(_) => self.sleep_until_next_frame(),
                Err(e) => return e,
            }
        }
    }

    pub fn sleep_until_next_frame(&mut self) {
        let elapsed = self.last_loop_end.elapsed();
        if elapsed < self.target_delta {
            self.sleeper.sleep(self.target_delta - elapsed);
            self.last_loop_end += self.target_delta;
        } else {
            self.last_loop_end = Instant::now();
        }
    }
}
