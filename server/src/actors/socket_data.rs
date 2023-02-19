use std::collections::HashMap;

use actix::{Context, Actor, Recipient};
use rand::rngs::ThreadRng;

use crate::{models::{socket_messages::SocketMessage}};

pub struct SocketData {
    pub sessions: HashMap<usize, Recipient<SocketMessage>>,
    pub rng: ThreadRng
}

impl SocketData {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn add_recipient(&mut self, id: usize, recipient: Recipient<SocketMessage>) {
        self.sessions.insert(id, recipient);
    }

    pub fn remove_recipient(&mut self, id: usize) {
        self.sessions.remove(&id);
    }

    pub fn send_all(&self, msg: SocketMessage) {
        for (_, recipient) in self.sessions.iter() {
            recipient.do_send(msg.clone());
        }
    }
}

impl Actor for SocketData {
    type Context = Context<Self>;
}
