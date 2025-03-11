#![allow(static_mut_refs)]

use realfft::{RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;
use std::f32::consts::PI;

mod audio;
fn main() {

	let mut audio = audio::Audio::init().unwrap();
	let sample_rate = audio.sample_rate();

	// audio stream thread
	std::thread::spawn(move || {
		loop {
			std::thread::sleep(std::time::Duration::from_millis(1));
			audio.read_stream().unwrap()
		}
	});

	loop {
		// let mut mags: Vec<f32> = vec!();
		// unsafe {
		// 	let mags = compute_fft_mags(&mut audio::SAMPLEBUF);
		// }
		// println!("{:?}", &mags[..10]);
		unsafe {
			std::thread::sleep(std::time::Duration::from_millis(10));
			println!("buf {:?}", audio::SAMPLEBUF[0] / 100000.0);
		}
	}

}

fn hann_window(buf: &mut [f32]) {
	let len = buf.len();
	for i in 0..len {
		buf[i] *= 0.5 * (1.0 - (2.0 * PI * i as f32 / (len as f32)).cos());
	}
}

fn compute_fft_mags(in_buf: &mut [f32]) -> Vec<f32> {
	let size = in_buf.len();
	let mut planner = RealFftPlanner::<f32>::new();
	let fft = planner.plan_fft_forward(size);

	let mut in_buf = in_buf.to_vec();
	let mut out_buf = vec![Complex::new(0.0, 0.0); fft.len()];

	hann_window(&mut in_buf);

	out_buf.iter().map(|c| c.norm()).collect()
}
