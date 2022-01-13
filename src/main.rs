#![feature(test)]
extern crate test;


#[cfg(test)]
mod tests {
  use gstuff::rdtsc;
  use strings::rope;
  use std::{mem::MaybeUninit, io::BufRead};
  use test::{Bencher, black_box};
  use jumprope::*;
  use ropey::Rope;
  use std::io::{BufReader, BufWriter};

  #[bench]
  fn strings (b: &mut Bencher) {
    let mut rope = strings::rope::Rope::new();
    println! ("{}", rdtsc() % 65536);
    
    
    b.iter(|| {
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};

      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};//?
      rope.push_copy(text);//cool
      if rope.len() > 1024 * 1024 {rope.remove (0, 314 * 1024)};
    });
    
      
      println!("{}", rope.len());
 
  }
  #[bench]
  fn jstrings (b: &mut Bencher){
    let mut jrope = JumpRope::new();
     let mut c =0;
    println! ("{}", rdtsc() % 65536);
    b.iter(|| {
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};
      jrope.insert(0, text);//have not push_copy analog
      //  c=c+1;
      if jrope.len_bytes() > 1024*1024 {jrope.remove (0..314*1024)};  
      
  
    });
      println!("{}", jrope.to_string().len())
    
  
  }#[bench]
  fn ropey_strings(b:&mut Bencher){
    let mut ropeys = Rope::new();
    let mut c =0;
    println! ("{}", rdtsc() % 65536);
    b.iter(||{
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};
      ropeys.insert(0, text);//?
      c=c+1;
      if ropeys.len_bytes() > 1024*1024 {ropeys.remove (0..314*1024)};
    });
    println!("{}", ropeys.to_string().len())
  }
}

fn main() {
  println!("Hello");
    /*let mut jrope = JumpRope::new();
    rope.insert(5, "really "); // "Some really large text document"
    rope.replace(0..4, "My rad");  // "My rad really large text document"
    assert_eq!(rope, "My rad really large text document");

    // Extract to a string
    let s: String = rope.to_string();
    assert_eq!(s, "My rad really large text document");*/
}



