use super::{BUTTON, LED};
use crate::app::App;
use drogue_device::{
    actors::{button::Button, led::Led},
    ActorContext, DeviceContext,
};
use embassy::executor::Spawner;

pub struct MyDevice {
    app: ActorContext<'static, App>,
    led1: ActorContext<'static, Led<LED>>,
    led2: ActorContext<'static, Led<LED>>,
    button: ActorContext<'static, Button<'static, BUTTON, App>>,
}

static DEVICE: DeviceContext<MyDevice> = DeviceContext::new();

impl MyDevice {
    pub async fn start(button: BUTTON, led1: LED, led2: LED, spawner: Spawner) {
        DEVICE.configure(MyDevice {
            app: ActorContext::new(Default::default()),
            button: ActorContext::new(Button::new(button)),
            led1: ActorContext::new(Led::new(led1)),
            led2: ActorContext::new(Led::new(led2)),
        });

        DEVICE
            .mount(|device| async move {
                let led1 = device.led1.mount((), spawner);
                let led2 = device.led2.mount((), spawner);
                let app = device.app.mount((led1, led2), spawner);
                device.button.mount(app, spawner);
            })
            .await;
    }
}
