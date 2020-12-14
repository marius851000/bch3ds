use crate::deserialize::{
    read_referenced_null_terminated_ascii_string, read_vec_inline, ReadVecError,
};
use crate::model::{ModelHeader, ModelHeaderError};
use crate::model::{Object, ObjectError};
use crate::model::{ObjectEntry, ObjectEntryError};
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum ModelError {
    ModelHeaderError(ModelHeaderError),
    SeekError(io::Error, &'static str),
    ReadStringError(io::Error, &'static str),
    NullReference(&'static str),
    ReadObjectEntryError(ReadVecError<ObjectEntryError>),
    ObjectError(ObjectError),
}

#[derive(Debug)]
pub struct Model {
    //name: String,
    //layer_id: u32,
    pub mesh: Vec<Object>,
    //skeleton: Vec<Bone>,
    //material: Vec<Material>,
    //metadata: Vec<MetaData>,
    //transform:
    //min_vector
    //max_vector
    //vertices_count: i32,
}

impl Model {
    pub fn read<F: Read + Seek>(file: &mut F) -> Result<Model, ModelError> {
        let header = ModelHeader::read(file).map_err(ModelError::ModelHeaderError)?;

        file.seek(SeekFrom::Start(header.object_node_name_offsets as u64))
            .map_err(|e| ModelError::SeekError(e, "object names"))?;
        let mut object_name = Vec::new();
        file.seek(SeekFrom::Current(12))
            .map_err(|e| ModelError::SeekError(e, "useless data for us n°1"))?;

        for _ in 0..header.object_node_name_entries {
            file.seek(SeekFrom::Current(8))
                .map_err(|e| ModelError::SeekError(e, "useless data for us n°2"))?;
            object_name.push(
                read_referenced_null_terminated_ascii_string(file)
                    .map_err(|e| ModelError::ReadStringError(e, "object name"))?
                    .ok_or(ModelError::NullReference("object name"))?,
            );
        }

        // vertices header
        file.seek(SeekFrom::Start(header.vertices.pointer_table_offset as u64))
            .map_err(|e| ModelError::SeekError(e, "vertices header"))?;

        let objects_entry = read_vec_inline(
            file,
            |f| ObjectEntry::read(f),
            header.vertices.pointer_table_entries as u64,
        )
        .map_err(|e| ModelError::ReadObjectEntryError(e))?;

        // vertices
        let mut mesh = Vec::new();
        for obj in objects_entry.iter() {
            mesh.push(
                Object::read(file, obj, &object_name).map_err(|e| ModelError::ObjectError(e))?,
            );
        };

        if header.meta_data_pointer_offset != 0 {
            file.seek(SeekFrom::Start(header.meta_data_pointer_offset as u64)).map_err(|e| ModelError::SeekError(e, "model meta data"))?;
            debug!("TODO: in model.rs: read metadata");
        };

        debug!("TODO: in model.rs: something related to skeleton");

        Ok(Model {
            //name,
            //layer_id,
            mesh,
        })
    }
}
