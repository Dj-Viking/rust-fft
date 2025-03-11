#![allow(static_mut_refs)]
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
		println!("sock {:?}", audio.sock);
		//println!("audio samps {:?}", unsafe { audio::SAMPLEBUF.len() });	
	}
}
