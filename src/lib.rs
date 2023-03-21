use std::io::{Read, Write};

use dds::{D3DFormat, Dds};

pub use crate::format::Format;
use crate::format::{get_dds_format, get_format_data, FormatData};
use crate::mip_map::TextureInfo;
pub use crate::texture_header::*;
use crate::tile::{tile, untile};

mod format;
mod math;
mod mip_map;
#[cfg(test)]
mod tests;
mod texture_header;
mod tile;

#[derive(Debug, Copy, Clone)]
pub struct Config {
  pub width: u32,
  pub height: u32,
  pub depth: Option<u32>,
  pub pitch: u32,
  pub tiled: bool,
  pub packed_mips: bool,
  pub format: Format,
  pub mipmap_levels: Option<u32>,
  pub base_address: u32,
  pub mip_address: u32,
}

pub fn convert_to_dds<W: Write>(
  config: &Config,
  src: &[u8],
  output: &mut W,
) -> Result<(), dds::Error> {
  let mut dds = Dds::new_d3d(
    get_dds_format(&config.format),
    config.width,
    config.height,
    config.depth,
    config.mipmap_levels,
    None,
  )?;

  let format_data = get_format_data(&config.format);
  let info = build_texture_info(config, &format_data);

  let mut output_offset = 0;

  for mip in 0..dds.get_num_mipmap_levels() {
    let mut offset_x = 0;
    let mut offset_y = 0;

    let input_offset = info.get_mip_location(mip, &mut offset_x, &mut offset_y, true) as usize;

    let (width, height) = info.get_mip_size(mip);
    let blocks_x = 1.max(width / format_data.block_width);
    let blocks_y = 1.max(height / format_data.block_height);

    let input = &src[input_offset..];
    let output = &mut dds.data[output_offset..];

    output_offset += untile(
      output,
      input,
      &format_data,
      blocks_x,
      blocks_y,
      offset_x,
      offset_y,
    ) as usize;
  }

  // check if the expected length has been written
  if output_offset != dds.data.len() {
    panic!(
      "There were only {} bytes of {} expected written.",
      output_offset,
      dds.data.len()
    );
  }

  dds.write(output)
}

pub fn convert_from_dds<R: Read, W: Write>(
  config: &Config,
  src: &mut R,
) -> Result<Vec<u8>, dds::Error> {
  let dds = Dds::read(src)?;

  let format_data = get_format_data(&config.format);

  let info = build_texture_info(config, &format_data);
  validate_provided_dds(&dds, &info, config.mipmap_levels.unwrap_or(1));

  let mut expected_size = 0;

  for mip in 0..dds.get_num_mipmap_levels() {
    let (width, height) = info.get_mip_size(mip);

    let blocks_x = 1.max(width / format_data.block_width);
    let blocks_y = 1.max(height / format_data.block_height);

    expected_size += (blocks_x * blocks_y * format_data.bytes_per_block) as usize;
  }

  let mut output = vec![0; expected_size];
  let mut input_offset = 0;

  for mip in 0..dds.get_num_mipmap_levels() {
    let mut offset_x = 0;
    let mut offset_y = 0;

    let output_offset = info.get_mip_location(mip, &mut offset_x, &mut offset_y, true) as usize;

    let (width, height) = info.get_mip_size(mip);
    let blocks_x = 1.max(width / format_data.block_width);
    let blocks_y = 1.max(height / format_data.block_height);

    let input = &dds.data[input_offset..];
    let output = &mut output[output_offset..];

    input_offset += tile(
      output,
      input,
      &format_data,
      blocks_x,
      blocks_y,
      offset_x,
      offset_y,
    ) as usize;
  }

  if input_offset != dds.data.len() {
    panic!(
      "There were only {} bytes of {} expected read...",
      input_offset,
      dds.data.len()
    );
  }

  Ok(output)
}

fn build_texture_info<'a>(config: &Config, format_data: &'a FormatData) -> TextureInfo<'a> {
  TextureInfo {
    width: config.width,
    height: config.height,
    depth: config.depth.unwrap_or(1),
    pitch: config.pitch,
    tiled: config.tiled,
    packed_mips: config.packed_mips,
    format: format_data,
    base_address: config.base_address << 12,
    mip_address: config.mip_address << 12,
  }
}

fn validate_provided_dds(dds: &Dds, info: &TextureInfo, mipmap_levels: u32) {
  match dds.get_d3d_format() {
    Some(D3DFormat::DXT1) => {}
    Some(D3DFormat::DXT5) => {}
    _ => panic!("Unsupported image data format."),
  }

  if dds.get_width() != info.width {
    panic!(
      "Provided dds has invalid width, expected was {}.",
      info.width
    );
  }

  if dds.get_height() != info.height {
    panic!(
      "Provided dds has invalid height, expected was {}.",
      info.height
    );
  }

  if dds.get_depth() != info.depth {
    panic!(
      "Provided dds has invalid depth, expected was {}.",
      info.depth
    );
  }

  if dds.get_num_mipmap_levels() != mipmap_levels {
    panic!("Provided dds has invalid mipmap level count, expected was {mipmap_levels}.");
  }
}
