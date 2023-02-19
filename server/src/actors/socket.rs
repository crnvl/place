use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::models::socket_messages::{Disconnect, Connect, SocketMessage};

use super::socket_data::SocketData;

pub struct Socket {
    pub id: usize,
    pub data: Addr<SocketData>,
    pub hb: Instant,
}

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.data
            .send(Connect {
                addr: addr.recipient::<SocketMessage>(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.data.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Socket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::debug!("Websocket message: {:?}", msg);
        match msg {
            Ok(ws::Message::Text(text)) => {
                let m = text.trim();

                self.data.do_send(SocketMessage { id: self.id, text: m.to_string() });
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            },
            _ => (),
        }
    }
}

impl Handler<SocketMessage> for Socket {
    type Result = ();

    fn handle(&mut self, msg: SocketMessage, ctx: &mut Self::Context) {
        ctx.text(msg.text);
    }
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
impl Socket {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                log::debug!("Websocket Client heartbeat failed, disconnecting!");
                
                act.data.do_send(Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
