#![cfg_attr(feature = "microbit", no_std)]
#![cfg_attr(feature = "microbit", no_main)]
#![macro_use]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

mod app;
mod device;
use device::*;
use embassy::executor::Spawner;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        //use wasm_bindgen::prelude::*;
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
            gpio::{Input},
            peripherals::{P0_14},
            Peripherals,
        };
        use drogue_device::{
            *,
            actors::led::matrix::MatrixCommand,
            bsp::boards::nrf52::microbit::*,
        };

        static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Trace);
        static MATRIX: ActorContext<LedMatrixActor, 10> = ActorContext::new();

        type BUTTON = Input<'static, P0_14>;
        type LED = MatrixOutput;

        #[embassy::main]
        async fn main(spawner: Spawner, p: Peripherals) {
            rtt_init_print!();
            log::set_logger(&LOGGER).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            let board = Microbit::new(p);
            let matrix = MATRIX.mount(spawner, LedMatrixActor::new(board.display, None));

            // Max brightness
            matrix.request(MatrixCommand::MaxBrightness).unwrap().await;

            let led1 = MatrixOutput::new(matrix.clone(), 0, 0);
            let led2 = MatrixOutput::new(matrix.clone(), 4, 4);

            MyDevice::start(board.btn_a, led1, led2, spawner).await;
        }

        pub struct MatrixOutput {
            matrix: Address<LedMatrixActor>,
            row: usize,
            col: usize,
        }

        impl MatrixOutput {
            pub fn new(address: Address<LedMatrixActor>, row: usize, col: usize) -> Self {
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
