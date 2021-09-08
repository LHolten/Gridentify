use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use futures_util::sink::SinkExt;
use futures_util::{Future, FutureExt, StreamExt};
use rustls_acme::TlsStream;
use serde::{Deserialize, Serialize};
use simple_error::{bail, try_with, SimpleError, SimpleResult};

pub trait JsonConnection {
    type Send<'a>: 'a + Future<Output = SimpleResult<()>>;
    fn send_serialize<'a, T: ?Sized + Serialize>(&'a mut self, data: &T) -> Self::Send<'a>;
    type Receive<'a, T: 'a>: 'a + Future<Output = SimpleResult<T>>;
    fn receive_deserialize<'b, T: for<'a> Deserialize<'a>>(&'b mut self) -> Self::Receive<'b, T>;
}

fn as_simple<O, T: std::error::Error>(result: Result<O, T>) -> SimpleResult<O> {
    result.map_err(SimpleError::from)
}

fn websocket_send(
    ws: &mut WebSocketStream<TlsStream>,
    string_data: String,
) -> impl '_ + Future<Output = SimpleResult<()>> {
    ws.send(Message::Text(string_data)).map(as_simple)
}

impl JsonConnection for WebSocketStream<TlsStream> {
    type Send<'a> = impl 'a + Future<Output = SimpleResult<()>>;

    fn send_serialize<'a, J: ?Sized + Serialize>(&'a mut self, data: &J) -> Self::Send<'a> {
        let string_data = serde_json::to_string(data).unwrap();
        websocket_send(self, string_data)
    }

    type Receive<'a, T: 'a> = impl 'a + Future<Output = SimpleResult<T>>;

    fn receive_deserialize<'b, T: for<'a> Deserialize<'a>>(&'b mut self) -> Self::Receive<'b, T> {
        async move {
            if let Some(message) = self.next().await {
                let message = try_with!(message, "couldn\'t read websocket message");
                let text = try_with!(message.into_text(), "message is not text");
                serde_json::from_str(&text).map_err(SimpleError::from)
            } else {
                bail!("no message")
            }
        }
    }
}
