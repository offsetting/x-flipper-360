use binrw::{BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::*, BitfieldSpecifier};

// https://github.com/xenia-project/xenia/blob/master/src/xenia/gpu/xenos.h
// see /licenses/xenia.txt

#[bitfield]
#[derive(Clone, Debug)]
pub struct TextureSize1D {
  pub width: B24,
  #[skip]
  _padding0: B8,
}

#[bitfield]
#[derive(Clone, Debug)]
pub struct TextureSize2D {
  pub width: B13,
  pub height: B13,
  pub stack_depth: B6,
}

#[bitfield]
#[derive(Clone, Debug)]
pub struct TextureSize3D {
  pub width: B11,
  pub height: B11,
  pub depth: B10,
}

#[bitfield]
#[derive(Clone, Debug)]
pub struct TextureSizeStack {
  pub width: B13,
  pub height: B13,
  pub depth: B6,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 6]
pub enum TextureFormat {
  _1Reverse = 0,
  _1 = 1,
  _8 = 2,
  _1_5_5_5 = 3,
  _5_6_5 = 4,
  _6_5_5 = 5,
  _8_8_8_8 = 6,
  _2_10_10_10 = 7,
  _8A = 8,
  _8B = 9,
  _8_8 = 10,
  CrY1CbY0Rep = 11,
  Y1CrY0CbRep = 12,
  _16_16Edram = 13,
  _8_8_8_8A = 14,
  _4_4_4_4 = 15,
  _10_11_11 = 16,
  _11_11_10 = 17,
  Dxt1 = 18,
  Dxt2_3 = 19,
  Dxt4_5 = 20,
  _16_16_16_16Edram = 21,
  _24_8 = 22,
  _24_8Float = 23,
  _16 = 24,
  _16_16 = 25,
  _16_16_16_16 = 26,
  _16Expand = 27,
  _16_16Expand = 28,
  _16_16_16_16Expand = 29,
  _16Float = 30,
  _16_16Float = 31,
  _16_16_16_16Float = 32,
  _32 = 33,
  _32_32 = 34,
  _32_32_32_32 = 35,
  _32Float = 36,
  _32_32Float = 37,
  _32_32_32_32Float = 38,
  _32As8 = 39,
  _32As8_8 = 40,
  _16Mpeg = 41,
  _16_16Mpeg = 42,
  _8Interlaced = 43,
  _32As8Interlaced = 44,
  _32As8_8Interlaced = 45,
  _16Interlaced = 46,
  _16MpegInterlaced = 47,
  _16_16MpegInterlaced = 48,
  Dxn = 49,
  _8_8_8_8As16_16_16_16 = 50,
  _Dxt1As16_16_16_16 = 51,
  _Dxt2_3As16_16_16_16 = 52,
  _Dxt4_5As16_16_16_16 = 53,
  _2_10_10_10As16_16_16_16 = 54,
  _10_11_11As16_16_16_16 = 55,
  _11_11_10As16_16_16_16 = 56,
  _32_32_32Float = 57,
  Dxt3A = 58,
  Dxt5A = 59,
  Ctx1 = 60,
  Dxt3AAs1_1_1_1 = 61,
  _8_8_8_8GammaEdram = 62,
  _2_10_10_10FloatEdram = 63,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum NumFormat {
  Fraction = 0,
  Integer = 1,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum TextureKind {
  InvalidTexture = 0,
  InvalidVertex = 1,
  Texture = 2,
  Vertex = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum TextureSign {
  Unsigned = 0,
  Signed = 1,
  UnsignedBiased = 2,
  Gamma = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
pub enum ClampMode {
  Repeat = 0,
  MirroredRepeat = 1,
  ClampToEdge = 2,
  MirrorClampToEdge = 3,
  ClampToHalfway = 4,
  MirrorClampToHalfway = 5,
  ClampToBorder = 6,
  MirrorClampToBorder = 7,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum Endian {
  None = 0,
  _8in16 = 1,
  _8in32 = 2,
  _16in32 = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum RequestSize {
  _256Bit = 0,
  _512Bit = 1,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
pub enum Swizzle {
  X = 0,
  Y = 1,
  Z = 2,
  W = 3,
  Zero = 4,
  One = 5,
  Keep = 7,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum ClampPolicy {
  D3D = 0,
  OpenGL = 1,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum Dimension {
  OneD = 0,
  TwoDOrStacked = 1,
  ThreeD = 2,
  CubeMap = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum BorderColor {
  AgbrBlack = 0,
  AgbrWhite = 1,
  AcbycrBlack = 2,
  AcbcryBlack = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum TriClamp {
  Normal = 0,
  OneSixth = 1,
  OneFourth = 2,
  ThreeEighths = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum MinMagFilter {
  Point = 0,
  Linear = 1,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum MipFilter {
  Point = 0,
  Linear = 1,
  Basemap = 2,
  Keep = 3,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
pub enum AnisoFilter {
  Disabled = 0,
  Max1To1 = 1,
  Max2To1 = 2,
  Max4To1 = 3,
  Max8To1 = 4,
  Max16To1 = 5,
  UseFetchConst = 7,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum SignedRepeatingFractionMode {
  ZeroClampMinusOne = 0,
  NoZero = 1,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
pub enum ArbitraryFilter {
  _2x4Sym = 0,
  _2x4Asym = 1,
  _4x2Sym = 2,
  _4x2Asym = 3,
  _4x4Sym = 4,
  _4x4Asym = 5,
  UseFetchConst = 7,
}

#[bitfield]
#[derive(Debug)]
pub struct TextureMetadata {
  pub kind: TextureKind,
  pub sign_x: TextureSign,
  pub sign_y: TextureSign,
  pub sign_z: TextureSign,
  pub sign_w: TextureSign,
  pub clamp_x: ClampMode,
  pub clamp_y: ClampMode,
  pub clamp_z: ClampMode,
  pub signed_repeating_fraction: SignedRepeatingFractionMode,
  pub dim_tbd: B2,
  // u32
  pub pitch: B9,
  // u32
  pub tiled: bool,
  pub format: TextureFormat,
  pub endianness: Endian,
  pub request_size: RequestSize,
  pub stacked: bool,
  pub clamp_policy: ClampPolicy,
  pub base_address: B20,
  // u32
  pub texture_size: u32,
  pub num_format: NumFormat,
  pub swizzle_x: Swizzle,
  pub swizzle_y: Swizzle,
  pub swizzle_z: Swizzle,
  pub swizzle_w: Swizzle,
  pub exp_adjust: B6,
  // i32
  pub mag_filter: MipFilter,
  pub min_filter: MipFilter,
  pub mip_filter: MipFilter,
  pub aniso_filter: AnisoFilter,
  pub arbitrary_filter: ArbitraryFilter,
  pub border_size: B1,
  // u32
  pub vol_mag_filter: MinMagFilter,
  pub vol_min_filter: MinMagFilter,
  pub min_mip_level: B4,
  // u32
  pub max_mip_level: B4,
  // u32
  pub mag_aniso_walk: B1,
  // u32
  pub min_aniso_walk: B1,
  // u32
  pub lodbias: B10,
  // i32
  pub grad_exp_adjust_h: B5,
  // i32
  pub grad_exp_adjust_v: B5,
  // i32
  pub border_color: BorderColor,
  pub force_bc_w_to_max: bool,
  pub tri_clamp: TriClamp,
  pub aniso_bias: B4,
  // i32
  pub dimension: Dimension,
  pub packed_mips: bool,
  pub mip_address: B20, // u32
}

#[derive(BinRead, BinWrite, Debug)]
pub struct TextureHeader {
  pub common: u32,
  pub reference_count: u32,
  pub fence: u32,
  pub read_fence: u32,
  pub identifier: u32,
  pub base_flush: u32,
  pub mip_flush: u32,
  metadata: [u8; 24],
}

impl TextureHeader {
  pub fn metadata(&self) -> TextureMetadata {
    let mut metadata = self.metadata;

    // reversing ever u32 independently
    for chunk in (0..metadata.len()).step_by(4) {
      let dword = &mut metadata[chunk..chunk + 4];
      dword.reverse();
    }

    TextureMetadata::from_bytes(metadata)
  }
}

// impl Serialize for TextureMetadata {
//   fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//     let mut state = serializer.serialize_struct("GpuTextureFetchConstant", 46)?;
//
//     state.serialize_field("kind", &self.kind())?;
//     state.serialize_field("sign_x", &self.sign_x())?;
//     state.serialize_field("sign_y", &self.sign_y())?;
//     state.serialize_field("sign_z", &self.sign_z())?;
//     state.serialize_field("sign_w", &self.sign_w())?;
//     state.serialize_field("clamp_x", &self.clamp_x())?;
//     state.serialize_field("clamp_y", &self.clamp_y())?;
//     state.serialize_field("clamp_z", &self.clamp_z())?;
//     state.serialize_field(
//       "signed_repeating_fraction",
//       &self.signed_repeating_fraction(),
//     )?;
//     state.serialize_field("dim_tbd", &self.dim_tbd())?;
//     state.serialize_field("pitch", &self.pitch())?;
//     state.serialize_field("tiled", &self.tiled())?;
//     state.serialize_field("format", &self.format())?;
//     state.serialize_field("endianness", &self.endianness())?;
//     state.serialize_field("request_size", &self.request_size())?;
//     state.serialize_field("stacked", &self.stacked())?;
//     state.serialize_field("clamp_policy", &self.clamp_policy())?;
//     state.serialize_field("base_address", &self.base_address())?;
//
//     let mut texture_size: [u8; 4] = self.texture_size().to_le_bytes();
//
//     match &self.dimension() {
//       Dimension::OneD => {
//         state.serialize_field("texture_size", &TextureSize1D::from_bytes(texture_size))?
//       }
//       Dimension::TwoDOrStacked => {
//         state.serialize_field("texture_size", &TextureSize2D::from_bytes(texture_size))?
//       }
//       Dimension::ThreeD => {
//         state.serialize_field("texture_size", &TextureSize3D::from_bytes(texture_size))?
//       }
//       Dimension::CubeMap => {
//         state.serialize_field("texture_size", &TextureSizeStack::from_bytes(texture_size))?
//       }
//     };
//
//     state.serialize_field("num_format", &self.num_format())?;
//     state.serialize_field("swizzle_x", &self.swizzle_x())?;
//     state.serialize_field("swizzle_y", &self.swizzle_y())?;
//     state.serialize_field("swizzle_z", &self.swizzle_z())?;
//     state.serialize_field("swizzle_w", &self.swizzle_w())?;
//     state.serialize_field("exp_adjust", &self.exp_adjust())?;
//     state.serialize_field("mag_filter", &self.mag_filter())?;
//     state.serialize_field("min_filter", &self.min_filter())?;
//     state.serialize_field("mip_filter", &self.mip_filter())?;
//     state.serialize_field("aniso_filter", &self.aniso_filter())?;
//     state.serialize_field("arbitrary_filter", &self.arbitrary_filter())?;
//     state.serialize_field("border_size", &self.border_size())?;
//     state.serialize_field("vol_mag_filter", &self.vol_mag_filter())?;
//     state.serialize_field("vol_min_filter", &self.vol_min_filter())?;
//     state.serialize_field("min_mip_level", &self.min_mip_level())?;
//     state.serialize_field("max_mip_level", &self.max_mip_level())?;
//     state.serialize_field("mag_aniso_walk", &self.mag_aniso_walk())?;
//     state.serialize_field("min_aniso_walk", &self.min_aniso_walk())?;
//     state.serialize_field("lodbias", &self.lodbias())?;
//     state.serialize_field("grad_exp_adjust_h", &self.grad_exp_adjust_h())?;
//     state.serialize_field("grad_exp_adjust_v", &self.grad_exp_adjust_v())?;
//     state.serialize_field("border_color", &self.border_color())?;
//     state.serialize_field("force_bc_w_to_max", &self.force_bc_w_to_max())?;
//     state.serialize_field("tri_clamp", &self.tri_clamp())?;
//     state.serialize_field("aniso_bias", &self.aniso_bias())?;
//     state.serialize_field("dimension", &self.dimension())?;
//     state.serialize_field("packed_mips", &self.packed_mips())?;
//     state.serialize_field("mip_address", &self.mip_address())?;
//
//     state.end()
//   }
// }
//
// impl Serialize for TextureSize1D {
//   fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//     let mut state = serializer.serialize_struct("GpuTextureSize1D", 1)?;
//
//     state.serialize_field("width", &self.width())?;
//
//     state.end()
//   }
// }
//
// impl Serialize for TextureSize2D {
//   fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//     let mut state = serializer.serialize_struct("GpuTextureSize2D", 1)?;
//
//     state.serialize_field("width", &self.width())?;
//     state.serialize_field("height", &self.height())?;
//     state.serialize_field("stack_depth", &self.stack_depth())?;
//
//     state.end()
//   }
// }
//
// impl Serialize for TextureSize3D {
//   fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//     let mut state = serializer.serialize_struct("GpuTextureSize3D", 1)?;
//
//     state.serialize_field("width", &self.width())?;
//     state.serialize_field("height", &self.height())?;
//     state.serialize_field("depth", &self.depth())?;
//
//     state.end()
//   }
// }
//
// impl Serialize for TextureSizeStack {
//   fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//     let mut state = serializer.serialize_struct("GpuTextureSizeStack", 1)?;
//
//     state.serialize_field("width", &self.width())?;
//     state.serialize_field("height", &self.height())?;
//     state.serialize_field("depth", &self.depth())?;
//
//     state.end()
//   }
// }
