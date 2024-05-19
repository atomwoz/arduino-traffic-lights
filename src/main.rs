#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::{port::mode, port::Pin, Peripherals};

struct Semaphore {
    red: Pin<mode::Output>,
    yellow: Pin<mode::Output>,
    green: Pin<mode::Output>,
}

impl Semaphore {
    fn make_green(&mut self) {
        self.red.set_low();
        self.yellow.set_low();
        self.green.set_high();
    }
    
    fn make_yellow(&mut self) {
        self.red.set_low();
        self.yellow.set_high();
        self.green.set_low();
    }

    fn make_red(&mut self) {
        self.red.set_high();
        self.yellow.set_low();
        self.green.set_low();
    }

    fn make_transform_to_green(&mut self) {
        self.red.set_high();
        self.yellow.set_high();
        self.green.set_low();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut main_road_semaphore = Semaphore {
        red: pins.d5.into_output().downgrade(),
        yellow: pins.d6.into_output().downgrade(),
        green: pins.d7.into_output().downgrade(),
    };
    
    let mut side_road_semaphore = Semaphore {
        red: pins.d8.into_output().downgrade(),
        yellow: pins.d9.into_output().downgrade(),
        green: pins.d10.into_output().downgrade(),
    };

    loop {
        // Main road green, side road red
        main_road_semaphore.make_green();
        side_road_semaphore.make_red();
        arduino_hal::delay_ms(5000);
        
        // Main road yellow, side road red
        main_road_semaphore.make_yellow();
        side_road_semaphore.make_red();
        arduino_hal::delay_ms(2000);

        // Both red (buffer)
        main_road_semaphore.make_red();
        side_road_semaphore.make_red();
        arduino_hal::delay_ms(1000);

        // Main road red, side road transform to green
        side_road_semaphore.make_transform_to_green();
        arduino_hal::delay_ms(2000);

        // Main road red, side road green
        main_road_semaphore.make_red();
        side_road_semaphore.make_green();
        arduino_hal::delay_ms(5000);

        // Main road red, side road yellow
        main_road_semaphore.make_red();
        side_road_semaphore.make_yellow();
        arduino_hal::delay_ms(2000);

        // Both red (buffer)
        main_road_semaphore.make_red();
        side_road_semaphore.make_red();
        arduino_hal::delay_ms(1000);

        // Main road transform to green, side road red
        main_road_semaphore.make_transform_to_green();
        arduino_hal::delay_ms(2000);
    }
}
