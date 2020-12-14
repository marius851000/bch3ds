use crate::deserialize::{read_u16_le, read_u32_le, read_u8, read_i8, read_i16_le, read_f32_le};
use crate::model::ObjectEntry;
use crate::model::Vertex;
use crate::IndexBufferFormat;
use crate::SkinningMode;
use crate::VSHAttribute;
use crate::{AttributeFormat, AttributeFormatType, AttributeData};
use crate::{PICACommandReader, PICACommandReaderError};
use crate::math::vector3_tranform;
use std::cmp::max;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum ObjectError {
    IOError(io::Error, &'static str),
    PICACommandReaderError(PICACommandReaderError, &'static str),
    NotLongEnoughtError(&'static str),
    InvalidSkinning,
}

fn ioe(err: io::Error, content: &'static str) -> ObjectError {
    ObjectError::IOError(err, content)
}

fn get_vector<F: Read>(file: &mut F, format: AttributeFormat) -> Result<[AttributeData; 4], ObjectError> {
    let read_function: Box<dyn Fn(&mut F) -> Result<AttributeData, ObjectError>>;
    match format.r#type {
        AttributeFormatType::SignedByte => read_function = Box::new(|f| read_i8(f).map_err(|e| ioe(e, "i8 member of a vector")).map(|x| AttributeData::I8(x))),
        AttributeFormatType::UnsignedByte => read_function = Box::new(|f| read_u8(f).map_err(|e| ioe(e, "u8 member of a vector")).map(|x| AttributeData::U8(x))),
        AttributeFormatType::SignedShort => read_function = Box::new(|f| read_i16_le(f).map_err(|e| ioe(e, "i16 member of a vector")).map(|x| AttributeData::I16(x))),
        AttributeFormatType::Single => read_function = Box::new(|f| read_f32_le(f).map_err(|e| ioe(e, "f32 member of a vector")).map(|x| AttributeData::F32(x))),
    };

    let mut result = [AttributeData::None; 4];

    for loop_count in 0..(max(format.attribute_length, 3)+1) as usize {
        result[loop_count] = read_function(file)?;
    };

    Ok(result)
}

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    pub material_id: u16,
    pub render_priority: u16,
    pub name: String,
    pub is_visible: bool,
    //TODO: bounding box
    pub has_normal: bool,
    pub has_tangent: bool,
    pub has_color: bool,
    pub has_node: bool,
    pub has_weight: bool,
    pub tex_uv_count: i32,
}

impl Object {
    pub fn read<F: Read + Seek>(
        file: &mut F,
        obj: &ObjectEntry,
        object_name: &[String],
    ) -> Result<Object, ObjectError> {
        let mut has_node = false;
        let mut has_weight = false;
        let mut has_normal = false;
        let mut has_tangent = false;
        let mut has_color = false;
        let mut tex_uv_count = 0;

        let material_id = obj.material_id;
        let render_priority = obj.render_priority;
        let name: String = if obj.node_id >= object_name.len() as u16 {
            String::from("mesh") + &obj.node_id.to_string()
        } else {
            object_name[obj.node_id as usize].clone()
        };

        //TODO: object visibility
        let is_visible = true;

        //vertices
        file.seek(SeekFrom::Start(
            obj.vsh_attributes_buffer_commands_offset as u64,
        ))
        .map_err(|e| ioe(e, "bsh attribute buffer commands offset"))?;

        let vsh_commands =
            PICACommandReader::read(file, obj.vsh_attributes_buffer_commands_word_count as u64)
                .map_err(|e| ObjectError::PICACommandReaderError(e, "vsh commands"))?;

        let mut vsh_attributes_uniform_reg6 = vsh_commands.float_uniform[6].clone();
        let mut vsh_attributes_uniform_reg7 = vsh_commands.float_uniform[7].clone();

        let mut position_offset = [0.0; 4];
        for value in &mut position_offset {
            *value = vsh_attributes_uniform_reg6
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 6 for position offset",
                ))?;
        }

        let texture0_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for texture 0 scale",
                ))?;
        let texture1_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for texture 1 scale",
                ))?;
        let texture2_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for texture 2 scale",
                ))?;
        let bone_weight_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for bone weight scale",
                ))?;
        let position_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for position scale",
                ))?;
        let normal_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for normal scale",
                ))?;
        let tangent_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for tangent scale",
                ))?;
        let color_scale =
            vsh_attributes_uniform_reg7
                .pop()
                .ok_or(ObjectError::NotLongEnoughtError(
                    "vsh attributes uniform reg 7 for color scale",
                ))?;

        // faces
        let faces_count = obj.faces_header_entries;
        let has_faces = faces_count > 0;
        let mut faces_table_offset = 0;

        if !has_faces {
            todo!("TODO: has faces");
        };

        let mut vertices = Vec::new();
        
        for f in 0..faces_count {
            let mut skinning_mode: SkinningMode;
            let mut node_list: Vec<u16> = Vec::new();
            let idx_buffer_offset: u32;
            let idx_buffer_format: IndexBufferFormat;
            let idx_buffer_total_vertices: u32;

            if has_faces {
                let base_offset = obj.faces_header_offset + f * 0x34;
                file.seek(SeekFrom::Start(base_offset as u64))
                    .map_err(|e| ioe(e, "a faces data"))?;
                skinning_mode =
                    SkinningMode::new(read_u16_le(file).map_err(|e| ioe(e, "skinning mode"))?)
                        .ok_or(ObjectError::InvalidSkinning)?;
                let node_id_entries = read_u16_le(file).map_err(|e| ioe(e, "node id entries"))?;
                for _ in 0..node_id_entries {
                    node_list.push(read_u16_le(file).map_err(|e| ioe(e, "node entry"))?);
                }
                file.seek(SeekFrom::Start((base_offset + 0x2c) as u64))
                    .map_err(|e| ioe(e, "some data"))?;
                let face_header_offset =
                    read_u32_le(file).map_err(|e| ioe(e, "face header offset"))?;
                let face_header_word_count =
                    read_u32_le(file).map_err(|e| ioe(e, "face header word count"))?;
                file.seek(SeekFrom::Start(face_header_offset as u64))
                    .map_err(|e| ioe(e, "face header offset"))?;

                let idx_commands = PICACommandReader::read(file, face_header_word_count as u64)
                    .map_err(|e| ObjectError::PICACommandReaderError(e, "idx commands"))?;
                idx_buffer_offset = idx_commands.get_index_buffer_address();
                idx_buffer_format = idx_commands.get_index_buffer_format();
                idx_buffer_total_vertices = idx_commands.get_index_buffer_total_vertices();
            } else {
                todo!("TODO: has faces");
            }

            let vsh_attributes_buffer_offset = vsh_commands.get_vsh_attributes_buffer_offset(0);
            let vsh_attributes_buffer_stride = vsh_commands.get_vsh_attributes_buffer_stride(0);
            let vsh_total_attributes = vsh_commands.get_vsh_total_attributes(0);
            let vsh_main_attributes_buffer_permutation = vsh_commands.get_vsh_attributes_buffer_permutation_none();
            let vsh_attributes_buffer_permutation = vsh_commands.get_vsh_attributes_buffer_permutation(0);
            let vsh_attributes_buffer_format = vsh_commands.get_vsh_attributes_buffer_format();

            for attribute in 0..vsh_total_attributes {
                match vsh_main_attributes_buffer_permutation.get(
                    *vsh_attributes_buffer_permutation.get(
                        attribute as usize
                    ).unwrap() as usize //TODO: remove those unwrap
                ).unwrap() {
                    VSHAttribute::Normal => has_normal = true,
                    VSHAttribute::Tangent => has_tangent = true,
                    VSHAttribute::Color => has_color = true,
                    VSHAttribute::TextureCoordinate0 => {
                        tex_uv_count = max(tex_uv_count, 1);
                        break
                    },
                    VSHAttribute::TextureCoordinate1 => {
                        tex_uv_count = max(tex_uv_count, 2);
                        break
                    },
                    VSHAttribute::TextureCoordinate2 => {
                        tex_uv_count = max(tex_uv_count, 3);
                        break
                    },
                    _ => (),
                }
            };

            if !node_list.is_empty() {
                has_node = true;
                has_weight = true;
            };

            file.seek(SeekFrom::Start(idx_buffer_offset as u64)).map_err(|e| ioe(e, "idx buffer offset"))?;

            for face_index in 0..idx_buffer_total_vertices {
                let index: u16 = match idx_buffer_format {
                    IndexBufferFormat::U8 => read_u8(file).map(|v| v as u16),
                    IndexBufferFormat::U16 => read_u16_le(file),
                }.map_err(|e| ioe(e, "index"))?;

                let file_position = file.seek(SeekFrom::Current(0)).map_err(|e| ioe(e, "seeking to the current location"))?;
                let vertex_offset = vsh_attributes_buffer_offset as u64 + (index as u64 * vsh_attributes_buffer_stride as u64);
                file.seek(SeekFrom::Start(vertex_offset)).map_err(|e| ioe(e, "vertex offset"))?;

                let mut vertex = Vertex::default();
                vertex.diffuse_color = 0xffffffff;
                for attribute in 0..vsh_total_attributes {
                    let att = vsh_main_attributes_buffer_permutation.get(*vsh_attributes_buffer_permutation.get(attribute as usize).unwrap() as usize).unwrap(); //TODO
                    let mut format = vsh_attributes_buffer_format.get(*vsh_attributes_buffer_permutation.get(attribute as usize).unwrap() as usize).unwrap().clone(); //TODO
                    if *att == VSHAttribute::BoneWeight {
                        format.r#type = AttributeFormatType::UnsignedByte;
                    };

                    let vector = get_vector(file, format)?;

                    match att {
                        VSHAttribute::Position => {
                            let x: f32 = (vector[0].to_f32() * position_scale) + position_offset[0];
                            let y: f32 = (vector[1].to_f32() * position_scale) + position_offset[1];
                            let z: f32 = (vector[2].to_f32() * position_scale) + position_offset[2];
                            vertex.position = [x, y, z];
                        },
                        VSHAttribute::Normal => {
                            vertex.normal = [vector[0].to_f32() * normal_scale, vector[1].to_f32() * normal_scale, vector[2].to_f32() * normal_scale]
                        },
                        VSHAttribute::Tangent => {
                            vertex.tangent = [vector[0].to_f32() * tangent_scale, vector[1].to_f32() * tangent_scale, vector[2].to_f32() * tangent_scale]
                        },
                        unused => debug!("TODO: in object.rs: a {:?} is not used", unused)
                    };
                }

                if vertex.node.len() == 0 && node_list.len() <= 4 {
                    for n in &node_list {
                        vertex.node.push(n.clone().into())
                    };
                    if vertex.weight.is_empty() {
                        vertex.weight.push(1.0);
                    };
                };

                if skinning_mode != SkinningMode::SmoothSkinning && vertex.node.len() > 0 {
                    if vertex.weight.len() == 0 {
                        vertex.weight.push(1.0);
                    };
                    //vertex.position = vector3_tranform(vertex.position, skeleton)
                    debug!("TODO: some unimplemented tranform") //some transform
                }

                vertices.push(vertex);
                file.seek(SeekFrom::Start(file_position)).unwrap();
            };
        };

        debug!("TODO: in object.rs: bounding box");

        Ok(Object {
            vertices,
            material_id,
            render_priority,
            name,
            is_visible,
            //bounding_box is ignored
            has_normal,
            has_tangent,
            has_color,
            has_node,
            has_weight,
            tex_uv_count,
        })
    }
}
