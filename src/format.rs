use dds::D3DFormat;

#[derive(Debug, Copy, Clone)]
pub enum Format {
  Dxt1,
  Dxt3,
  Dxt5,
  RGBA8,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatData {
  pub(crate) block_width: u32,
  pub(crate) block_height: u32,
  pub(crate) bytes_per_block: u32,
}

// https://github.com/gildor2/UEViewer/blob/86bd93f1dffba6c98a3acc8f08a59e662e32ccd6/Unreal/UnrealMaterial/UnTexture.cpp#L52

pub(crate) const DXT1: FormatData = FormatData {
  block_width: 4,
  block_height: 4,
  bytes_per_block: 8,
};

pub(crate) const DXT3: FormatData = FormatData {
  block_width: 4,
  block_height: 4,
  bytes_per_block: 16,
};

pub(crate) const DXT5: FormatData = FormatData {
  block_width: 4,
  block_height: 4,
  bytes_per_block: 16,
};

pub(crate) const RGBA8: FormatData = FormatData {
  block_width: 1,
  block_height: 1,
  bytes_per_block: 4,
};

pub(crate) fn get_dds_format(format: &Format) -> D3DFormat {
  match format {
    Format::Dxt1 => D3DFormat::DXT1,
    Format::Dxt3 => D3DFormat::DXT3,
    Format::Dxt5 => D3DFormat::DXT5,
    Format::RGBA8 => D3DFormat::A8R8G8B8,
  }
}

pub(crate) fn get_format_data(format: &Format) -> FormatData {
  match format {
    Format::Dxt1 => DXT1,
    Format::Dxt3 => DXT3,
    Format::Dxt5 => DXT5,
    Format::RGBA8 => RGBA8,
  }
}
