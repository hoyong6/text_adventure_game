use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use serde_json::{Value, json};
use chrono;

struct MyWebSocket {
  hb: Instant
}

impl Actor for MyWebSocket {
  type Context = ws::WebsocketContext<Self>;
  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);
  }
}

impl MyWebSocket {
  fn new() -> Self {
    Self { hb: Instant::now() }
  }
  // 5s轮询心跳
  fn hb(&self, ctx: &mut <Self as Actor>::Context) {
    ctx.run_interval(Duration::from_secs(5), |act, ctx| {
        ctx.text(json!({
            "type": "heartbeat",
            "time": act.hb.elapsed().as_secs()
        }).to_string());
    });
  }
  fn handle_command(&self, command: &str) -> String {
    match command {
        "get_time" => {
            let now = chrono::Local::now();
            json!({
                "type": "time",
                "time": now.to_rfc3339()
            }).to_string()
        },
        "get_random_number" => {
            use rand::Rng;
            let number: u32 = rand::thread_rng().gen_range(1..101);
            json!({
                "type": "random_number",
                "number": number
            }).to_string()
        },
        _ => json!({
            "type": "error",
            "message": "Unknown command"
        }).to_string()
    }
  }
}

// 处理 WebSocket 消息 系统自带钩子函数
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
          self.hb = Instant::now();
          ctx.pong(&msg);
      },
      Ok(ws::Message::Text(text)) => {
          println!("Received message: {}", text);
          
          // 尝试将接收到的文本解析为 JSON
          if let Ok(json) = serde_json::from_str::<Value>(&text) {
              if let Some(command) = json.get("command").and_then(Value::as_str) {
                  let response = self.handle_command(command);
                  ctx.text(response);
              } else {
                  ctx.text(json!({
                      "type": "error",
                      "message": "Invalid command format"
                  }).to_string());
              }
          } else {
              // 如果不是 JSON，就按原来的方式处理
              ctx.text(format!("You said: {}", text));
          }
      },
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}

pub async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  ws::start(MyWebSocket::new(), &req, stream)
}