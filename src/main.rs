mod controller;
mod network;
mod serial;
use controller::BoardController;

use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::delay::Ets;
use esp_idf_sys as _;

#[allow(unused_imports)]
use network::init_wifi;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // uart::uart_test();
    let mut board = BoardController::new();
    board.init_boards();
    Ets.delay_ms(200_u32);
    board.write_color(0xaaff00, 9);
}
