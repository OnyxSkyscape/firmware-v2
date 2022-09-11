use std::{thread, time::Duration};

use crate::serial::*;
use embedded_hal::{self, digital::v2::OutputPin, prelude::_embedded_hal_blocking_delay_DelayMs};
use esp_idf_hal::{delay::Ets, gpio, peripherals::Peripherals};

pub struct BoardController {
    serial: SerialController,
    amplifier_power: gpio::GpioPin<gpio::Output>,
    module_power: gpio::GpioPin<gpio::Output>,
}

impl BoardController {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().expect("Failed to take peripherals");
        let pins = peripherals.pins;
        let amplifier_power = pins.gpio27.into_output().unwrap();
        let module_power = pins.gpio13.into_output().unwrap();
        let serial_clock = pins.gpio12.into_output().unwrap();
        let serial_data = pins.gpio14.into_output().unwrap();
        Self {
            serial: SerialController::new(serial_clock.degrade(), serial_data.degrade()),
            amplifier_power: amplifier_power.degrade(),
            module_power: module_power.degrade(),
        }
    }

    #[allow(unused)]
    pub fn write_color(&mut self, color: u32, panel: u8) {
        self.serial.write(Demux::Reset, 0x1ff);
        Ets.delay_ms(50_u32);
        self.serial.write(Demux::Select, panel as u16);
        Ets.delay_ms(50_u32);
        self.serial.write(Demux::Enable, 0x1ff);
        for i in (0..3).rev() {
            Ets.delay_ms(50_u32);
            self.serial
                .write(Demux::Color, ((color >> (i * 8)) as u16 & 0xff) << 1);
        }
    }

    fn setup_pins(&mut self) {
        self.serial.clock_pin.set_high().unwrap();
        self.serial.data_pin.set_high().unwrap();

        // NOTE: insufficiently large delay may result in a malfunction
        // recommended default value is 700 ms
        Ets.delay_ms(700_u32);
        self.amplifier_power.set_high().unwrap();
        self.module_power.set_high().unwrap();
        Ets.delay_ms(700_u32);
    }

    pub fn init_boards(&mut self) {
        self.setup_pins();
        self.serial.init();
        self.serial.write(Demux::Reset, 0x1ff);
        Ets.delay_ms(50_u32);
        for i in 0..512 {
            self.serial.write(Demux::Select, i);
            if i % 50 == 0 {
                thread::sleep(Duration::from_millis(10));
            }
        }
        Ets.delay_ms(50_u32);
        self.serial.write(Demux::Enable, 0x1ff);
        for _ in 0..3 {
            Ets.delay_ms(50_u32);
            self.serial.write(Demux::Color, 0);
        }
    }
}
