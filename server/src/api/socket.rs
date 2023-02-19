use std::time::Instant;

use actix::prelude::*;
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use rand::Rng;

use crate::{actors::socket::Socket, mongo_db::MongoRepo};
use crate::actors::socket_data::SocketData;
use crate::models::socket_messages::{Connect, Disconnect, SocketMessage};

impl Handler<Connect> for SocketData {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();

        self.add_recipient(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for SocketData {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.remove_recipient(msg.id);
    }
}

impl Handler<SocketMessage> for SocketData {
    type Result = ();

    fn handle(&mut self, msg: SocketMessage, _: &mut Context<Self>) {
        self.send_all(msg);
    }
}

pub async fn grid_socket_index(
    req: HttpRequest,
    stream: web::Payload,
    socket_data: Data<Addr<SocketData>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        Socket {
            id: 0,
            data: socket_data.get_ref().clone(),
            db: MongoRepo::init().await,
            hb: Instant::now(),
        },
        &req,
        stream,
    );
    resp
}
