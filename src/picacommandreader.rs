use crate::deserialize::read_u32_le;
use crate::{PicaCommand, IndexBufferFormat, VSHAttribute, AttributeFormat, AttributeFormatType};
use std::fmt;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum PICACommandReaderError {
    IOError(io::Error, &'static str),
    UndefinedCurrentUniform,
}

pub struct PICACommandReader {
    pub commands: [u32; 0x10000],
    pub lookup_table: [f32; 256],
    pub float_uniform: Vec<Vec<f32>>,
}

impl fmt::Debug for PICACommandReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PICACommandReader").finish()
    }
}

impl PICACommandReader {
    pub fn read<F: Read + Seek>(
        file: &mut F,
        word_count: u64,
    ) -> Result<PICACommandReader, PICACommandReaderError> {
        let mut commands = [0; 0x10000];
        let mut word_read: u64 = 0;
        let mut current_uniform = None;
        let mut uniform: Vec<f32> = Vec::new();
        let mut lookup_table = [0.0; 256];
        let mut lut_index = 0;
        let mut float_uniform = vec![Vec::new(); 96];

        while word_read < word_count {
            let parameter =
                read_u32_le(file).map_err(|e| PICACommandReaderError::IOError(e, "parameter"))?;
            let header =
                read_u32_le(file).map_err(|e| PICACommandReaderError::IOError(e, "header"))?;
            word_read += 2;

            let mut id = (header & 0xffff) as u16;
            let mask = (header >> 16) & 0xf;
            let extra_parameter = (header >> 20) & 0x7ff;
            let consecutive_writing = (header & 0x80000000) > 0;

            commands[id as usize] =
                (commands[id as usize] & (!mask & 0xf)) | (parameter & (0xfffffff0 | mask));
            let command = PicaCommand::new_from_id(id);
            match command {
                PicaCommand::BlockEnd => break,
                PicaCommand::VertexShaderFloatUniformConfig => {
                    current_uniform = Some(parameter & 0x7fffffff)
                }
                PicaCommand::VertexShaderFloatUniformData => {
                    uniform.push(f32::from_le_bytes(commands[id as usize].to_le_bytes()))
                }
                PicaCommand::FragmentShaderLookUpTableData => {
                    //lookup_table[lut_index++] = commands[id as usize]; //TODO: lut_index++
                    todo!("fragment shader");
                }
                _ => (),
            }

            for i in 0..extra_parameter {
                if consecutive_writing {
                    id += 1
                };
                commands[id as usize] = (commands[id as usize] & (!mask & 0xf))
                    | (read_u32_le(file)
                        .map_err(|e| PICACommandReaderError::IOError(e, "some strange stuff"))?
                        & (0xfffffff0 | mask));
                word_read += 1;

                if (id > 0x2c0) && (id < (0x2c1 + 8)) {
                    uniform.push(f32::from_le_bytes(commands[id as usize].to_le_bytes()))
                } else if PicaCommand::new_from_id(id) == PicaCommand::FragmentShaderLookUpTableData
                {
                    //lookUpTable[lutIndex++] = commands[id]
                    todo!("look up fragment 2")
                };
            }

            if !uniform.is_empty() {
                match current_uniform {
                    None => return Err(PICACommandReaderError::UndefinedCurrentUniform),
                    Some(current_uniform) => {
                        let current_uniform = current_uniform as usize;
                        float_uniform[current_uniform].append(&mut uniform);
                    }
                }
            };

            lut_index = 0;

            while (file
                .seek(SeekFrom::Current(0))
                .map_err(|e| PICACommandReaderError::IOError(e, "telling position for padding"))?
                & 7)
                != 0
            {
                read_u32_le(file).map_err(|e| PICACommandReaderError::IOError(e, "padding"))?;
            }
        }

        Ok(PICACommandReader {
            commands,
            lookup_table,
            float_uniform,
        })
    }

    pub fn get_index_buffer_address(&self) -> u32{
        self.commands[0x227] & 0x7fffffff
    }

    pub fn get_index_buffer_format(&self) -> IndexBufferFormat {
        if self.commands[0x227] >> 31 == 0 {
            IndexBufferFormat::U8
        } else {
            IndexBufferFormat::U16
        }
    }

    pub fn get_index_buffer_total_vertices(&self) -> u32 {
        self.commands[0x228]
    }

    pub fn get_vsh_attributes_buffer_offset(&self, nb: usize) -> u32 {
        self.commands[0x203 + nb * 3]
    }

    pub fn get_vsh_attributes_buffer_stride(&self, nb: usize) -> u8 {
        (self.commands[0x205 + nb * 3] >> 16 & 0xFF) as u8
    }

    pub fn get_vsh_total_attributes(&self, nb: usize) -> u32 {
        self.commands[0x205 + nb *3] >> 28
    }

    pub fn get_vsh_attributes_buffer_permutation_none(&self) -> Vec<VSHAttribute> {
        let mut permutation: u64 = self.commands[0x2bb] as u64;
        permutation |= (self.commands[0x2bc] as u64) << 32;

        let mut attributes = Vec::new();
        for attribute in 0..16 { //TODO: 16 is 23 in the original
            attributes.push(VSHAttribute::new(((permutation >> (attribute * 4)) & 0xf ) as u8).unwrap());
        };

        attributes
    }

    pub fn get_vsh_attributes_buffer_permutation(&self, nb: usize) -> Vec<u8> {
        let mut permutation: u64 = self.commands[0x204 + nb * 3] as u64;
        permutation |= ((self.commands[0x205 + nb * 3] & 0xffff) as u64) << 32;

        let mut attributes = Vec::new();
        for attribute in 0..16 { //TODO: 16 is 23 in the original
            attributes.push(((permutation >> (attribute * 4)) & 0xf ) as u8);
        };

        attributes
    }

    pub fn get_vsh_attributes_buffer_format(&self) -> Vec<AttributeFormat>{
        let mut permutation: u64 = self.commands[0x201] as u64;
        permutation |= (self.commands[0x202] as u64) << 32;

        let mut formats = Vec::new();
        for attribute in 0..16 {  //TODO: 16 is 23 in the original
            let value = ((permutation >> (attribute * 4)) & 0xf ) as u8;
            formats.push(AttributeFormat {
                r#type: AttributeFormatType::new(value & 0b11).unwrap(),
                attribute_length: (value >> 2) as u32,
            })
        };

        formats


    }
}
