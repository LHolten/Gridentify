use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;

pub(crate) fn send<T: Serialize>(data: &T, stream: &mut TcpStream) -> Result<(), Error> {
    let mut vec_data = serde_json::to_vec(data).unwrap();
    vec_data.push(b'\n');
    stream.write(vec_data.as_slice())?;
    Ok(())
}

pub(crate) fn receive<'a, T: Deserialize<'a>>(
    data: &'a mut Vec<u8>,
    stream: &mut TcpStream,
) -> Result<T, Error> {
    data.clear();
    BufReader::new(stream).read_until(b'\n', data)?;
    serde_json::from_slice(data.as_slice()).or(Err(Error::new(
        ErrorKind::InvalidData,
        "could't deserialize",
    )))
}
