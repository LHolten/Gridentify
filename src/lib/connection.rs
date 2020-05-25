use native_tls::TlsStream;
use serde::{Deserialize, Serialize};
use simple_error::bail;
use simple_error::try_with;
use simple_error::SimpleResult;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use tungstenite::{Message, WebSocket};

pub trait JsonConnection {
    fn send<T: Serialize>(&mut self, data: &T) -> SimpleResult<()>;
    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> SimpleResult<T>;
    fn set_nodelay(&mut self, v: bool) -> SimpleResult<()>;
}

impl JsonConnection for TcpStream {
    fn send<T: Serialize>(&mut self, data: &T) -> SimpleResult<()> {
        let mut msg = serde_json::to_string(data).unwrap();
        msg.push('\n');
        Ok(try_with!(
            self.write_all(msg.as_bytes()),
            "couldn't write message"
        ))
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> SimpleResult<T> {
        let mut msg = String::new();
        try_with!(
            BufReader::new(self).read_line(&mut msg),
            "couldn't read line from connection"
        );
        Ok(try_with!(
            serde_json::from_str(msg.as_str()),
            "not well formatted json"
        ))
    }

    fn set_nodelay(&mut self, v: bool) -> SimpleResult<()> {
        Ok(try_with!(
            TcpStream::set_nodelay(&self, v),
            "could not set no delay"
        ))
    }
}

impl JsonConnection for WebSocket<TlsStream<TcpStream>> {
    fn send<T: Serialize>(&mut self, data: &T) -> SimpleResult<()> {
        let string_data = serde_json::to_string(data).unwrap();
        try_with!(
            self.write_message(Message::Text(string_data)),
            "couldn\'t write message"
        );
        Ok(try_with!(self.write_pending(), "couldn\'t write message"))
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> SimpleResult<T> {
        let message = try_with!(self.read_message(), "couldn\'t read websocket message");
        if let Message::Text(value) = message {
            return Ok(try_with!(
                serde_json::from_str(value.as_str()),
                "not well formatted json"
            ));
        }
        bail!("got non string message")
    }

    fn set_nodelay(&mut self, v: bool) -> SimpleResult<()> {
        Ok(try_with!(
            self.get_mut().get_mut().set_nodelay(v),
            "couldn't set no delay"
        ))
    }
}
