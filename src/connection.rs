use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;
use tungstenite::{Message, WebSocket};

pub trait JsonConnection {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<(), Error>;
    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, Error>;
    fn set_nodelay(&mut self, v: bool) -> Result<(), Error>;
}

impl JsonConnection for TcpStream {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<(), Error> {
        let mut vec_data = serde_json::to_vec(data).unwrap();
        vec_data.push(0);
        self.write(vec_data.as_slice())?;
        Ok(())
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, Error> {
        let mut data = Vec::new();
        let num_bytes = BufReader::new(self).read_until(0, &mut data)?;
        if num_bytes == 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "couldn\'t deserialize data",
            ));
        }
        data.pop().unwrap();
        serde_json::from_slice(data.as_slice()).or(Err(Error::new(
            ErrorKind::InvalidData,
            "couldn\'t deserialize data",
        )))
    }

    fn set_nodelay(&mut self, v: bool) -> Result<(), Error> {
        TcpStream::set_nodelay(&self, v)
    }
}

impl JsonConnection for WebSocket<TcpStream> {
    fn send<T: Serialize>(&mut self, data: &T) -> Result<(), Error> {
        let string_data = serde_json::to_string(data).unwrap();
        self.write_message(Message::Text(string_data))
            .or(Err(Error::new(
                ErrorKind::Interrupted,
                "couldn\'t write message",
            )))?;
        self.write_pending().or(Err(Error::new(
            ErrorKind::Interrupted,
            "couldn\'t write message",
        )))?;
        Ok(())
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, Error> {
        let message = self.read_message().or(Err(Error::new(
            ErrorKind::Interrupted,
            "couldn\'t read message",
        )))?;
        if let Message::Text(value) = message {
            return serde_json::from_str(value.as_str()).or(Err(Error::new(
                ErrorKind::InvalidData,
                "couldn\'t deserialize data",
            )));
        }
        Err(Error::new(ErrorKind::InvalidData, "got weird message"))
    }

    fn set_nodelay(&mut self, v: bool) -> Result<(), Error> {
        self.get_mut().set_nodelay(v)
    }
}
