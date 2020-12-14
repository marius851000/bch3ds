use std::io;
use std::io::Read;

#[derive(Debug)]
pub enum ReferenceDictError {
    IOError(io::Error, &'static str, &'static str), //category, part
}

#[derive(Debug, Clone, Copy)]
pub struct ReferenceDict {
    pub pointer_table_offset: u32,
    pub pointer_table_entries: u32,
    pub name_offset: u32,
}

fn read_u32_pair<F: Read>(
    file: &mut F,
    category: &'static str,
    part: &'static str,
) -> Result<u32, ReferenceDictError> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)
        .map_err(|err| ReferenceDictError::IOError(err, category, part))?;
    Ok(u32::from_le_bytes(buffer))
}

impl ReferenceDict {
    pub fn read<F: Read>(file: &mut F, category: &'static str) -> Result<Self, ReferenceDictError> {
        let pointer_table_offset = read_u32_pair(file, category, "pointer table offset")?;
        let pointer_table_entries = read_u32_pair(file, category, "pointer table entries")?;
        let name_offset = read_u32_pair(file, category, "name offset")?;

        Ok(Self {
            pointer_table_offset,
            pointer_table_entries,
            name_offset,
        })
    }
}

#[derive(Debug)]
pub struct BCHContentHeader {
    pub models: ReferenceDict,
    pub materials: ReferenceDict,
    pub shaders: ReferenceDict,
    pub textures: ReferenceDict,
    pub materials_lut: ReferenceDict,
    pub lights: ReferenceDict,
    pub cameras: ReferenceDict,
    pub fogs: ReferenceDict,
    pub skeletal_animations: ReferenceDict,
    pub material_animations: ReferenceDict,
    pub visibility_animations: ReferenceDict,
    pub light_animation: ReferenceDict,
    pub camera_animation: ReferenceDict,
    pub fog_animation: ReferenceDict,
    pub scene: ReferenceDict,
}

impl BCHContentHeader {
    pub fn read<F: Read>(file: &mut F) -> Result<Self, ReferenceDictError> {
        let models = ReferenceDict::read(file, "models")?;
        let materials = ReferenceDict::read(file, "materials")?;
        let shaders = ReferenceDict::read(file, "shaders")?;
        let textures = ReferenceDict::read(file, "textures")?;
        let materials_lut = ReferenceDict::read(file, "materials lut")?;
        let lights = ReferenceDict::read(file, "lights")?;
        let cameras = ReferenceDict::read(file, "cameras")?;
        let fogs = ReferenceDict::read(file, "fogs")?;
        let skeletal_animations = ReferenceDict::read(file, "skeletal animations")?;
        let material_animations = ReferenceDict::read(file, "material animations")?;
        let visibility_animations = ReferenceDict::read(file, "visibility animations")?;
        let light_animation = ReferenceDict::read(file, "light animation")?;
        let camera_animation = ReferenceDict::read(file, "camera animation")?;
        let fog_animation = ReferenceDict::read(file, "fog animation")?;
        let scene = ReferenceDict::read(file, "scene")?;

        Ok(BCHContentHeader {
            models,
            materials,
            shaders,
            textures,
            materials_lut,
            lights,
            cameras,
            fogs,
            skeletal_animations,
            material_animations,
            visibility_animations,
            light_animation,
            camera_animation,
            fog_animation,
            scene,
        })
    }
}
