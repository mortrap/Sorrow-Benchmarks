#![feature(test)]

extern crate test;


#[cfg(test)]
mod tests {
  use gstuff::rdtsc;
  use std::mem::MaybeUninit;
  use test::{Bencher, black_box};

  #[bench]
  fn strings (b: &mut Bencher) {
    let mut rope = strings::rope::Rope::new();
    println! ("{}", rdtsc() % 65536);

    b.iter(|| {
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};
      rope.push_copy (text);
      if rope.len() > 1024 * 1024 {rope.remove (0, 314 * 1024)}
    });
  }
}

