use crate::format::FormatData;

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_conversion.cc
// see /licenses/xenia.txt
pub(crate) fn untile(
  output_buffer: &mut [u8],
  input_buffer: &[u8],
  format: &FormatData,
  blocks_x: u32,
  blocks_y: u32,
  offset_x: u32,
  offset_y: u32,
) -> u32 {
  let bytes_per_block = format.bytes_per_block;

  // Bytes per pixel
  let log2_bpp = (bytes_per_block / 4) + ((bytes_per_block / 2) >> (bytes_per_block / 4));

  // Offset of the writer
  let mut output_offset = 0;

  for y in 0..blocks_y {
    let input_row_offset = tiled_offset_2d_row(y + offset_y, blocks_x, log2_bpp);

    for x in 0..blocks_x {
      let mut input_offset =
        tiled_offset_2d_column(x + offset_x, y + offset_y, log2_bpp, input_row_offset);
      input_offset >>= log2_bpp;

      copy(
        output_buffer,
        output_offset as usize,
        input_buffer,
        (input_offset * bytes_per_block) as usize,
        bytes_per_block as usize,
      );

      output_offset += bytes_per_block;
    }
  }

  blocks_x * blocks_y * bytes_per_block
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_conversion.cc
// see /licenses/xenia.txt
pub(crate) fn tile(
  output_buffer: &mut [u8],
  input_buffer: &[u8],
  format: &FormatData,
  blocks_x: u32,
  blocks_y: u32,
  offset_x: u32,
  offset_y: u32,
) -> u32 {
  let bytes_per_block = format.bytes_per_block;

  // Bytes per pixel
  let log2_bpp = (bytes_per_block / 4) + ((bytes_per_block / 2) >> (bytes_per_block / 4));

  // Offset of the reader
  let mut input_offset = 0;

  for y in 0..blocks_y {
    let input_row_offset = tiled_offset_2d_row(y + offset_y, blocks_x, log2_bpp);

    for x in 0..blocks_x {
      let mut output_offset =
        tiled_offset_2d_column(x + offset_x, y + offset_y, log2_bpp, input_row_offset);
      output_offset >>= log2_bpp;

      copy(
        output_buffer,
        (output_offset * bytes_per_block) as usize,
        input_buffer,
        input_offset as usize,
        bytes_per_block as usize,
      );

      input_offset += bytes_per_block;
    }
  }

  blocks_x * blocks_y * bytes_per_block
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_conversion.cc
// see /licenses/xenia.txt
const fn tiled_offset_2d_row(y: u32, width: u32, log2_bpp: u32) -> u32 {
  let macro0 = ((y / 32) * (width / 32)) << (log2_bpp + 7);
  let micro = ((y & 6) << 2) << log2_bpp;
  macro0 + ((micro & !0xF) << 1) + (micro & 0xF) + ((y & 8) << (3 + log2_bpp)) + ((y & 1) << 4)
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_conversion.cc
// see /licenses/xenia.txt
const fn tiled_offset_2d_column(x: u32, y: u32, log2_bpp: u32, base_offset: u32) -> u32 {
  let macro0 = (x / 32) << (log2_bpp + 7);
  let micro = (x & 7) << log2_bpp;
  let offset = base_offset + (macro0 + ((micro & !0xF) << 1) + (micro & 0xF));
  ((offset & !0x1FF) << 3)
    + ((offset & 0x1C0) << 2)
    + (offset & 0x3F)
    + ((y & 16) << 7)
    + (((((y & 8) >> 2) + (x >> 3)) & 3) << 6)
}

fn copy(
  output_buffer: &mut [u8],
  output_offset: usize,
  input_buffer: &[u8],
  input_offset: usize,
  count: usize,
) {
  for i in (0..count).step_by(2) {
    if output_buffer[output_offset + i] != 0x0 || output_buffer[output_offset + i + 1] != 0x0 {
      panic!("Overridden already written space...")
    }

    // swapping every two bytes
    output_buffer[output_offset + i] = input_buffer[input_offset + i + 1];
    output_buffer[output_offset + i + 1] = input_buffer[input_offset + i];
  }
}
