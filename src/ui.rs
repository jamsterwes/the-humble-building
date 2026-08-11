use cgmath::Euler;

pub struct UiState {
    pub scale: (f32, f32, f32),
    pub rot: (f32, f32, f32),
    pub model_updated: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            scale: (1.0, 1.0, 1.0),
            rot: (0.0, 0.0, 0.0),
            model_updated: false,
        }
    }

    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from(Euler {
            x: cgmath::Deg(self.rot.0),
            y: cgmath::Deg(self.rot.1),
            z: cgmath::Deg(self.rot.2),
        }) * cgmath::Matrix4::from_nonuniform_scale(self.scale.0, self.scale.1, self.scale.2)
    }
}

// Returns true if a slider changed this frame
pub fn slider_vec3_f32(
    ui: &dear_imgui_rs::Ui,
    id: &'static str,
    value: &mut (f32, f32, f32),
    min: (f32, f32, f32),
    max: (f32, f32, f32),
) -> bool {
    let mut x = value.0;
    let mut y = value.1;
    let mut z = value.2;
    let updated = ui.slider_f32(format!("X##{}", id), &mut x, min.0, max.0)
        | ui.slider_f32(format!("Y##{}", id), &mut y, min.1, max.1)
        | ui.slider_f32(format!("Z##{}", id), &mut z, min.2, max.2);
    *value = (x, y, z);
    return updated;
}

pub fn render_ui(_state: &mut UiState, ui: &dear_imgui_rs::Ui) {
    ui.window("Model Matrix").build(|| {
        ui.text("Scale:");
        _state.model_updated = slider_vec3_f32(
            ui,
            "scale",
            &mut _state.scale,
            (0.1, 0.1, 0.1),
            (10.0, 10.0, 10.0),
        );
        ui.text("Rotation (deg):");
        _state.model_updated |= slider_vec3_f32(
            ui,
            "rot",
            &mut _state.rot,
            (-180.0, -180.0, -180.0),
            (180.0, 180.0, 180.0),
        );
    });
}
