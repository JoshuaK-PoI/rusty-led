pub(crate) struct LedMatrixOptions {
    pub(crate) rows: u32,
    pub(crate) cols: u32,
    pub(crate) hardware_mapping: String,
}

impl LedMatrixOptions {
    pub fn new() -> Self {
        Self {
            rows: 32,
            cols: 32,
            hardware_mapping: "regular".to_string(),
        }
    }

    pub fn set_cols(&mut self, cols: u32) {
        self.cols = cols;
    }

    pub fn set_rows(&mut self, rows: u32) {
        self.rows = rows;
    }

    pub fn set_hardware_mapping(&mut self, hardware_mapping: &str) {
        self.hardware_mapping = hardware_mapping.to_string();
    }
}
