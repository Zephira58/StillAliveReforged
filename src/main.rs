use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::{thread, time::Duration};

fn main() {
   let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

   // Load a sound from a file, using a path relative to Cargo.toml
   let file = File::open("sound.mp3").unwrap();
   let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
   stream_handle.play_raw(source.convert_samples());

   // The sound plays in a separate audio thread,
   // so we need to keep the main thread alive while it's playing.
   // Press ctrl + C to stop the process once you're done.
   print!("\x1B[2J");
   thread::sleep(Duration::from_secs(5));
   print!("This was a triumph\n");
   thread::sleep(Duration::from_secs(1));
   println!("I'm making a note here: HUGE SUCCESS.\n");
   thread::sleep(Duration::from_secs(5));
   
}