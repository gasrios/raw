use endianness::{read_i16, read_u32, ByteOrder, ByteOrder::BigEndian, ByteOrder::LittleEndian};
use std::env;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read};

fn main() -> Result<(), Error> {
    if let Some(file_name) = env::args().nth(1) {
        let mut buffered_reader: BufReader<File>;
        let byte_order: ByteOrder;
        let offset: u32;

        // Process header to determine endianness and obtain first offset
        {
            buffered_reader = BufReader::new(File::open(file_name)?);

            let mut buffer: [u8; 4] = [0u8; 4];
            let mut total_bytes_read: usize;

            // Endianess
            total_bytes_read = buffered_reader.read(&mut buffer[..2])?;
            if total_bytes_read != 2 {
                return Err(Error::new(
                    UnexpectedEof,
                    format!(
                        "Unexpected end of file reading endianess: {total_bytes_read:?} bytes read"
                    ),
                ));
            }
            if buffer[0] == 0x49 && buffer[1] == 0x49 {
                byte_order = LittleEndian;
            } else if buffer[0] == 0x4D && buffer[1] == 0x4D {
                byte_order = BigEndian;
            } else {
                return Err(Error::new(
                    InvalidData,
                    format!("Invalid endianness specification: {:?}", &buffer[..2]),
                ));
            }

            // Version
            total_bytes_read = buffered_reader.read(&mut buffer[..2])?;
            if total_bytes_read != 2 {
                return Err(Error::new(
                    UnexpectedEof,
                    format!(
                        "Unexpected end of file reading version: {total_bytes_read:?} bytes read"
                    ),
                ));
            }
            let version: i16 = read_i16(&buffer[..2], byte_order).unwrap();
            if version != 42 {
                return Err(Error::new(
                    InvalidData,
                    format!(
                        "Failed to further identify the file as a TIFF file. Version: {version:?}"
                    ),
                ));
            }

            // Offset
            total_bytes_read = buffered_reader.read(&mut buffer)?;
            if total_bytes_read != 4 {
                return Err(Error::new(
                    UnexpectedEof,
                    format!(
                        "Unexpected end of file reading offset: {total_bytes_read:?} bytes read"
                    ),
                ));
            }
            offset = read_u32(&buffer, byte_order).unwrap();
            if offset < 8 {
                return Err(Error::new(
                    InvalidData,
                    format!("Offset is smaller than header size: {offset:?}"),
                ));
            }
        }

        println!("First offset: {offset:?}");
    }
    Ok(())
}
