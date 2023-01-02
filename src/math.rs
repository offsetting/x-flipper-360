/// Rounds `x` for every `alignment` steps. Always ceiling.
pub(crate) const fn align(x: u32, alignment: u32) -> u32 {
  let count = x / alignment;
  let reminder = x % alignment;

  let mut result = alignment * count;

  if reminder != 0 {
    result += alignment;
  }

  result
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/base/math.h
// see /licenses/xenia.txt
pub(crate) const fn log2_ceil(x: u32) -> u32 {
  if x == 0 {
    return 32;
  }

  // 32 = bits for u32
  32 - (x - 1).leading_zeros()
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/base/math.h
// see /licenses/xenia.txt
pub(crate) const fn next_pow2(y: u32) -> u32 {
  let mut x = y as i32;
  x -= 1;
  x |= x >> 1;
  x |= x >> 2;
  x |= x >> 4;
  x |= x >> 8;
  x |= x >> 16;
  x += 1;
  x as u32
}
