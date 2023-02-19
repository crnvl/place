use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};

use crate::{
    models::{
        point::Point,
        socket_messages::{Connect, Disconnect, SocketMessage},
    },
    mongo_db::MongoRepo,
};

use super::socket_data::SocketData;

pub struct Socket {
    pub id: usize,
    pub data: Addr<SocketData>,
    pub db: MongoRepo,
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
                let point: Point = serde_json::from_str(&text).unwrap();

                let db = self.db.clone();
                let data = self.data.clone();
                let id = self.id.clone();
                let fut = async move {
                    db.create_or_update_point(point).await;

                    let text = serde_json::to_string::<Vec<Point>>(db.get_all_points().await.as_ref()).unwrap();
                    data.do_send(SocketMessage {
                        id: id,
                        text: text,
                    });
                };
                let fut = actix::fut::wrap_future::<_, Self>(fut);
                ctx.spawn(fut);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
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
        /*         ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
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
        }); */
    }
}
