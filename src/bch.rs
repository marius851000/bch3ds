use crate::deserialize::{read_vec_pointer, ReadVecError};
use crate::model::{Model, ModelError};
use crate::{bch_to_absolute, BCHRelocateError};
use crate::{BCHContentHeader, ReferenceDictError};
use crate::{BCHHeader, BCHHeaderError};
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};

//TODO: Error
#[derive(Debug)]
pub enum BCHError {
    FailedToSeekFileStart(io::Error),
    FailedToCopyWholeFileToRam(io::Error),
    BCHHeaderError(BCHHeaderError),
    BCHRelocateError(BCHRelocateError),
    FetchError(io::Error, &'static str),
    BCHContentHeaderError(ReferenceDictError),
    ModelReadError(ReadVecError<ModelError>),
}

#[derive(Debug)]
pub struct BCH {
    pub models: Vec<Model>,
}

impl BCH {
    pub fn read<F: Read + Seek>(file: &mut F) -> Result<BCH, BCHError> {
        file.seek(SeekFrom::Start(0))
            .map_err(BCHError::FailedToSeekFileStart)?;
        let header = BCHHeader::read(file).map_err(BCHError::BCHHeaderError)?;

        let mut file_content = Vec::new();
        file.seek(SeekFrom::Start(0))
            .map_err(BCHError::FailedToSeekFileStart)?;
        file.read_to_end(&mut file_content)
            .map_err(BCHError::FailedToCopyWholeFileToRam)?;
        bch_to_absolute(&header, &mut file_content).map_err(BCHError::BCHRelocateError)?;

        let mut file = Cursor::new(file_content);

        // read the content header
        file.seek(SeekFrom::Start(header.contents_address as u64))
            .map_err(|err| BCHError::FetchError(err, "content header"))?;

        let content_header =
            BCHContentHeader::read(&mut file).map_err(BCHError::BCHContentHeaderError)?;

        // read models
        file.seek(SeekFrom::Start(
            content_header.models.pointer_table_offset as u64,
        ))
        .map_err(|err| BCHError::FetchError(err, "model"))?;

        let models: Vec<Model> = read_vec_pointer(
            &mut file,
            |file| Model::read(file),
            content_header.models.pointer_table_entries as u64,
        )
        .map_err(BCHError::ModelReadError)?;

        Ok(BCH {
            models
        })
    }
}
