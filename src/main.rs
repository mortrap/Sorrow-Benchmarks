#![allow(unknown_lints, uncommon_codepoints)]

#![feature(test)]

extern crate test;

use fomat_macros::fomat;
use gstuff::{re::Re, fail};


#[cfg(test)]
mod tests {
  use fomat_macros::fomat;
  use gstuff::rdtsc;
  use imbl::Vector;
  use std::{mem::MaybeUninit, collections::VecDeque};
  use test::{Bencher};
  use jumprope::JumpRope;
  use ropey::Rope;
  use gstuff::re::Re;
  use pyo3::types::IntoPyDict;
  use pyo3::Python;
  use std::fs;

use crate::parse_warc;
  
  

  
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
//  не могу прочитать 5гб варк файл?
#[bench]
fn python_code (_ben: &mut Bencher){
  fn python_codeʹ() -> Re<()>{
    let path = warc_file();
    std::env::set_var ("WARC_FILE", path);

    let gil = Python::acquire_gil();
    let py = gil.python();
    let script = fomat! (
      "import sys\n"
      r#"sys.path.append ("../Prototype-Parsing")"# "\n"
      "import main"
    );
    py.run (&script, None, None)?;
    Re::Ok(())
  }
  python_codeʹ ().unwrap();
}

  #[bench]
  fn fast_warc (_ben: &mut Bencher){
    fn fast_warcʹ() -> Re<()>{
      let gil = Python::acquire_gil();
      let py = gil.python();
      let script = fomat! (
        // https://github.com/chatnoir-eu/chatnoir-resiliparse/tree/develop/fastwarc
        // https://resiliparse.chatnoir.eu/en/latest/man/fastwarc.html
        // pip install --user fastwarc
        // pip install --user extruct
        "from fastwarc.warc import ArchiveIterator, is_http\n"
        "import extruct\n"
        // ⌥ 
        "for record in ArchiveIterator(open('c:/Users/artem/Downloads/qwe/CC-MAIN-20210224165708-20210224195708-00000.warc', 'rb'), func_filter=is_http):\n"
        "  print(record.record_id)\n"
        "  reader = record.reader\n"
        "  body = record.reader.read(1024 * 1024)\n"
        "  body = body.decode()\n"
        "  if 'application/ld+json' in body:\n"
        "    try:\n"
        "      metadata = extruct.extract(body)\n"
        "      print(metadata)\n"
        "    except Exception:\n"
        "      print('exception')\n"
        "  record.reader.consume()\n"
      );
      py.run (&script, None, None)?;
      Re::Ok(())
    }
    fast_warcʹ ().unwrap();
  }

  fn warc_file() -> String {
    let mut args = std::env::args();
    args.next().unwrap(); args.next().unwrap(); args.next().unwrap();
    args.next().unwrap()}

  #[bench]
  fn warc_streaming (_ben: &mut Bencher) {
    fn warc_streamingʹ () -> Re<()> {
      let path = warc_file();
      let mut file = std::fs::File::open (path)?;
      parse_warc (&mut file)?;
      Re::Ok(())
    }
    warc_streamingʹ().unwrap();
  }
}
fn parse_warc (warc: &mut dyn std::io::Read) -> Re<()> {
  let capacity = 2 * 1024 * 1024;
  let mut buf: Vec<u8> = Vec::with_capacity (capacity);
  unsafe {buf.set_len (buf.capacity())};
  let mut buf = &mut buf[..];

  let mut start = 0;
  let mut end = 0;
  let mut eof = false;
  let mut total = 0;

  // Absolute WARC position of `start` is `total - (end - start)`.
  macro_rules! warc_pos {() => {total - (end - start)}}

  loop {
    unsafe {std::ptr::copy_nonoverlapping (buf.as_mut_ptr().add (start), buf.as_mut_ptr(), end - start)}
    end -= start;
    start = 0;

    loop {
      if capacity / 5 * 4 < end {break}
      let got = warc.read (&mut buf[end..])?;
      total += got;
      end += got;
      if got == 0 {eof = true; break}
    }

    loop {
      // Invariant: `start` points at WARC headers here
      if end - start < 1024 {break}  // read more

      let newlineʹ = memchr::memchr (b'\n', &buf[start..end]) ?;
      if newlineʹ <= 4 {fail! ("Not a WARC header at " (warc_pos!()))}
      let newline = start + newlineʹ;
      let line = &buf[start .. newline-1];
      start = newline + 1;


      if line.starts_with (b"Content-Length: ") {
        let cl = unsafe {std::str::from_utf8_unchecked (&line[16..])};
        let cl: usize = cl.parse()?;
        let head_end = start + memchr::memmem::find (&buf[start..end], b"\n\n") ?;
        // Newlines before content + content + newlines after content
        start = head_end + 2 + cl + 2;
        println! ("head_end {head_end}; start {start}");
        while buf[start] == b'\n' {start += 1}
        println! ("{start} b");
        //println! ("warc doc found between {} and {}, cl {}", head_end + 2, start, cl);
      }
    }

    if eof {break}
  }
  Re::Ok(())
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


/*
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
*/
