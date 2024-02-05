pub(crate) struct LedRuntimeOptions {
    pub(crate) gpio_slowdown: u32,
}

impl LedRuntimeOptions {
    pub fn new() -> Self {
        Self {
            gpio_slowdown: 1,
        }
    }

    pub fn set_gpio_slowdown(&mut self, gpio_slowdown: u32) {
        self.gpio_slowdown = gpio_slowdown;
    }
}