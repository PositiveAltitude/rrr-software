use ws2812_esp32_rmt_driver::*;
use ws2812_esp32_rmt_driver::driver::color::*;


pub struct LedDriver {
    ws2812: Ws2812Esp32RmtDriver,
}

impl LedDriver {
    pub fn new(
        led_pin: u32,
        rmt_channel: u8,
    ) -> Result<Self, Ws2812Esp32RmtDriverError> {
        let ws =
            Ws2812Esp32RmtDriver::new(rmt_channel, led_pin);

        ws.map(|ws| Self { ws2812: ws })
    }

    pub fn set_rgb(&mut self,r: u8, g: u8, b: u8) -> Result<(), Ws2812Esp32RmtDriverError> {
        self.ws2812.write(LedPixelColorGrb24::new_with_rgb(r, g, b).as_ref())
    }
    pub fn off(&mut self) -> Result<(), Ws2812Esp32RmtDriverError> {
        self.ws2812.write(LedPixelColorGrb24::new_with_rgb(0, 0, 0).as_ref())
    }
}