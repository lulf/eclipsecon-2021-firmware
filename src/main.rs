#![no_std]
#![no_main]
#![macro_use]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

mod app;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        mod system;
    } else if #[cfg(feature = "microbit")] {
        use panic_probe as _;
        use rtt_logger::RTTLogger;
        use rtt_target::rtt_init_print;

        use embassy_nrf::{
            gpio::{AnyPin, Input, Level, NoPin, Output, OutputDrive, Pin, Pull},
            gpiote::PortInput,
            interrupt,
            peripherals::{P0_14, P0_21},
            Peripherals,
        };

        static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Trace);

        struct MatrixOutput {
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

use app::*;
use drogue_device::{DeviceContext, Package};
use embedded_hal::digital::v2::{InputPin, OutputPin};

use log::LevelFilter;

struct MyDevice {
    #[cfg(feature = "microbit")]
    app: App<PortInput<'static, P0_14>, MatrixOutput>,
    #[cfg(feature = "wasm")]
    app: App<WebButton, WebLed>,
}

static DEVICE: DeviceContext<MyDevice> = DeviceContext::new();

#[cfg(feature = "microbit")]
#[embassy::main]
async fn main(spawner: embassy::executor::Spawner, p: Peripherals) {
    rtt_init_print!();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let button = PortInput::new(Input::new(p.P0_14, Pull::Up));
    let led = MatrixOutput {
        row: Output::new(p.P0_21.degrade(), Level::Low, OutputDrive::Standard),
        col: Output::new(p.P0_28.degrade(), Level::Low, OutputDrive::Standard),
    };

    log::info!("Hello, started!");
    DEVICE.configure(MyDevice {
        app: App::new(button, led),
    });

    DEVICE
        .mount(|device| async move { device.app.mount((), spawner) })
        .await;
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
