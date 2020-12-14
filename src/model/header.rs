use crate::deserialize::{
    read_matrix4x3_f32_le, read_referenced_null_terminated_ascii_string, read_u16_le, read_u32_le,
    read_u8,
};
use crate::{ReferenceDict, ReferenceDictError};
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum ModelHeaderError {
    IOError(io::Error, &'static str),
    ReferenceDictError(ReferenceDictError),
    InvalidNullMagic(u32),
    NullPointerToName,
}

impl From<ReferenceDictError> for ModelHeaderError {
    fn from(err: ReferenceDictError) -> ModelHeaderError {
        ModelHeaderError::ReferenceDictError(err)
    }
}
// shortcut function
fn ioe(err: io::Error, content: &'static str) -> ModelHeaderError {
    ModelHeaderError::IOError(err, content)
}

#[derive(Debug)]
pub struct ModelHeader {
    pub flags: u8,
    pub skeleton_scaling_type: u8,
    pub silhouette_material_entries: u16,
    pub world_transform: [[f32; 3]; 4],
    pub materials: ReferenceDict,
    pub vertices: ReferenceDict,
    pub skeletons: ReferenceDict,
    pub object_node_visibility_offset: u32,
    pub object_node_count: u32,
    pub model_name: String,
    pub object_node_name_entries: u32,
    pub object_node_name_offsets: u32,
    pub meta_data_pointer_offset: u32,
}

impl ModelHeader {
    pub fn read<F: Read + Seek>(file: &mut F) -> Result<ModelHeader, ModelHeaderError> {
        let flags = read_u8(file).map_err(|e| ioe(e, "flags"))?;

        let skeleton_scaling_type = read_u8(file).map_err(|e| ioe(e, "skeleton scaling type"))?;

        let silhouette_material_entries =
            read_u16_le(file).map_err(|e| ioe(e, "silhouette material entries"))?;

        let world_transform = read_matrix4x3_f32_le(file).map_err(|e| ioe(e, "world transform"))?;

        let materials = ReferenceDict::read(file, "materials")?;

        let vertices = ReferenceDict::read(file, "vertices")?;

        file.seek(SeekFrom::Current(0x24))
            .map_err(|e| ioe(e, "unknown data"))?;

        let skeletons = ReferenceDict::read(file, "skeleton")?;

        let object_node_visibility_offset =
            read_u32_le(file).map_err(|e| ioe(e, "object node visibility offset"))?;

        let object_node_count = read_u32_le(file).map_err(|e| ioe(e, "object node count"))?;

        let model_name = read_referenced_null_terminated_ascii_string(file)
            .map_err(|e| ioe(e, "model name"))?
            .ok_or(ModelHeaderError::NullPointerToName)?;

        let object_node_name_entries =
            read_u32_le(file).map_err(|e| ioe(e, "object node name entries"))?;

        let object_node_name_offsets =
            read_u32_le(file).map_err(|e| ioe(e, "object node name offsets"))?;

        let null_magic = read_u32_le(file).map_err(|e| ioe(e, "null magic"))?;
        if null_magic != 0 {
            return Err(ModelHeaderError::InvalidNullMagic(null_magic));
        };

        let meta_data_pointer_offset =
            read_u32_le(file).map_err(|e| ioe(e, "meta data pointer offset"))?;

        Ok(ModelHeader {
            flags,
            skeleton_scaling_type,
            silhouette_material_entries,
            world_transform,
            materials,
            vertices,
            skeletons,
            object_node_visibility_offset,
            object_node_count,
            model_name,
            object_node_name_entries,
            object_node_name_offsets,
            meta_data_pointer_offset,
        })
    }
}
