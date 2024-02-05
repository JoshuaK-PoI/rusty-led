use super::{
    led_canvas::LedCanvas, led_matrix_options::LedMatrixOptions,
    led_runtime_options::LedRuntimeOptions,
};

pub(crate) struct LedMatrix {
    pub(crate) options: LedMatrixOptions,
    pub(crate) _runtime_options: LedRuntimeOptions,
}

impl LedMatrix {
    pub fn new(
        options: Option<LedMatrixOptions>,
        _runtime_options: Option<LedRuntimeOptions>,
    ) -> Result<Self, String> {
        let options = options.unwrap_or(LedMatrixOptions::new());
        let _runtime_options = _runtime_options.unwrap_or(LedRuntimeOptions::new());

        Ok(Self {
            options,
            _runtime_options,
        })
    }

    pub fn offscreen_canvas(&self) -> LedCanvas {
        LedCanvas::new(self.options.rows, self.options.cols)
    }
}
