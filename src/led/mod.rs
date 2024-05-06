
#[macro_export]
macro_rules! led_color {
    ($hex:literal) => {{
        let hex = if $hex.starts_with("#") {
            &$hex[1..]
        } else {
            $hex
        };
        let color_as_u32 = u32::from_str_radix(hex, 16)
            .expect("Could not parse hex value to u32");

        LedColor {
            red: ((color_as_u32 >> 16) & 0xFF) as u8,
            green: ((color_as_u32 >> 8) & 0xFF) as u8,
            blue: (color_as_u32 & 0xFF) as u8
        }
    }};

}
