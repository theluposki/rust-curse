fn main() {
  let character: char = 'a';
  let string: &str = "texto"; // &str
  let signed: i8 = -23; // i8(8 bits), i16 (16 bits), i32 (32 bits), i64 (64 bits), i128 (128 bits) 
  let unsigned: u8 = 23; // i8(8 bits), i16 (16 bits), i32 (32 bits), i64 (64 bits), i128 (128 bits) 
  let float: f32 = 21.54; // f32(32 bits), f64 (64 bits)
  let boolean: bool = false; // true or false

  println!("signed: {}, unsigned: {}, float: {}, boolean: {} char: {}, String: {}", signed, unsigned, float, boolean, character, string);
}
