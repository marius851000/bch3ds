#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SkinningMode {
    None,
    SmoothSkinning,
    RigidSkinning,
}

impl SkinningMode {
    pub fn new(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::SmoothSkinning),
            2 => Some(Self::RigidSkinning),
            _ => None,
        }
    }
}
