use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;

pub(crate) fn send<T: Serialize>(data: &T, stream: &mut TcpStream) -> Result<(), Error> {
    let mut vec_data = serde_json::to_vec(data).unwrap();
    vec_data.push(0);
    stream.write(vec_data.as_slice())?;
    Ok(())
}

pub(crate) fn receive<T: for<'a> Deserialize<'a>>(stream: &mut TcpStream) -> Result<T, Error> {
    let mut data = Vec::new();
    let num_bytes = BufReader::new(stream).read_until(0, &mut data)?;
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
