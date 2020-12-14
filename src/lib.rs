#![allow(dead_code)]

#[macro_use]
extern crate log;

mod bch;
pub use bch::{BCHError, BCH};

mod bchheader;
pub use bchheader::{BCHHeader, BCHHeaderError};

mod bchsections;
pub use bchsections::{BCHSection, BCHSectionError};

mod bchrelocator;
pub use bchrelocator::{bch_to_absolute, BCHRelocateError};

mod bchcontentheader;
pub use bchcontentheader::{BCHContentHeader, ReferenceDict, ReferenceDictError};

pub mod model;

mod picacommandreader;
pub use picacommandreader::{PICACommandReader, PICACommandReaderError};

mod picacommand;
pub use picacommand::{IndexBufferFormat, PicaCommand, VSHAttribute, AttributeFormat, AttributeFormatType, AttributeData};

mod skinningmode;
pub use skinningmode::SkinningMode;

mod math;

mod export_obj;
pub use export_obj::bch_to_obj;

mod deserialize;
