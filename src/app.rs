use drogue_device::{
    actors::{button::Button, led::Led},
    Actor, ActorContext, ActorSpawner, Address, Package,
};
use embassy::traits::gpio::WaitForAnyEdge;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct App<BUTTON, LED>
where
    BUTTON: WaitForAnyEdge + InputPin + 'static,
    LED: OutputPin + 'static,
{
    led: ActorContext<'static, Led<LED>>,
    button: ActorContext<'static, Button<'static, BUTTON, Led<LED>>>,
}

impl<BUTTON, LED> App<BUTTON, LED>
where
    BUTTON: WaitForAnyEdge + InputPin + 'static,
    LED: OutputPin + 'static,
{
    pub fn new(button: BUTTON, led: LED) -> Self {
        Self {
            led: ActorContext::new(Led::new(led)),
            button: ActorContext::new(Button::new(button)),
        }
    }
}

impl<BUTTON, LED> Package for App<BUTTON, LED>
where
    BUTTON: WaitForAnyEdge + InputPin + 'static,
    LED: OutputPin + 'static,
{
    type Primary = Button<'static, BUTTON, Led<LED>>;
    type Configuration = ();
    fn mount<S: ActorSpawner>(
        &'static self,
        _: Self::Configuration,
        spawner: S,
    ) -> Address<'static, Self::Primary> {
        log::info!("mounting led");
        let led = self.led.mount((), spawner);
        log::info!("mounting button");
        self.button.mount(led, spawner)
    }
}
