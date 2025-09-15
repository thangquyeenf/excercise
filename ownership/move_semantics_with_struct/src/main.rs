#[derive(Debug)]
struct Buffer {
  data: Vec<u8>
}

impl Buffer {
  fn new(data: Vec<u8>) -> Self {
    Self { data }
  }
}

fn take_buffer (buffer: Buffer) -> usize {
  buffer.data.len()
}

fn take_buffer_ref(buffer: &Buffer) -> usize {
  buffer.data.len()
}

fn main() {
    let buf = Buffer::new(vec![1, 2, 3, 5]);
    println!("{:?}", buf);
    // let size = take_buffer(&buf);
    // println!("{}", size);
    println!("{}", take_buffer_ref(&buf));
    
}
