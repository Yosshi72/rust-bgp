use tracing::{debug, info, instrument};

use crate::config::Config;
use crate::event::Event;
use crate::event_queue::{self, EventQueue};
use crate::state::State;

#[derive(Debug)]
pub struct Peer {
    state: State,
    event_queue: EventQueue,
    config: Config,
}

impl Peer {
    pub fn new(config: Config) -> Self {
        let state = State::Idle;
        let event_queue = EventQueue::new();
        Self {
            state,
            event_queue,
            config,
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        info!("peer is started");
        self.event_queue.enqueue(Event::ManualStart);
    }

    #[instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.event_queue.dequeue() {
            info!("event is occured, event = {:?}", event);
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match &self.state {
            State::Idle => match event {
                Event::ManualStart => {
                    self.state = State::Connect;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
