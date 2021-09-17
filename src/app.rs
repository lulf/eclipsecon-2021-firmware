use super::LED;
use core::future::Future;
use drogue_device::{
    actors::{button::*, led::*},
    kernel::actor::*,
};

#[derive(Debug)]
pub enum Command {
    Toggle,
}

pub struct App {
    address: Option<Address<'static, App>>,
    led1: Option<Address<'static, Led<LED>>>,
    led2: Option<Address<'static, Led<LED>>>,
    state: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: false,
            address: Default::default(),
            led1: Default::default(),
            led2: Default::default(),
        }
    }
}

impl FromButtonEvent<Command> for App {
    fn from(event: ButtonEvent) -> Option<Command>
    where
        Self: Sized,
    {
        match event {
            ButtonEvent::Pressed => Some(Command::Toggle),
            _ => None,
        }
    }
}

impl Actor for App {
    type Configuration = (Address<'static, Led<LED>>, Address<'static, Led<LED>>);

    #[rustfmt::skip]
    type Message<'m> = Command;

    #[rustfmt::skip]
    type OnMountFuture<'m, M> where M: 'm = impl Future<Output = ()> + 'm;
    fn on_mount<'m, M>(
        &'m mut self,
        config: Self::Configuration,
        address: Address<'static, Self>,
        inbox: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<'m, Self> + 'm,
    {
        self.address.replace(address);
        self.view_state(&config.0, &config.1).ok();
        self.led1.replace(config.0);
        self.led2.replace(config.1);

        async move {
            loop {
                match inbox.next().await {
                    Some(mut m) => match m.message() {
                        Command::Toggle => self.toggle().ok(),
                    },
                    _ => {}
                }
            }
        }
    }
}

impl App {
    fn toggle(&mut self) -> Result<(), ActorError> {
        self.state = !self.state;
        match (self.led1, self.led2) {
            (Some(led1), Some(led2)) => {
                self.view_state(&led1, &led2)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn view_state(
        &self,
        led1: &Address<Led<LED>>,
        led2: &Address<Led<LED>>,
    ) -> Result<(), ActorError> {
        led1.notify(LedMessage::State(self.state))?;
        led2.notify(LedMessage::State(!self.state))?;
        Ok(())
    }
}
