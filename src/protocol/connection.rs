use futures_util::sink::SinkExt;
use futures_util::{FutureExt, StreamExt};
use log::{log, Level};
use serde::{Deserialize, Serialize};
use simple_error::{bail, try_with, SimpleError, SimpleResult};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub type MyStream = WebSocketStream<TlsStream<TcpStream>>;

fn as_simple<O, T: std::error::Error>(result: Result<O, T>) -> SimpleResult<O> {
    result.map_err(SimpleError::from)
}

pub async fn send_serialize<J: ?Sized + Serialize>(
    stream: &mut MyStream,
    data: &J,
) -> SimpleResult<()> {
    let string_data = serde_json::to_string(data).unwrap();
    log!(Level::Trace, "sending message {}", &string_data);
    stream
        .send(Message::Text(string_data))
        .map(as_simple)
        .await?;
    stream.flush().map(as_simple).await
}

pub async fn receive_deserialize<T: for<'a> Deserialize<'a>>(
    stream: &mut MyStream,
) -> SimpleResult<T> {
    if let Some(message) = stream.next().await {
        let message = try_with!(message, "couldn\'t read websocket message");
        let text = try_with!(message.into_text(), "message is not text");
        serde_json::from_str(&text).map_err(SimpleError::from)
    } else {
        bail!("no message")
    }
}
