use actix::{Message, Recipient};

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct SocketMessage {
    pub id: usize,
    pub text: String,
}

#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<SocketMessage>,
}

/// Session is disconnected
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
