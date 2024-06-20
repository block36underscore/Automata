use super::Field;

#[derive(Clone, Copy)]
pub struct Material {
    pub id: u16,
}

pub type MaterialField<const W: usize, const H: usize> = Field<Material, W, H>;
