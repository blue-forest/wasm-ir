pub fn from_u32(value: u32) -> Vec<u8> {
  let mut result = Vec::new();
  let mut current = value;
  loop {
    let byte = (current & 0x7f) as u8;
    current >>= 7;
    if current == 0 {
      result.push(byte);
      break
    }
    result.push(byte + 0x80);
  }
  result
}
