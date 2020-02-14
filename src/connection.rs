use native_tls::TlsStream;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::net::TcpStream;
use tungstenite::{Message, WebSocket};

pub(crate) trait JsonConnection {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<()>;
    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T>;
    fn set_nodelay(&mut self, v: bool) -> Result<()>;
}

impl JsonConnection for TcpStream {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let mut vec_data = serde_json::to_vec(data).unwrap();
        vec_data.push(0);
        self.write_all(vec_data.as_slice())?;
        Ok(())
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
        let mut data = Vec::new();
        BufReader::new(self).read_until(0, &mut data)?;
        data.pop()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "no message"))?;
        Ok(serde_json::from_slice(data.as_slice())?)
    }

    fn set_nodelay(&mut self, v: bool) -> Result<()> {
        TcpStream::set_nodelay(&self, v)
    }
}

impl JsonConnection for WebSocket<TlsStream<TcpStream>> {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let string_data = serde_json::to_string(data).unwrap();
        self.write_message(Message::Text(string_data))
            .or_else(|_| {
                Err(Error::new(
                    ErrorKind::Interrupted,
                    "couldn\'t write message",
                ))
            })?;
        self.write_pending().or_else(|_| {
            Err(Error::new(
                ErrorKind::Interrupted,
                "couldn\'t write message",
            ))
        })?;
        Ok(())
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
        let message = self
            .read_message()
            .or_else(|_| Err(Error::new(ErrorKind::Interrupted, "couldn\'t read message")))?;
        if let Message::Text(value) = message {
            return Ok(serde_json::from_str(value.as_str())?);
        }
        Err(Error::new(ErrorKind::InvalidData, "got weird message"))
    }

    fn set_nodelay(&mut self, v: bool) -> Result<()> {
        self.get_mut().get_mut().set_nodelay(v)
    }
}
