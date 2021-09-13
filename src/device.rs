use super::{BUTTON, LED};

use drogue_device::{ActorContext, actors::{led::Led, button::Button}, DeviceContext};
use embassy::executor::Spawner;

pub struct MyDevice {
    led: ActorContext<'static, Led<LED>>,
    button: ActorContext<'static, Button<'static, BUTTON, Led<LED>>>,
}

static DEVICE: DeviceContext<MyDevice> = DeviceContext::new();

impl MyDevice {
    pub async fn start(button: BUTTON, led: LED, spawner: Spawner) {
        DEVICE.configure(MyDevice {
            button: ActorContext::new(Button::new(button)),
            led: ActorContext::new(Led::new(led)),
        });

        DEVICE
            .mount(|device| async move {
                let led = device.led.mount((), spawner);
                device.button.mount(led, spawner);
            })
            .await;
    }
}
