use cgmath::Euler;

pub struct UiState {
    pub xscale: f32,
    pub yscale: f32,
    pub zscale: f32,
    pub xrot: f32,
    pub yrot: f32,
    pub zrot: f32,
    pub model_updated: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            xscale: 1.0,
            yscale: 1.0,
            zscale: 1.0,
            xrot: 0.0,
            yrot: 0.0,
            zrot: 0.0,
            model_updated: false,
        }
    }

    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from(Euler {
            x: cgmath::Deg(self.xrot),
            y: cgmath::Deg(self.yrot),
            z: cgmath::Deg(self.zrot),
        }) * cgmath::Matrix4::from_nonuniform_scale(self.xscale, self.yscale, self.zscale)
    }
}

pub fn render_ui(_state: &mut UiState, ui: &dear_imgui_rs::Ui) {
    ui.window("Wesley").build(|| {
        ui.text("Scale:");
        _state.model_updated = {
            ui.slider_f32("X##scale", &mut _state.xscale, 0.1, 10.0)
                | ui.slider_f32("Y##scale", &mut _state.yscale, 0.1, 10.0)
                | ui.slider_f32("Z##scale", &mut _state.zscale, 0.1, 10.0)
        };
        ui.text("Rotation (deg):");
        _state.model_updated |= {
            ui.slider_f32("X##rot", &mut _state.xrot, -180.0, 180.0)
                | ui.slider_f32("Y##rot", &mut _state.yrot, -180.0, 180.0)
                | ui.slider_f32("Z##rot", &mut _state.zrot, -180.0, 180.0)
        };
    });
}
