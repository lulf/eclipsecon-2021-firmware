use super::{BUTTON, LED};
use crate::app::{App, Command};
use drogue_device::{
    actors::{
        button::{Button, ButtonPressed},
        led::Led,
    },
    ActorContext, DeviceContext,
};
use embassy::executor::Spawner;

pub struct MyDevice {
    app: ActorContext<App>,
    led1: ActorContext<Led<LED>>,
    led2: ActorContext<Led<LED>>,
    button: ActorContext<Button<BUTTON, ButtonPressed<App>>>,
}

static DEVICE: DeviceContext<MyDevice> = DeviceContext::new();

impl MyDevice {
    pub async fn start(button: BUTTON, led1: LED, led2: LED, spawner: Spawner) {
        let device = DEVICE.configure(MyDevice {
            app: ActorContext::new(),
            button: ActorContext::new(),
            led1: ActorContext::new(),
            led2: ActorContext::new(),
        });

        let led1 = device.led1.mount(spawner, Led::new(led1));
        let led2 = device.led2.mount(spawner, Led::new(led2));
        let app = device.app.mount(spawner, App::new(led1, led2));
        device.button.mount(
            spawner,
            Button::new(button, ButtonPressed(app, Command::Toggle)),
        );
    }
}
