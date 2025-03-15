#![allow(static_mut_refs)]

use rustfft::{FftPlanner, num_complex::Complex};

use rustfft::num_complex::ComplexFloat;
use std::f32::consts::PI;

mod audio;
fn main() {

	let mut audio = audio::Audio::init().unwrap();
	let sample_rate = audio.sample_rate();

	// audio stream thread
	std::thread::spawn(move || {
		loop {
			let _ = audio.read_stream().unwrap();
		}
	});

	loop {
		// unsafe {
		// 	if audio::SAMPLEBUF.len() < 4096 {
		// 		println!("{}", audio::SAMPLEBUF.len());
		// 		break;
		// 	}
		// 	println!("{}", audio::SAMPLEBUF.len());
		// 	// break;
		// }
		let mut mags: Vec<f32> = vec!();
		unsafe {
			// println!("final buf{:?}", audio::FINAL_SAMPLEBUF);
			let mags = compute_fft_mags(&mut audio::FINAL_SAMPLEBUF);
			// println!("mags {:?}", mags);
		}
	}

}

fn hann_window(buf: &mut [f32]) {
	let len = buf.len();
	for i in 0..len {
		let mul = (1.0 - (2.0 * PI * i as f32 / ((len - 1) as f32)).cos());
		buf[i] *= 0.5 * mul;
	}
}

fn compute_fft_mags(in_buf: &mut [f32]) -> Vec<f32> {
	let size = in_buf.len();
	// println!("sizse {}", size);
	let mut planner = FftPlanner::new();
	let fft = planner.plan_fft_forward(size);
	// println!("fft thing{:?}", fft.len());

	let mut in_buf = in_buf.to_vec();
	hann_window(&mut in_buf);
	let mut out_fft: Vec<Complex<f32>> = in_buf.iter().map(|&x| Complex::new(x, 0.0)).collect();

	// println!("in buf len {}", in_buf.len());


	// println!("in buf len {}", in_buf.len());

	fft.process(&mut out_fft);

	out_fft.iter().map(|c| c.l1_norm()).collect()
}
