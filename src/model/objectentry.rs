use crate::deserialize::{read_f32_le, read_u16_le, read_u32_le, read_vector};
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum ObjectEntryError {
    IOError(io::Error, &'static str),
    NullMagicError(u32),
}

fn ioe(err: io::Error, content: &'static str) -> ObjectEntryError {
    ObjectEntryError::IOError(err, content)
}

#[derive(Debug)]
pub struct ObjectEntry {
    pub material_id: u16,
    pub flags: u16,
    pub node_id: u16,
    pub render_priority: u16,
    pub vsh_attributes_buffer_commands_word_count: u32,
    pub vsh_attributes_buffer_commands_offset: u32,
    pub faces_header_offset: u32,
    pub faces_header_entries: u32,
    pub vsh_extra_attributes_buffer_commands_offset: u32,
    pub vsh_extra_attributes_buffer_commands_word_counts: u32,
    pub center_vector: [f32; 3],
    pub flags_offset: u32,
    pub bounding_box_offset: u32,
}

impl ObjectEntry {
    pub fn read<F: Read + Seek>(file: &mut F) -> Result<Self, ObjectEntryError> {
        let material_id = read_u16_le(file).map_err(|e| ioe(e, "material id"))?;

        let flags = read_u16_le(file).map_err(|e| ioe(e, "flags"))?;

        let node_id = read_u16_le(file).map_err(|e| ioe(e, "node id"))?;

        let render_priority = read_u16_le(file).map_err(|e| ioe(e, "render priority"))?;

        let vsh_attributes_buffer_commands_offset =
            read_u32_le(file).map_err(|e| ioe(e, "vsh attributes buffer commands offset"))?;

        let vsh_attributes_buffer_commands_word_count =
            read_u32_le(file).map_err(|e| ioe(e, "vsh attributes buffer commands word count"))?;

        let faces_header_offset = read_u32_le(file).map_err(|e| ioe(e, "faces header offset"))?;

        let faces_header_entries = read_u32_le(file).map_err(|e| ioe(e, "faces header entries"))?;

        let vsh_extra_attributes_buffer_commands_offset =
            read_u32_le(file).map_err(|e| ioe(e, "vsh extra attributes buffer commands offset"))?;

        let vsh_extra_attributes_buffer_commands_word_counts = read_u32_le(file)
            .map_err(|e| ioe(e, "vsh extra attributes buffer commands word counts"))?;

        let mut center_vector = [0.0; 3];

        read_vector(file, |f| read_f32_le(f), &mut center_vector)
            .map_err(|e| ioe(e, "center vector"))?;

        let flags_offset = read_u32_le(file).map_err(|e| ioe(e, "flags offset"))?;

        let null_magic = read_u32_le(file).map_err(|e| ioe(e, "null magic"))?;

        if null_magic != 0 {
            return Err(ObjectEntryError::NullMagicError(null_magic));
        };

        let bounding_box_offset = read_u32_le(file).map_err(|e| ioe(e, "bounding box offset"))?;

        Ok(ObjectEntry {
            material_id,
            flags,
            node_id,
            render_priority,
            vsh_attributes_buffer_commands_offset,
            vsh_attributes_buffer_commands_word_count,
            faces_header_offset,
            faces_header_entries,
            vsh_extra_attributes_buffer_commands_offset,
            vsh_extra_attributes_buffer_commands_word_counts,
            center_vector,
            flags_offset,
            bounding_box_offset,
        })
    }
}
