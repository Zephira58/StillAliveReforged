#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::{thread, time::Duration};
use std::io::Write;
use color_please::*;

fn scroll(s: &str) {
   for c in s.chars() {
       print!("{c}");
       std::io::stdout().flush().expect("Flushing to succeed");
       std::thread::sleep(std::time::Duration::from_millis(75));
   }
}

fn fastscroll(s: &str) {
   for c in s.chars() {
       print!("{c}");
       std::io::stdout().flush().expect("Flushing to succeed");
       std::thread::sleep(std::time::Duration::from_millis(50));
   }
}

fn slowscroll(s: &str) {
   for c in s.chars() {
       print!("{c}");
       std::io::stdout().flush().expect("Flushing to succeed");
       std::thread::sleep(std::time::Duration::from_millis(130));
   }
}

fn prl(s: &str) {
   for c in s.chars() {
       print!("{c}");
       std::io::stdout().flush().expect("Flushing to succeed");
       std::thread::sleep(std::time::Duration::from_millis(100));
   }
}

fn main() {
   let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

   // Load a sound from a file, using a path relative to Cargo.toml
   let file = File::open("sound.mp3").unwrap();
   let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
   stream_handle.play_raw(source.convert_samples());

   // The sound plays in a separate audio thread,
   // so we need to keep the main thread alive while it's playing.
   // Press ctrl + C to stop the process once you're done.

   print!("{esc}c", esc = 27 as char);
   set_fg(Color::Yellow);
   prl("Forns FORM-29827281-12:\n");
   prl("Test Assesement Report\n");
   thread::sleep(Duration::from_secs(2));
   prl("\n");
   prl("\n");
   prl("\n");
   prl("This was a triumph.\n");
   thread::sleep(Duration::from_secs(1));
   prl("I'm making a note here: HUGE SUCCESS.\n");
   thread::sleep(Duration::from_secs(1));
   slowscroll("It's hard to overstate my satisfaction.\n");
   thread::sleep(Duration::from_secs(2));
   prl("Aperture Science\n");
   thread::sleep(Duration::from_secs(2));
   prl("We do what we must because we can.\n");
   thread::sleep(Duration::from_secs(1));
   prl("For the good of all of us.\n");
   scroll("Except the ones who are dead.\n");
   thread::sleep(Duration::from_millis(350));
   scroll("But there's no sense crying over every mistake.\n");
   scroll("You just keep on trying till you run out of cake.\n");
   scroll("And the Science gets done.\n");
   scroll("And you make a neat gun.\n");
   scroll("For the people who are still alive.\n");
   thread::sleep(Duration::from_secs(1));
   print!("{esc}c", esc = 27 as char);
   set_fg(Color::Yellow);
   scroll("Personal File Addendun: \n");
   scroll("Dear <<Subject Name Here>> \n");
   scroll("\n\n");
   thread::sleep(Duration::from_millis(200));
   scroll("\nI'm not even angry.\n");
   thread::sleep(Duration::from_millis(1300));
   prl("I'm being so sincere right now.\n");
   thread::sleep(Duration::from_secs(2));
   prl("Even though you broke my heart.\nAnd killed me.\n");
   thread::sleep(Duration::from_millis(2000));
   prl("And tore me to pieces.\n");
   thread::sleep(Duration::from_millis(1500));
   prl("And threw every piece into a fire.\n");
   thread::sleep(Duration::from_millis(2500));
   fastscroll("As they burned it hurt because.\nI was so happy for you!\n");
   thread::sleep(Duration::from_millis(450));
   scroll("Now these points of data make a beautiful line.\n");
   prl("And we're out of beta.\n");
   scroll("We're releasing on time.\n");
   scroll("So I'm GLaD. I got burned.\n");
   scroll("Think of all the things we learned\n");
   scroll("for the people who are still alive.\n");
   thread::sleep(Duration::from_millis(500));
   print!("{esc}c", esc = 27 as char);
   set_fg(Color::Yellow);
   scroll("Forns FORM-55551-6:\n");
   scroll("Personal File Addendun Addendun:\n");
   fastscroll("\nOne last thing:\n");
   scroll("\nGo ahead and leave me.\n");
   thread::sleep(Duration::from_millis(1000));
   prl("I think I prefer to stay inside.\n");
   thread::sleep(Duration::from_secs(2));
   prl("Maybe you'll find someone else to help you.\n");
   thread::sleep(Duration::from_secs(2));
   slowscroll("Maybe Black Mesa...\n");
   thread::sleep(Duration::from_secs(1));
   prl("THAT WAS A JOKE.. HAHA. FAT CHANCE.\n");
   thread::sleep(Duration::from_secs(2));
   scroll("Anyway, this cake is great.\n");
   thread::sleep(Duration::from_millis(300));
   scroll("It's so delicious and moist.\n");
   scroll("Look at me still talking\n");
   scroll("when there's Science to do.\n");
   scroll("When I look out there, it makes me GLaD I'm not you.\n");
   scroll("I've experiments to run.\n");
   scroll("There is research to be done.\n");
   scroll("On the people who are still alive.\n");
   thread::sleep(Duration::from_secs(2));
   print!("{esc}c", esc = 27 as char);
   set_fg(Color::Yellow);
   fastscroll("PS: And belive me i am\n");
   scroll("still alive.\n");
   thread::sleep(Duration::from_secs(1));
   scroll("\nPPS: I'm doing Science and I'm\nstill alive.\n");
   thread::sleep(Duration::from_secs(1));
   scroll("\nPPPS: I feel FANTASTIC and I'm\nstill alive.\n");
   thread::sleep(Duration::from_secs(1));
   fastscroll("\n FINAL THOUGHT:\n");
   fastscroll("While you're dying I'll be\nstill alive.\n");
   fastscroll("\n FINAL THOUGHT PS:\n");
   fastscroll("And when you're dead I will be\nstill alive.\n");
   prl("\nSTILL ALIVE\n");
   slowscroll("Still alive..");

}