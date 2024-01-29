use rpi_led_matrix::{LedMatrixOptions, LedRuntimeOptions, LedMatrix, LedColor};

macro_rules! led_color {
    ($red:expr, $green:expr, $blue:expr) => {
        &LedColor{ red:$red, green:$green, blue:$blue }
    }
}


fn main() {
    let mut options = LedMatrixOptions::new();
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat");

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);
    
    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    // Reset canvas
    canvas.fill(led_color!(0, 0, 0));
}
