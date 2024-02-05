#[cfg(target_os="linux")]
use rpi_led_matrix::{LedMatrixOptions, LedRuntimeOptions, LedMatrix, LedColor};

#[cfg(not(target_os="linux"))]
mod simulator;
#[cfg(not(target_os="linux"))]
use crate::simulator::{LedMatrixOptions, LedRuntimeOptions, LedMatrix, LedColor, led_canvas::LedCanvasTrait};

macro_rules! led_color {
    ($hex:literal) => {
        {
            let hex = if $hex.starts_with("#") {
                &$hex[1..]
            } else {
                $hex
            };

            LedColor::from(u32::from_str_radix(hex, 16).unwrap())
        }
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

    canvas.clear();

    // Set of colors
    let colors = vec![
        led_color!("#2EC866"),
        led_color!("#003865"),
        led_color!("#FF0000"),
    ];

    // busy loop
    // cycle colors indefinitely
    let mut i = 0;
    loop {
        let color = colors[i];

        // Draw a box outline with diagonals through the middle
        // and a circle in the center

        canvas.draw_line(0, 0, 63, 0, &color);
        canvas.draw_line(63, 0, 63, 31, &color);
        canvas.draw_line(63, 31, 0, 31, &color);
        canvas.draw_line(0, 31, 0, 0, &color);
        canvas.draw_line(0, 0, 63, 31, &color);
        canvas.draw_line(63, 0, 0, 31, &color);
        canvas.draw_circle(32, 16, 10, &color);


        // @FIXME: 
        // Because the simulator uses a double buffer
        // (our own pixel buffer and the minifb window buffer)
        // there is a race condition where the next
        // draw_line or draw_circle will be called before
        // the previous draw call has finished.
        // This is not an issue with the real LED matrix
        // because that directly talks to GPIO.
        // Ideally, the simulator should be updated to
        // mimic the hardware more closely.
        // For now, this 'flush_buffer' call is a workaround
        // to make the simulator work as expected.
        #[cfg(not(target_os="linux"))]
        println!("Drawing color: {:?}", color);
        #[cfg(not(target_os="linux"))]
        canvas.flush_buffer();

        std::thread::sleep(std::time::Duration::from_millis(250));
        canvas.clear();
        i = (i + 1) % colors.len();
    }
}
