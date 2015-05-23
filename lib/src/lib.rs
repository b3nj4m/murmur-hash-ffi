use std::mem;

#[no_mangle]
pub extern fn murmur(input: &str, seed: u32) -> u32 {
  #![allow(non_snake_case)]

  let mut remainder = input.len() & 3;
  let inputBytes = input.as_bytes();
  let inputChunks = inputBytes.chunks(4);
  let mut h1: u32 = seed;
  let c1: u32 = 0xcc9e2d51;
  let c2: u32 = 0x1b873593;

  let mut h1b: u32;
  let mut k1: u32;

  for chunk in inputChunks {
    if chunk.len() != 4 {
      break;
    }

    unsafe {
      k1 = mem::transmute::<[u8; 4], u32>([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }

    k1 = ((((k1 & 0xffff) * c1) + ((((k1 >> 16) * c1) & 0xffff) << 16))) & 0xffffffff;
    k1 = (k1 << 15) | (k1 >> 17);
    k1 = ((((k1 & 0xffff) * c2) + ((((k1 >> 16) * c2) & 0xffff) << 16))) & 0xffffffff;

    h1 ^= k1;
    h1 = (h1 << 13) | (h1 >> 19);
    h1b = ((((h1 & 0xffff) * 5) + ((((h1 >> 16) * 5) & 0xffff) << 16))) & 0xffffffff;
    h1 = ((h1b & 0xffff) + 0x6b64) + ((((h1b >> 16) + 0xe654) & 0xffff) << 16);
  }

  k1 = 0;

  let lastChunk = inputBytes.chunks(4).last().unwrap();
  //TODO inefficient
  while remainder > 0 {
    match remainder {
      3 => k1 ^= (lastChunk[3] as u32) << 16,
      2 => k1 ^= (lastChunk[2] as u32) << 8,
      1 => k1 ^= lastChunk[1] as u32,
      _ => {}
    }
    remainder -= 1;
  }

  k1 = (((k1 & 0xffff) * c1) + ((((k1 >> 16) * c1) & 0xffff) << 16)) & 0xffffffff;
  k1 = (k1 << 15) | (k1 >> 17);
  k1 = (((k1 & 0xffff) * c2) + ((((k1 >> 16) * c2) & 0xffff) << 16)) & 0xffffffff;
  h1 ^= k1;

  h1 ^= inputBytes.len() as u32;

  h1 ^= h1 >> 16;
  h1 = (((h1 & 0xffff) * 0x85ebca6b) + ((((h1 >> 16) * 0x85ebca6b) & 0xffff) << 16)) & 0xffffffff;
  h1 ^= h1 >> 13;
  h1 = ((((h1 & 0xffff) * 0xc2b2ae35) + ((((h1 >> 16) * 0xc2b2ae35) & 0xffff) << 16))) & 0xffffffff;
  h1 ^= h1 >> 16;

  return h1 >> 0 as u32;
}
