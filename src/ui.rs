pub struct UiState {}

impl UiState {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn render_ui(_state: &mut UiState, ui: &dear_imgui_rs::Ui) {
    ui.window("Wesley").build(|| {
        ui.text("Testing...");
    });
}
