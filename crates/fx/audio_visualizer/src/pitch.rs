// "pitch" - Licensed under the Boost Software License (see /LICENSE)

//! Quickly and accurately determine the pitch and volume of a sound sample.
//!
//! This crate uses a
//! [Bitstream Autocorrelation Function (BCF)
//! ](http://www.cycfi.com/2018/03/fast-and-efficient-pitch-detection-bitstream-autocorrelation/)
//! invented by Joel de Guzman to determine the pitch of the sound sample.
//!
//! # How to use
//! It is really quite simple.  Just write:
//! ```
//! let (hz, amplitude) = pitch::detect(&samples)
//! ```
//! `samples` is a fixed-size array of f32s.  I use 2,048 for the array length.
//! The length of the array should be 2 times the period of the lowest note you
//! want to pick up (because of math and some theories).  So 2,048 gives us
//! notes up to 1,024 samples per period, which in 48,000Hz is about 47Hz
//! (48,000Hz / 1,024 Samples).  For reference, the lowest sine wave that a
//! human can hear is 20Hz (a little more than an octave lower).
//!
//! # Example
//! Example code can be found in `test.rs`.  The audio files I used were me
//! playing my trombone on notes A1, A2, A3, and A4, and generating sine, saw,
//! and square waves with Audacity on the note A4 (making sure to set 48,000hz
//! sample rate).  Here is my example output (Note: I didn't tune my trombone
//! beforehand since this program is a tuner, and may give different results
//! than other tuners):
//!
//! ```none
//! A1: 55.422962 Hz, 0.7379681 Vl
//! A2: 107.865166 Hz, 0.43305764 Vl
//! A3: 215.78847 Hz, 0.69536424 Vl
//! A4: 436.30658 Hz, 0.117435224 Vl
//! SINE_A4: 439.99805 Hz, 0.8001343 Vl
//! SAW_A4: 439.99167 Hz, 0.8001343 Vl
//! SQUARE_A4: 434.3891 Hz, 0.80022585 Vl
//! ```

type SampleType = f32;

// BCF constants:
const SPS: u32 = 48_000; // Sample Hz
const MAX_FREQ: SampleType = 10_000.0; // Stupidly high note
const MIN_PERIOD: SampleType = (SPS as SampleType) / MAX_FREQ; // Minumum Period Samples

const NBITS: usize = ::std::mem::size_of::<usize>() * 8;

struct ZeroCross(bool);

impl ZeroCross {
	fn new() -> Self {
		ZeroCross(false)
	}

	fn get(&mut self, s: SampleType, t: SampleType) -> bool {
		if s < -t {
			self.0 = false;
		} else if s > t {
			self.0 = true;
		}

		self.0
	}
}

struct BitStream {
	bits: Vec<usize>,
	len: usize,
}

impl BitStream {
	fn new(samples: &[SampleType], threshold: SampleType) -> Self {
		let mut zc = ZeroCross::new();
		let mut bin = BitStream {
			bits: Vec::with_capacity(samples.len() / NBITS),
			len: samples.len(),
		};

		let mut i = 0;
		'a: loop {
			let mut register = 0usize;
			for shift in 0..NBITS {
				let setv = zc.get(samples[i], threshold);
				if setv {
					register ^= (::std::usize::MAX
						^ register) & (1 << shift);
				}
				i += 1;
				if i == samples.len() {
					bin.bits.push(register);
					break 'a;
				}
			}
			bin.bits.push(register);
		}

		bin
	}

	fn get(&self, index: usize, shift: usize) -> usize {
		let v = self.bits[index];
		if shift > 0 {
			v >> shift | self.bits[index + 1] << (NBITS - shift)
		} else {
			v
		}
	}

	fn autocorrelate(&self) -> usize {
		let start_pos = MIN_PERIOD as usize;
		let mut min_count = ::std::u32::MAX;
		let mut est_index = 0usize;

		let mid_array = (self.bits.len() / 2) - 1;
		let mid_pos = self.len / 2;
		let mut index = start_pos / NBITS;
		let mut shift = start_pos % NBITS;

		// get autocorrelation values for the first half of the sample.
		for pos in start_pos..mid_pos {
			let mut count = 0;
			for i in 0..mid_array {
				count += (self.get(i, 0)
						^ self.get(i + index, shift))
					 .count_ones();
			}
			shift += 1;
			if shift == NBITS {
				shift = 0;
				index += 1;
			}

			if count < min_count {
				min_count = count;
				est_index = pos;
			}
		}

		est_index
	}
}

fn bcf(samples: &[SampleType]) -> Option<(SampleType, SampleType)> {
	// Get The Amplitude (Volume).
	let mut volume: SampleType = 0.0;
	for i in samples.iter() {
		volume = volume.max(i.abs());
	}

	// Convert Into a Bitstream of Zero-Crossings
	let bin = BitStream::new(samples, volume * 0.00001);

	// Binary Autocorrelation
	let est_index = bin.autocorrelate();

	// println!("Zero-Crossing Autocorrelation Hz: {}", (SPS as SampleType) / (est_index as SampleType));

    return Some(((SPS as SampleType) / (est_index as SampleType), volume));

	// Estimate the pitch:
	// - Get the start edge
	let mut prev = 0.0;
	let mut esam = samples.iter().enumerate();
	let start_edge = loop {
		let (i, start_edge2) = esam.next()?;
		if *start_edge2 > 0.0 {
			break (i, start_edge2);
		}
		prev = *start_edge2;
	};

	let mut dy = start_edge.1 - prev;
	let dx1 = -prev / dy;

	// - Get the next edge
	let mut nsam = esam.skip(est_index - start_edge.0 - 1);
	let next_edge = loop {
		let (i, next_edge2) = nsam.next()?;
		if *next_edge2 > 0.0 {
			break (i, next_edge2);
		}
		prev = *next_edge2;
	};

	dy = next_edge.1 - prev;
	let dx2 = -prev / dy;

	let n_samples: SampleType = (next_edge.0 - start_edge.0) as SampleType + (dx2 - dx1);

	println!("Final (correction) Hz: {}", (SPS as SampleType) / n_samples);
//	println!("{} {} {} {} {} {}", est_index, n_samples, next_edge.0, start_edge.0, dx2, dx1);

	// The frequency
	Some(((SPS as SampleType) / n_samples, volume))
}

/// Do the BCF calculation on raw samples.  Returns `(hz, amplitude[0-1])`.
pub fn detect(samples: &[SampleType]) -> (SampleType, SampleType) {
	bcf(samples).unwrap_or((0.0, 0.0))
}