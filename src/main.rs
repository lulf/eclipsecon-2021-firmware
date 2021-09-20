#![cfg_attr(feature = "microbit", no_std)]
#![cfg_attr(feature = "microbit", no_main)]
#![macro_use]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

mod app;
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
            static mut OUTPUT2: OutputPin = OutputPin::new();

            // Configure HTML elements
            unsafe {
                INPUT1.configure("button");
                OUTPUT1.configure("led1", |value| LedColor::Red + value );
                OUTPUT2.configure("led2", |value| LedColor::Green + value );
            }

            let button = WebButton::new(unsafe { &INPUT1 });
            let led1 = WebLed::new(unsafe { &OUTPUT1 });
            let led2 = WebLed::new(unsafe { &OUTPUT2 });

            MyDevice::start(button, led1, led2, spawner).await;
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
        use embassy::{time::Duration, util::Forever};
        use drogue_device::{
            *,
            drivers::led::matrix::*,
            actors::led::matrix::*,
        };

        static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Trace);
        static MATRIX: Forever<ActorContext<'static, LedMatrixActor<Output<'static, AnyPin>, 5, 5>, 10>> = Forever::new();

        type BUTTON = PortInput<'static, P0_14>;
        type LED = MatrixOutput;

        #[embassy::main]
        async fn main(spawner: Spawner, p: Peripherals) {
            rtt_init_print!();
            log::set_logger(&LOGGER).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            let output_pin = |pin: AnyPin| {
                Output::new(pin, Level::Low, OutputDrive::Standard)
            };

            // LED Matrix
            let rows = [
                output_pin(p.P0_21.degrade()),
                output_pin(p.P0_22.degrade()),
                output_pin(p.P0_15.degrade()),
                output_pin(p.P0_24.degrade()),
                output_pin(p.P0_19.degrade()),
            ];

            let cols = [
                output_pin(p.P0_28.degrade()),
                output_pin(p.P0_11.degrade()),
                output_pin(p.P0_31.degrade()),
                output_pin(p.P1_05.degrade()),
                output_pin(p.P0_30.degrade()),
            ];

            let matrix = LedMatrix::new(rows, cols);
            let matrix = MATRIX.put(ActorContext::new(LedMatrixActor::new(Duration::from_millis(1000 / 200), matrix)));
            let matrix = matrix.mount((), spawner);

            let button = PortInput::new(Input::new(p.P0_14, Pull::Up));
            let led1 = MatrixOutput::new(matrix.clone(), 0, 0);
            let led2 = MatrixOutput::new(matrix.clone(), 4, 4);

            MyDevice::start(button, led1, led2, spawner).await;
        }

        pub struct MatrixOutput {
            matrix: Address<'static, LedMatrixActor<Output<'static, AnyPin>, 5, 5>>,
            row: usize,
            col: usize,
        }

        impl MatrixOutput {
            pub fn new(address: Address<'static, LedMatrixActor<Output<'static, AnyPin>, 5, 5>>, row: usize, col: usize) -> Self {
                Self {
                    matrix: address,
                    row,
                    col,
                }
            }
        }

        impl OutputPin for MatrixOutput {
            type Error = ();

            fn set_low(&mut self) -> Result<(), ()> {
                self.matrix.notify(MatrixCommand::Off(self.row, self.col)).unwrap();
                Ok(())
            }

            fn set_high(&mut self) -> Result<(), ()> {
                self.matrix.notify(MatrixCommand::On(self.row, self.col)).unwrap();
                Ok(())
            }
        }

    }
}
