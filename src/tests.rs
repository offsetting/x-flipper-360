use crate::math::{align, log2_ceil, next_pow2};

#[test]
fn test_align() {
  assert_eq!(align(0, 32), 0);
  assert_eq!(align(1, 32), 32);
  assert_eq!(align(32, 32), 32);
  assert_eq!(align(45, 32), 64);
}

#[test]
fn test_log2_ceil() {
  assert_eq!(log2_ceil(1), 0);
  assert_eq!(log2_ceil(2), 1);
  assert_eq!(log2_ceil(3), 2);
  assert_eq!(log2_ceil(4), 2);
  assert_eq!(log2_ceil(5), 3);
  assert_eq!(log2_ceil(6), 3);
  assert_eq!(log2_ceil(7), 3);
  assert_eq!(log2_ceil(8), 3);
  assert_eq!(log2_ceil(9), 4);
}

#[test]
fn test_next_pow2() {
  assert_eq!(next_pow2(0), 0);
  assert_eq!(next_pow2(1), 1);
  assert_eq!(next_pow2(2), 2);
  assert_eq!(next_pow2(3), 4);
  assert_eq!(next_pow2(4), 4);
  assert_eq!(next_pow2(5), 8);
  assert_eq!(next_pow2(6), 8);
  assert_eq!(next_pow2(7), 8);
}
