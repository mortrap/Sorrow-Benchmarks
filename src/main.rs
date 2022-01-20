#![feature(test)]
extern crate test;


#[cfg(test)]
mod tests {
  use gstuff::rdtsc;
  use imbl::Vector;
  use std::{mem::MaybeUninit, collections::VecDeque};
  use test::{Bencher};
  use jumprope::*;
  use ropey::Rope;
  
  

  #[bench]
  fn strings (b: &mut Bencher) {
    let mut rope = strings::rope::Rope::new();    
    
    b.iter(|| {
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};

      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};//?
      rope.push_copy(text);//cool
      if rope.len() > 1024 * 1024 {rope.remove (0, 314 * 1024)};
    }); 
  }

  /// Fails on non-unicode.
  #[bench]
  fn jumpstrings (b: &mut Bencher){
    let mut jrope = JumpRope::new();
     let mut c =0;
    b.iter(|| {
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};
      jrope.insert (jrope.len_chars(), text);
        c=c+1;
      if jrope.len_bytes() > 1024*1024 {jrope.remove (0..314*1024)};    
    });
  }

  #[bench]
  fn ropeytest(b:&mut Bencher){
    let mut ropeys = Rope::new();
    
    b.iter(||{
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc() % 256) as u8;
      let text = unsafe {std::str::from_utf8_unchecked (&buf)};
      ropeys.insert (0, text);
      
      if ropeys.len_bytes() > 1024*1024 {ropeys.remove (0..314*1024)};
    });
  }

  #[bench]
  fn vect(b:&mut Bencher){
    let mut vector = Vec::<u8>::new();
    b.iter(||{
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc()% 256) as u8;
      // let text = unsafe {std::str::from_utf8(&buf)};
    
      vector.extend_from_slice(&buf);
      if vector.len() > 1024*1024 {vector.drain (0..314*1024);}
    });
  }

  #[bench]
  fn deque (b:&mut Bencher){
    let mut deque = VecDeque::<u8>::new();
    b.iter (||{
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc()% 256) as u8;
      // let text = unsafe {std::str::from_utf8(&buf)};
    
      for ch in buf {deque.push_back (ch)}
      if deque.len() > 1024*1024 {deque.drain (0..314*1024);}
    })
  }

  // No batch `extend`?
  #[bench]
  fn imvec(b:&mut Bencher){
    let mut imvec: Vector<u8> = Vector::new();
    b.iter(||{
      let mut buf: [u8; 65536] = unsafe {MaybeUninit::uninit().assume_init()};
      buf[rdtsc() as usize % buf.len()] = (rdtsc()% 256) as u8;
      //let text = unsafe {std::str::from_utf8(&buf)};
    
      for ch in &buf {
        imvec.push_back (*ch)
      }
      //imvec.extend_from_slice(&buf);
      //if imvec.len() > 1024*1024 {imvec.drain (0..314*1024);}
    });
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



fn main() {
  Python::with_gil(|py| {
      let custom_manager = PyModule::from_code(py, r#"
class House(object):
  def __init__(self, address):
      self.address = address
  def __enter__(self):
      print(f"Welcome to {self.address}!")
  def __exit__(self, type, value, traceback):
      if type:
          print(f"Sorry you had {type} trouble at {self.address}")
      else:
          print(f"Thank you for visiting {self.address}, come again soon!")

      "#, "house.py", "house").unwrap();

      let house_class = custom_manager.getattr("House").unwrap();
      let house = house_class.call1(("123 Main Street",)).unwrap();

      house.call_method0("__enter__").unwrap();

      let result = py.eval("undefined_variable + 1", None, None);

      // If the eval threw an exception we'll pass it through to the context manager.
      // Otherwise, __exit__  is called with empty arguments (Python "None").
      match result {
          Ok(_) => {
              let none = py.None();
              house.call_method1("__exit__", (&none, &none, &none)).unwrap();
          },
          Err(e) => {
              house.call_method1(
                  "__exit__",
                  (e.ptype(py), e.pvalue(py), e.ptraceback(py))
              ).unwrap();
          }
      }
  })
}

