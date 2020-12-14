use crate::BCHHeader;
use crate::{BCHSection, BCHSectionError};
use std::io;

//TODO: Error
#[derive(Debug)]
pub enum BCHRelocateError {
    IOError(io::Error),
    BCHSectionError(BCHSectionError),
    NotLongEnought,
}

impl From<io::Error> for BCHRelocateError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<BCHSectionError> for BCHRelocateError {
    fn from(err: BCHSectionError) -> Self {
        Self::BCHSectionError(err)
    }
}

pub fn bch_to_absolute(header: &BCHHeader, bytes: &mut Vec<u8>) -> Result<(), BCHRelocateError> {
    for count in 0..(header.relocation_length as u64 / 4) {
        let offset = count * 4;
        let pos = header.relocation_address as usize + offset as usize;
        let mut value_buffer = [0; 4];
        for counter in 0..4 {
            value_buffer[counter] = match bytes.get(pos + counter) {
                Some(value) => *value,
                None => return Err(BCHRelocateError::NotLongEnought),
            }
        }

        let value = u32::from_le_bytes(value_buffer);
        if cfg!(feature = "compare") {
            println!("found u32 {}", value);
        };

        let mut ptr_address = value & 0x1ffffff;

        let target = BCHSection::new(((value >> 25) & 0xf) as u8)?;
        let source = BCHSection::new(((value >> 29) & 0xf) as u8)?;

        if target != BCHSection::Strings {
            ptr_address <<= 2;
        };

        accumulate32(
            bytes,
            get_address(source, header) + ptr_address,
            get_address(target, header),
        )?;
    }
    Ok(())
}

fn get_address(section: BCHSection, header: &BCHHeader) -> u32 {
    (match section {
        BCHSection::Contents => header.contents_address,
        BCHSection::Strings => header.strings_address,
        BCHSection::Commands | BCHSection::CommandsSrc => header.commands_address,
        BCHSection::RawData
        | BCHSection::RawDataTexture
        | BCHSection::RawDataVertex
        | BCHSection::RawDataIndex8 => header.raw_data_address,
        BCHSection::RawDataIndex16 => header.raw_data_address | (1 << 31),
        BCHSection::RawExt
        | BCHSection::RawExtTexture
        | BCHSection::RawExtVertex
        | BCHSection::RawExtIndex8 => header.raw_ext_address,
        BCHSection::RawExtIndex16 => header.raw_ext_address | (1 << 31),
        BCHSection::BaseAddress => 0,
    }) as u32
}

fn accumulate32(bytes: &mut Vec<u8>, address: u32, mut value: u32) -> Result<(), BCHRelocateError> {
    //Peek32:
    let mut buffer = [0, 0, 0, 0];
    for counter in 0..4 {
        buffer[counter] = match bytes.get(address as usize + counter) {
            Some(value) => *value,
            None => return Err(BCHRelocateError::NotLongEnought),
        };
    }

    if cfg!(feature = "compare") {
        println!("found u32 {}", u32::from_le_bytes(buffer));
    }
    value += u32::from_le_bytes(buffer);

    for (counter, byte) in value.to_le_bytes().iter().cloned().enumerate() {
        bytes[address as usize + counter] = byte;
    }

    Ok(())
}
