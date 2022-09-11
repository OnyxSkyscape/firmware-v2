use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{self, digital::v2::OutputPin};
use esp_idf_hal::{delay::Ets, gpio};

#[allow(dead_code)]
pub enum Demux {
    Select = 0b00,
    Color = 0b01,
    Enable = 0b10,
    Reset = 0b11,
}

pub struct SerialController {
    pub clock_pin: gpio::GpioPin<gpio::Output>,
    pub data_pin: gpio::GpioPin<gpio::Output>,
}

impl SerialController {
    pub fn new(
        mut clock_pin: gpio::GpioPin<gpio::Output>,
        mut data_pin: gpio::GpioPin<gpio::Output>,
    ) -> Self {
        clock_pin.set_high().unwrap();
        data_pin.set_high().unwrap();

        Self {
            clock_pin,
            data_pin,
        }
    }

    fn flip_bits(data: u16) -> u16 {
        let mut out = 0;
        for i in 0..9 {
            out += ((data >> i) & 1) << (8 - i);
        }
        out
    }

    fn invert_bits(data: u16) -> u16 {
        data ^ 0x1ff
    }

    fn send_clock(&mut self) {
        Ets.delay_ms(1_u32);
        self.clock_pin.set_low().unwrap();
        Ets.delay_ms(1_u32);
        self.clock_pin.set_high().unwrap();
    }

    fn send_bit(&mut self) {
        Ets.delay_ms(1_u32);
        self.data_pin.set_low().unwrap();
        Ets.delay_ms(1_u32);
        self.data_pin.set_high().unwrap();
    }

    fn raw_write(&mut self, data: u16) {
        self.send_clock();
        for i in (0..11).rev() {
            if (data >> i) & 1 == 1 {
                self.send_bit();
            }
            self.send_clock();
        }
        self.send_clock()
    }

    pub fn write(&mut self, demux: Demux, data: u16) {
        let data = ((Self::flip_bits(Self::invert_bits(data)) as u16) << 2) ^ demux as u16;
        self.raw_write(data);
        //self.log(data);
    }

    pub fn init(&mut self) {
        for _ in 0..12 {
            self.send_clock();
        }
    }
    #[allow(unused)]
    fn log(&self, reg: u16) {
        println!("{:011b}", reg);
    }
}
