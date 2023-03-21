use std::fs::File;
use std::io::Read;
use binrw::__private::assert;
use crate::{build_texture_info, convert_to_dds};
use crate::format::get_format_data;
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

#[test]
fn extract_file() {
  let config = crate::Config {
    width: 128,
    height: 128,
    depth: Some(1),
    pitch: 4,
    tiled: true,
    packed_mips: true,
    format: crate::Format::Dxt5,
    mipmap_levels: Some(8),
    base_address: 0,
    mip_address: 4,
  };

  let mut image_file = File::open("data/image_body.bin").expect("Cant open file");
  let mut image_data = [0u8; 65536];
  image_file.read(&mut image_data).expect("TODO: panic message");

  let mut dds_file = File::create("data/image_result.dds").expect("");

  convert_to_dds(&config, &image_data, &mut dds_file).expect("");
}

#[test]
fn get_mip_location() {
  let config = crate::Config {
    width: 128,
    height: 128,
    depth: Some(1),
    pitch: 4,
    tiled: true,
    packed_mips: true,
    format: crate::Format::Dxt5,
    mipmap_levels: Some(8),
    base_address: 0,
    mip_address: 4,
  };

  let format_data = get_format_data(&config.format);
  let info = build_texture_info(&config, &format_data);

  let mut offset_x = 0;
  let mut offset_y = 0;
  let mut mip_offset = 0;

  mip_offset = info.get_mip_location(1, &mut offset_x, &mut offset_y, true);

  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);
  assert_eq!(mip_offset, 16384);

  mip_offset = info.get_mip_location(3, &mut offset_x, &mut offset_y, true);

  assert_eq!(offset_x, 4);
  assert_eq!(offset_y, 0);
  assert_eq!(mip_offset, 49152);

  mip_offset = info.get_mip_location(0, &mut offset_x, &mut offset_y, true);

  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);
  assert_eq!(mip_offset, 0);

  mip_offset = info.get_mip_location(8, &mut offset_x, &mut offset_y, true);

  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);
  assert_eq!(mip_offset, 49152);

  mip_offset = info.get_mip_location(7, &mut offset_x, &mut offset_y, true);

  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 1);
  assert_eq!(mip_offset, 49152);

  let (width, height) =  info.get_mip_size(4);
  assert_eq!(width, 8);
  assert_eq!(height, 8);

  info.get_packed_tile_offset(8, &mut offset_x, &mut offset_y);
  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);

  info.get_packed_tile_offset(0, &mut offset_x, &mut offset_y);
  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);

  info.get_packed_tile_offset(7, &mut offset_x, &mut offset_y);
  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);

  info.get_packed_tile_offset0(2, 2, 16, &mut offset_x, &mut offset_y);
  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 0);

  info.get_packed_tile_offset0(16, 16, 3, &mut offset_x, &mut offset_y);
  assert_eq!(offset_x, 0);
  assert_eq!(offset_y, 2);

  // print y-coordinates
}
