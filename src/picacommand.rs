#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PicaCommand {
    BlockEnd,
    VertexShaderFloatUniformConfig,
    VertexShaderFloatUniformData,
    FragmentShaderLookUpTableData,
    Unknown(u16),
}

impl PicaCommand {
    pub fn new_from_id(id: u16) -> PicaCommand {
        match id {
            0x23d => PicaCommand::BlockEnd,
            0x2c0 => PicaCommand::VertexShaderFloatUniformConfig,
            0x2c1 => PicaCommand::VertexShaderFloatUniformData,
            0x1c8 => PicaCommand::FragmentShaderLookUpTableData,

            unk => PicaCommand::Unknown(unk),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum IndexBufferFormat {
    U8,
    U16,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VSHAttribute {
    Position = 0,
    Normal = 1,
    Tangent = 2,
    Color = 3,
    TextureCoordinate0 = 4,
    TextureCoordinate1 = 5,
    TextureCoordinate2 = 6,
    BoneIndex = 7,
    BoneWeight = 8,
    UserAttribute0 = 9,
    UserAttribute1 = 0xa,
    UserAttribute2 = 0xb,
    UserAttribute3 = 0xc,
    UserAttribute4 = 0xd,
    UserAttribute5 = 0xe,
    UserAttribute6 = 0xf,
    UserAttribute7 = 0x10,
    UserAttribute8 = 0x11,
    UserAttribute9 = 0x12,
    UserAttribute10 = 0x13,
    UserAttribute11 = 0x14,
    Interleave = 0x15,
    Quantity = 0x16,
}

impl VSHAttribute {
    pub fn new(nb: u8) -> Option<VSHAttribute> {
        Some(match nb {
            0 => Self::Position,
            1 => Self::Normal,
            2 => Self::Tangent,
            3 => Self::Color,
            4 => Self::TextureCoordinate0,
            5 => Self::TextureCoordinate1,
            6 => Self::TextureCoordinate2,
            7 => Self::BoneIndex,
            8 => Self::BoneWeight,
            9 => Self::UserAttribute0,
            10 => Self::UserAttribute1,
            11 => Self::UserAttribute2,
            12 => Self::UserAttribute3,
            13 => Self::UserAttribute4,
            14 => Self::UserAttribute5,
            15 => Self::UserAttribute6,
            16 => Self::UserAttribute7,
            17 => Self::UserAttribute8,
            18 => Self::UserAttribute9,
            19 => Self::UserAttribute10,
            20 => Self::UserAttribute11,
            21 => Self::Interleave,
            22 => Self::Quantity,
            _ => return None
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeData {
    I8(i8),
    U8(u8),
    I16(i16),
    F32(f32),
    None,
}

impl AttributeData {
    pub fn to_f32(&self) -> f32 {
        match self {
            Self::I8(nb) => f32::from(*nb),
            Self::U8(nb) => f32::from(*nb),
            Self::I16(nb) => f32::from(*nb),
            Self::F32(nb) => *nb,
            Self::None => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeFormatType {
    SignedByte,
    UnsignedByte,
    SignedShort,
    Single,
}

impl AttributeFormatType {
    pub fn new(nb: u8) -> Option<AttributeFormatType> {
        Some(match nb {
            0 => Self::SignedByte,
            1 => Self::UnsignedByte,
            2 => Self::SignedShort,
            3 => Self::Single,
            _ => return None
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeFormat {
    pub r#type: AttributeFormatType,
    pub attribute_length: u32,
}
