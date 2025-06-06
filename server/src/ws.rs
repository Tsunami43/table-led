use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;

pub struct WsSession;

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            println!("WS received: {}", text);
            ctx.text(format!("Echo: {}", text));
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WsSession {}, &req, stream)
}

