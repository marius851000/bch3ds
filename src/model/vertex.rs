#[derive(Clone, Default, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub texture0: [f32; 2],
    pub texture1: [f32; 2],
    pub texture2: [f32; 2],
    pub node: Vec<i32>,
    pub weight: Vec<f32>,
    pub diffuse_color: u32,
}
