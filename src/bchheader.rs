use crate::deserialize::{
    pad_file, read_i32_le, read_null_terminated_ascii_string, read_u16_le, read_u8,
};
use std::io;
use std::io::{Read, Seek};

//TODO: Error
#[derive(Debug)]
pub enum BCHHeaderError {
    IOError(io::Error),
    InvalidMagic(String),
}

impl From<io::Error> for BCHHeaderError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

#[derive(Debug)]
pub struct BCHHeader {
    pub backward_compatibility: u8,
    //Version
    pub forward_compatibility: u8,
    pub converter_version: u16,

    pub contents_address: i32,
    pub strings_address: i32,
    pub commands_address: i32,
    pub raw_data_address: i32,
    pub raw_ext_address: i32,
    pub relocation_address: i32,

    pub contents_length: i32,
    pub strings_length: i32,
    pub commands_length: i32,
    pub raw_data_length: i32,
    pub raw_ext_length: i32,
    pub relocation_length: i32,
    pub un_init_data_length: i32,
    pub un_init_commands_length: i32,

    pub flags: u8,
    pub address_count: u16,
}

impl BCHHeader {
    pub fn read<T: Read + Seek>(file: &mut T) -> Result<BCHHeader, BCHHeaderError> {
        if cfg!(feature = "compare") {
            println!("calling read on SPICA.Formats.CtrH3D.H3DHeader");
        };
        let magic = read_null_terminated_ascii_string(file)?;
        if magic != "BCH" {
            return Err(BCHHeaderError::InvalidMagic(magic));
        }

        let backward_compatibility = read_u8(file)?;
        let forward_compatibility = read_u8(file)?;

        let converter_version = read_u16_le(file)?;

        let contents_address = read_i32_le(file)?;
        let strings_address = read_i32_le(file)?;
        let commands_address = read_i32_le(file)?;
        let raw_data_address = read_i32_le(file)?;
        let raw_ext_address = read_i32_le(file)?;
        let relocation_address = read_i32_le(file)?;

        let contents_length = read_i32_le(file)?;
        let strings_length = read_i32_le(file)?;
        let commands_length = read_i32_le(file)?;
        let raw_data_length = read_i32_le(file)?;
        let raw_ext_length = read_i32_le(file)?;
        let relocation_length = read_i32_le(file)?;
        let un_init_data_length = read_i32_le(file)?;
        let un_init_commands_length = read_i32_le(file)?;

        let flags = read_u8(file)?;

        pad_file(file, 2)?;

        let address_count = read_u16_le(file)?;

        Ok(BCHHeader {
            backward_compatibility,
            forward_compatibility,
            converter_version,
            contents_address,
            strings_address,
            commands_address,
            raw_data_address,
            raw_ext_address,
            relocation_address,
            contents_length,
            strings_length,
            commands_length,
            raw_data_length,
            raw_ext_length,
            relocation_length,
            un_init_data_length,
            un_init_commands_length,
            flags,
            address_count,
        })
    }

    pub fn get_version(&self) -> u8 {
        self.backward_compatibility
    }
}
