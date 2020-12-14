//TODO: Error
#[derive(Debug)]
pub enum BCHSectionError {
    InvalidCast(u8),
}

#[derive(Debug, PartialEq)]
pub enum BCHSection {
    Contents,
    Strings,
    Commands,
    CommandsSrc,
    RawData,
    RawDataTexture,
    RawDataVertex,
    RawDataIndex16,
    RawDataIndex8,
    RawExt,
    RawExtTexture,
    RawExtVertex,
    RawExtIndex16,
    RawExtIndex8,
    BaseAddress,
}

impl BCHSection {
    pub fn new(cast: u8) -> Result<BCHSection, BCHSectionError> {
        Ok(match cast {
            0 => Self::Contents,
            1 => Self::Strings,
            2 => Self::Commands,
            3 => Self::CommandsSrc,
            4 => Self::RawData,
            5 => Self::RawDataTexture,
            6 => Self::RawDataVertex,
            7 => Self::RawDataIndex16,
            8 => Self::RawDataIndex8,
            9 => Self::RawExt,
            10 => Self::RawExtTexture,
            11 => Self::RawExtVertex,
            12 => Self::RawExtIndex16,
            13 => Self::RawExtIndex8,
            14 => Self::BaseAddress,
            unk => return Err(BCHSectionError::InvalidCast(unk)),
        })
    }
}
