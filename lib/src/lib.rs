extern crate num;

use std::mem;
use num::Signed;

#[no_mangle]
pub extern fn murmur(input: &str, seed: i32) -> i32 {
  #![allow(non_snake_case)]

  let mut remainder = input.len() & 3;
  let inputBytes = input.as_bytes();
  let inputChunks = inputBytes.chunks(4);
  let mut h1: u64 = seed as u64;
  let c1: u64 = 0xcc9e2d51;
  let c2: u64 = 0x1b873593;

  let mut h1b: u64;
  let mut k1: u64;

  for chunk in inputChunks {
    if chunk.len() != 4 {
      break;
    }

    unsafe {
      k1 = mem::transmute::<[u8; 4], u32>([chunk[0], chunk[1], chunk[2], chunk[3]]) as u64;
    }

    k1 = (((k1 & 0xffff) * c1) + ((((k1 >> 16) * c1) & 0xffff) << 16)) & 0xffffffff;
    k1 = (k1 << 15) | (k1 >> 17);
    k1 = (((k1 & 0xffff) * c2) + ((((k1 >> 16) * c2) & 0xffff) << 16)) & 0xffffffff;

    h1 ^= k1;
    h1 = (h1 << 13) | (h1 >> 19);
    h1b = (((h1 & 0xffff) * 5) + ((((h1 >> 16) * 5) & 0xffff) << 16)) & 0xffffffff;
    h1 = (((h1b & 0xffff) + 0x6b64) + ((((h1b >> 16) + 0xe654) & 0xffff) << 16)) & 0xffffffff;
  }

  k1 = 0;

  let lastChunk = inputBytes.chunks(4).last().unwrap();

  while remainder > 0 {
    k1 ^= (lastChunk[remainder - 1] as u64) << ((remainder - 1) * 8);
    remainder -= 1;
  }

  k1 = (((k1 & 0xffff) * c1) + ((((k1 >> 16) * c1) & 0xffff) << 16)) & 0xffffffff;
  k1 = (k1 << 15) | (k1 >> 17);
  k1 = (((k1 & 0xffff) * c2) + ((((k1 >> 16) * c2) & 0xffff) << 16)) & 0xffffffff;
  h1 ^= k1;

  h1 ^= inputBytes.len() as u64;

  h1 ^= h1 >> 16;
  h1 = (((h1 & 0xffff) * 0x85ebca6b) + ((((h1 >> 16) * 0x85ebca6b) & 0xffff) << 16)) & 0xffffffff;
  h1 ^= h1 >> 13;
  h1 = (((h1 & 0xffff) * 0xc2b2ae35) + ((((h1 >> 16) * 0xc2b2ae35) & 0xffff) << 16)) & 0xffffffff;
  h1 ^= h1 >> 16;

  return (h1 as i32).abs();
}

#[cfg(test)]
mod tests {
  #![allow(non_snake_case)]
  use super::*;
  
  #[test]
  fn itWorks() {
    assert_eq!(murmur("cheese", 42), 1495267687);
    assert_eq!(murmur("beans", 42), 869695115);
  }
}
