
#[derive(Debug, Clone, Copy)]
pub(crate) struct LedColor {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl LedColor {
    pub(crate) fn zero() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Into<u32> for LedColor {
    fn into(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}

impl From<u32> for LedColor {
    fn from(color: u32) -> Self {
        Self {
            red: ((color >> 16) & 0xFF) as u8,
            green: ((color >> 8) & 0xFF) as u8,
            blue: (color & 0xFF) as u8,
        }
    }
}

impl Into<String> for LedColor {
    fn into(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}