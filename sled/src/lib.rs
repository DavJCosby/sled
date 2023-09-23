use palette::Srgb;

pub struct SLED {
    leds: LEDs,
}

pub struct LEDs {
    leds: Vec<Srgb>,
}

impl SLED {
    pub fn new(config_file_path: &str) -> Self {
        // 1. deserialize config file

        // 2. process led segments

        // 3. find sum led count. Now that we have that we know how many color values to initialize
        let num_leds = 50;
        let leds = vec![Srgb::new(0.0, 0.0, 0.0); num_leds];
        // 4. create various utility maps to help us track down the specific led we are looking for.

        // 5. construct
        SLED {
            leds: LEDs { leds },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
