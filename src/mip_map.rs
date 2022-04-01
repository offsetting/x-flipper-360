use crate::format::FormatData;
use crate::math::{align, log2_ceil, next_pow2};

#[derive(Debug, Copy, Clone)]
pub(crate) struct TextureInfo<'a> {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) depth: u32,
  pub(crate) pitch: u32,
  pub(crate) tiled: bool,
  pub(crate) packed_mips: bool,
  pub(crate) format: &'a FormatData,
  pub(crate) base_address: u32,
  pub(crate) mip_address: u32,
}

#[derive(Debug, Copy, Clone)]
struct TextureExtent {
  pitch: u32,
  height: u32,
  block_pitch_h: u32,
  block_pitch_v: u32,
  depth: u32,
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_info.cc
// see /licenses/xenia.txt
impl<'a> TextureInfo<'a> {
  pub(crate) fn get_mip_size(&self, mip: u32) -> (u32, u32) {
    (
      1.max(self.width / 2_u32.pow(mip)),
      1.max(self.height / 2_u32.pow(mip)),
    )
  }

  pub(crate) fn get_mip_location(
    &self,
    mip: u32,
    offset_x: &mut u32,
    offset_y: &mut u32,
    is_guest: bool,
  ) -> u32 {
    if mip == 0 {
      // Short-circuit. Mip 0 is always stored in base_address.
      if !self.packed_mips {
        *offset_x = 0;
        *offset_y = 0;
      } else {
        self.get_packed_tile_offset(0, offset_x, offset_y);
      }
      return self.base_address;
    }

    if self.mip_address == 0 {
      // Short-circuit. There is no mip data.
      *offset_x = 0;
      *offset_y = 0;
      return 0;
    }

    let address_base: u32 = self.mip_address;
    let mut address_offset: u32 = 0;

    let bytes_per_block = self.format.bytes_per_block;

    if !self.packed_mips {
      for i in 1..mip {
        address_offset += self.get_mip_extent(i, is_guest).all_blocks() * bytes_per_block;
      }
      *offset_x = 0;
      *offset_y = 0;
      return address_base + address_offset;
    }

    let width_pow2 = next_pow2(self.width);
    let height_pow2 = next_pow2(self.height);

    // Walk forward to find the address of the mip.
    let mut packed_mip_base = 1;

    for i in packed_mip_base..mip {
      let mip_width = 1.max(width_pow2 >> i);
      let mip_height = 1.max(height_pow2 >> i);

      if mip_width.min(mip_height) <= 16 {
        // We've reached the point where the mips are packed into a single tile.
        packed_mip_base = i;
        break;
      }
      address_offset += self.get_mip_extent(i, is_guest).all_blocks() * bytes_per_block;
    }

    // Now, check if the mip is packed at an offset.
    self.get_packed_tile_offset0(
      width_pow2 >> mip,
      height_pow2 >> mip,
      mip - packed_mip_base,
      offset_x,
      offset_y,
    );
    address_base + address_offset
  }

  fn get_packed_tile_offset(
    &self,
    packed_tile: u32,
    offset_x: &mut u32,
    offset_y: &mut u32,
  ) -> bool {
    let has_packed_mips = self.packed_mips;

    if !has_packed_mips {
      *offset_x = 0;
      *offset_y = 0;
      return false;
    }

    self.get_packed_tile_offset0(
      next_pow2(self.width),
      next_pow2(self.height),
      packed_tile,
      offset_x,
      offset_y,
    )
  }

  fn get_packed_tile_offset0(
    &self,
    width: u32,
    height: u32,
    packed_tile: u32,
    offset_x: &mut u32,
    offset_y: &mut u32,
  ) -> bool {
    // Tile size is 32x32, and once textures go <=16 they are packed into a
    // single tile together. The math here is insane. Most sourced
    // from graph paper and looking at dds dumps.
    //   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    // 0         +.4x4.+ +.....8x8.....+ +............16x16............+
    // 1         +.4x4.+ +.....8x8.....+ +............16x16............+
    // 2         +.4x4.+ +.....8x8.....+ +............16x16............+
    // 3         +.4x4.+ +.....8x8.....+ +............16x16............+
    // 4 x               +.....8x8.....+ +............16x16............+
    // 5                 +.....8x8.....+ +............16x16............+
    // 6                 +.....8x8.....+ +............16x16............+
    // 7                 +.....8x8.....+ +............16x16............+
    // 8 2x2                             +............16x16............+
    // 9 2x2                             +............16x16............+
    // 0                                 +............16x16............+
    // ...                                            .....
    // This only works for square textures, or textures that are some non-pot
    // <= square. As soon as the aspect ratio goes weird, the textures start to
    // stretch across tiles.
    //
    // The 2x2 and 1x1 squares are packed in their specific positions because
    // each square is the size of at least one block (which is 4x4 pixels max)
    //
    // if (tile_aligned(w) > tile_aligned(h)) {
    //   // wider than tall, so packed horizontally
    // } else if (tile_aligned(w) < tile_aligned(h)) {
    //   // taller than wide, so packed vertically
    // } else {
    //   square
    // }
    // It's important to use logical sizes here, as the input sizes will be
    // for the entire packed tile set, not the actual texture.
    // The minimum dimension is what matters most: if either width or height
    // is <= 16 this mode kicks in.

    let log2_width = log2_ceil(width);
    let log2_height = log2_ceil(height);
    if log2_width.min(log2_height) > 4 {
      // Too big, not packed.
      *offset_x = 0;
      *offset_y = 0;
      return false;
    }

    // Find the block offset of the mip.
    if packed_tile < 3 {
      if log2_width > log2_height {
        // Wider than tall. Laid out vertically.
        *offset_x = 0;
        *offset_y = 16 >> packed_tile;
      } else {
        // Taller than wide. Laid out horizontally.
        *offset_x = 16 >> packed_tile;
        *offset_y = 0;
      }
    } else {
      if log2_width > log2_height {
        // Wider than tall. Laid out vertically.
        *offset_x = 16 >> (packed_tile - 2);
        *offset_y = 0;
      } else {
        // Taller than wide. Laid out horizontally.
        *offset_x = 0;
        *offset_y = 16 >> (packed_tile - 2);
      }
    }

    *offset_x /= self.format.block_width;
    *offset_y /= self.format.block_height;

    true
  }

  fn get_mip_extent(&self, mip: u32, is_guest: bool) -> TextureExtent {
    if mip == 0 {
      return TextureExtent::calculate(
        self.format,
        self.pitch,
        self.height,
        self.depth,
        self.tiled,
        true,
      );
    }
    let mip_width;
    let mip_height;

    if is_guest {
      mip_width = next_pow2(self.width) >> mip;
      mip_height = next_pow2(self.height) >> mip;
    } else {
      mip_width = 1.max((self.width) >> mip);
      mip_height = 1.max((self.height) >> mip);
    }

    TextureExtent::calculate(
      self.format,
      mip_width,
      mip_height,
      self.depth,
      self.tiled,
      is_guest,
    )
  }
}

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/texture_info.cc
// see /licenses/xenia.txt
impl TextureExtent {
  fn all_blocks(&self) -> u32 {
    self.block_pitch_h * self.block_pitch_v * self.depth
  }

  fn calculate(
    format: &FormatData,
    pitch: u32,
    height: u32,
    depth: u32,
    is_tiled: bool,
    is_guest: bool,
  ) -> Self {
    let block_width = align(pitch, format.block_width) / format.block_width;
    let block_height = align(height, format.block_height) / format.block_height;

    let mut extent = TextureExtent {
      pitch,
      height,
      block_pitch_h: block_width,
      block_pitch_v: block_height,
      depth,
    };

    if is_guest {
      // Texture dimensions must be a multiple of tile
      // dimensions (32x32 blocks).
      extent.block_pitch_h = align(extent.block_pitch_h, 32);
      extent.block_pitch_v = align(extent.block_pitch_v, 32);

      extent.pitch = extent.block_pitch_h * format.block_width;
      extent.height = extent.block_pitch_v * format.block_height;

      let bytes_per_block = format.bytes_per_block;
      let mut byte_pitch = extent.block_pitch_h * bytes_per_block;

      if !is_tiled {
        // Each row must be a multiple of 256 bytes in linear textures.
        byte_pitch = align(byte_pitch, 256);
        extent.block_pitch_h = byte_pitch / bytes_per_block;
        extent.pitch = extent.block_pitch_h * format.block_width;
      }

      // Is depth special?
      // extent.depth = extent.depth;
    } else {
      extent.pitch = extent.block_pitch_h * format.block_width;
      extent.height = extent.block_pitch_v * format.block_height;
    }

    extent
  }
}
