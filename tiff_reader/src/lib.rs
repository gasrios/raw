/*
 * © 2023 Guilherme Rios All Rights Reserved
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.
 */

use endianness::ByteOrder;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::UnexpectedEof, Read};

// TODO all methods here should extend trait TiffReader

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

/// # Errors
///
/// TODO add docs
pub fn read_offset(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<u64, Error> {
    // We need to convert TIFF offset type u32 to Rust's u64
    Ok(u64::from(read_u32(reader, byte_order)?))
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
pub fn read<const SIZE: usize>(reader: &mut BufReader<File>) -> Result<[u8; SIZE], Error> {
    let mut buffer: [u8; SIZE] = [0u8; SIZE];
    let bytes_read: usize = reader.read(&mut buffer)?;
    if bytes_read != SIZE {
        return Err(Error::new(
            UnexpectedEof,
            format!("Tried to read {SIZE} bytes, found only {bytes_read} bytes available"),
        ));
    }
    Ok(buffer)
}