use actix::prelude::*;
use actix::{Actor, Recipient, StreamHandler};
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

// session wrapper
pub struct SocketData {
    pub sessions: Vec<Recipient<Message>>,
}

impl SocketData {
    pub fn new() -> Self {
        Self { sessions: vec![] }
    }
}

impl Actor for SocketData {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Handler<Connect> for SocketData {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.push(msg.addr);
        self.sessions.len() - 1
    }
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message {
    pub text: String,
}

/// Define HTTP actor
pub struct Session {
    pub data: Addr<SocketData>,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            _ => (),
        }
    }
}

pub async fn grid_socket_index(
    req: HttpRequest,
    stream: web::Payload,
    socket_data: Data<Addr<SocketData>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        Session {
            data: socket_data.get_ref().clone(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}
