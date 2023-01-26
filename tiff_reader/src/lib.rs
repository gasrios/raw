use endianness::ByteOrder;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::UnexpectedEof, Read};

/// # Panics
///
/// TODO add docs
///
/// # Errors
///
/// TODO add docs
pub fn read_i16(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<i16, Error> {
    let buffer: [u8; 2] = read(reader)?;
    Ok(endianness::read_i16(&buffer, byte_order).unwrap())
}

/// # Panics
///
/// TODO add docs
///
/// # Errors
///
/// TODO add docs
pub fn read_u16(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<u16, Error> {
    let buffer: [u8; 2] = read(reader)?;
    Ok(endianness::read_u16(&buffer, byte_order).unwrap())
}

/// # Panics
///
/// TODO add docs
///
/// # Errors
///
/// TODO add docs
pub fn read_u32(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<u32, Error> {
    let buffer: [u8; 4] = read(reader)?;
    Ok(endianness::read_u32(&buffer, byte_order).unwrap())
}

/// # Panics
///
/// TODO add docs
///
/// # Errors
///
/// TODO add docs
pub fn read<const BYTES2READ: usize>(
    reader: &mut BufReader<File>,
) -> Result<[u8; BYTES2READ], Error> {
    let mut buffer: [u8; BYTES2READ] = [0u8; BYTES2READ];
    let bytes_read: usize = reader.read(&mut buffer)?;
    if bytes_read != BYTES2READ {
        return Err(Error::new(
            UnexpectedEof,
            format!("Tried to read {BYTES2READ} bytes, found only {bytes_read} bytes available"),
        ));
    }
    Ok(buffer)
}
