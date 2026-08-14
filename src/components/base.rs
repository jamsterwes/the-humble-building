pub struct TransformComponent {
    pub pos: (f32, f32, f32),
    pub rot: (f32, f32, f32),
    pub scale: (f32, f32, f32),
}

impl TransformComponent {
    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_translation(self.pos.into())
            * cgmath::Matrix4::from(cgmath::Euler {
                x: cgmath::Deg(self.rot.0),
                y: cgmath::Deg(self.rot.1),
                z: cgmath::Deg(self.rot.2),
            })
            * cgmath::Matrix4::from_nonuniform_scale(self.scale.0, self.scale.1, self.scale.2)
    }
}

// TODO: add something
pub struct CameraComponent {}
