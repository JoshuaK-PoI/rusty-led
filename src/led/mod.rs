#[macro_export]
macro_rules! led_color {
    ($hex:literal) => {{
        let hex = if $hex.starts_with("#") {
            &$hex[1..]
        } else {
            $hex
        };

        LedColor::from(u32::from_str_radix(hex, 16).unwrap())
    }};
    ($hex:expr) => {{
        LedColor::from(($hex as u32))
    }};
}
