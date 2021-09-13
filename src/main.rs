#![cfg_attr(feature = "microbit", no_std)]
#![cfg_attr(feature = "microbit", no_main)]
#![macro_use]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

mod device;
use device::*;
use embassy::executor::Spawner;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        use wasm_bindgen::prelude::*;
        use drogue_wasm::*;

        type BUTTON = WebButton;
        type LED = WebLed;

        #[embassy::main]
        async fn main(spawner: Spawner) {
            wasm_logger::init(wasm_logger::Config::default());

            static mut INPUT1: InputPin = InputPin::new();
            static mut OUTPUT1: OutputPin = OutputPin::new();

            // Configure HTML elements
            unsafe {
                INPUT1.configure("button");
                OUTPUT1.configure("led", |value| {
                    if value {
                        log::info!("ON");
                        "ON"
                    } else {
                        log::info!("OFF");
                        "OFF"
                    }
                });
            }

            let button = WebButton::new(unsafe { &INPUT1 });
            let led = WebLed::new(unsafe { &OUTPUT1 });

            MyDevice::start(button, led, spawner).await;
        }

    } else if #[cfg(feature = "microbit")] {
        use panic_probe as _;
        use rtt_logger::RTTLogger;
        use rtt_target::rtt_init_print;
        use log::LevelFilter;
        use embedded_hal::digital::v2::OutputPin;

        use embassy_nrf::{
            gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
            gpiote::PortInput,
            peripherals::{P0_14},
            Peripherals,
        };

        static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Trace);

        type BUTTON = PortInput<'static, P0_14>;
        type LED = MatrixOutput;

        #[embassy::main]
        async fn main(spawner: Spawner, p: Peripherals) {
            rtt_init_print!();
            log::set_logger(&LOGGER).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            let button = PortInput::new(Input::new(p.P0_14, Pull::Up));
            let led = MatrixOutput {
                row: Output::new(p.P0_21.degrade(), Level::Low, OutputDrive::Standard),
                col: Output::new(p.P0_28.degrade(), Level::Low, OutputDrive::Standard),
            };

            MyDevice::start(button, led, spawner).await;
        }

        pub struct MatrixOutput {
            row: Output<'static, AnyPin>,
            col: Output<'static, AnyPin>,
        }

        impl OutputPin for MatrixOutput {
            type Error = ();

            fn set_low(&mut self) -> Result<(), ()> {
                self.row.set_low().unwrap();
                self.col.set_high().unwrap();
                Ok(())
            }

            fn set_high(&mut self) -> Result<(), ()> {
                self.row.set_high().unwrap();
                self.col.set_low().unwrap();
                Ok(())
            }
        }

    }
}
