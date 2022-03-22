use super::LED;
use core::future::Future;
use drogue_device::{actors::led::*, kernel::actor::*};

#[derive(Debug, Clone)]
pub enum Command {
    Toggle,
}

pub struct App {
    led1: Address<Led<LED>>,
    led2: Address<Led<LED>>,
    state: bool,
}

impl App {
    pub fn new(led1: Address<Led<LED>>, led2: Address<Led<LED>>) -> Self {
        Self {
            state: false,
            led1,
            led2,
        }
    }
}

impl Actor for App {
    #[rustfmt::skip]
    type Message<'m> = Command;

    #[rustfmt::skip]
    type OnMountFuture<'m, M> = impl Future<Output = ()> + 'm where M: 'm + Inbox<Self>;

    fn on_mount<'m, M>(
        &'m mut self,
        _: Address<Self>,
        inbox: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<Self> + 'm,
    {
        self.view_state().ok();
        async move {
            loop {
                match inbox.next().await {
                    Some(mut m) => match m.message() {
                        Command::Toggle => {
                            self.toggle().ok();
                        }
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
        self.view_state()?;
        Ok(())
    }

    fn view_state(&self) -> Result<(), ActorError> {
        self.led1.notify(LedMessage::State(self.state))?;
        self.led2.notify(LedMessage::State(!self.state))?;
        Ok(())
    }
}
