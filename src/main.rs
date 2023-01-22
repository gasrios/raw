use endianness::{
    read_i16, read_u16, read_u32, ByteOrder, ByteOrder::BigEndian, ByteOrder::LittleEndian,
};
use std::env::args;
use std::fs::File;
use std::io::{
    BufReader, Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read, Seek, SeekFrom,
};

fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut buffered_reader: BufReader<File> = BufReader::new(File::open(file_name)?);

        /*
         * From TIFF 6.0 Specification, page 13
         *
         * Image File Header
         *
         * A TIFF file begins with an 8-byte image file header, containing the following information:
         *
         * Bytes 0-1: The byte order used within the file. Legal values are:
         *            “II” (4949.H)
         *            “MM” (4D4D.H)
         *
         *            In the “II” format, byte order is always from the least significant byte
         *            to the most significant byte, for both 16-bit and 32-bit integers This is
         *            called little-endian byte order. In the “MM” format, byte order is always
         *            from most significant to least significant, for both 16-bit and 32-bit
         *            integers. This is called big-endian byte order.
         */
        let byte_order: ByteOrder;
        {
            let buffer: [u8; 2] = read(&mut buffered_reader)?;
            if buffer[0] == 0x49 && buffer[1] == 0x49 {
                byte_order = LittleEndian;
            } else if buffer[0] == 0x4D && buffer[1] == 0x4D {
                byte_order = BigEndian;
            } else {
                return Err(Error::new(
                    InvalidData,
                    format!(
                        "Invalid byte order specification: {:?}. Legal values are “II” (4949.H) and “MM” (4D4D.H)",
                        &buffer
                    ),
                ));
            }
        }

        {
            /*
             * Bytes 2-3: An arbitrary but carefully chosen number (42) that further identifies the
             *            file as a TIFF file.
             *
             *            The byte order depends on the value of Bytes 0-1.
             */
            let buffer: [u8; 2] = read(&mut buffered_reader)?;
            // FIXME implement trait std::convert::From<T>
            let version: i16 = read_i16(&buffer, byte_order).unwrap();
            if version != 42 {
                return Err(Error::new(
                    InvalidData,
                    format!("Failed to further identify the file as a TIFF file, was expecting 42, found {version}"),
                ));
            }
        }

        /*
         * Bytes 4-7: The offset (in bytes) of the first IFD. The directory may be at any
         *            location in the file after the header but must begin on a word boundary.
         *            In particular, an Image File Directory may follow the image data it
         *            describes. Readers must follow the pointers wherever they may lead.
         *
         *            The term byte offset is always used in this document to refer to a
         *            location with respect to the beginning of the TIFF file. The first byte
         *            of the file has an offset of 0.
         */
        let mut offset: u64;
        {
            let buffer: [u8; 4] = read(&mut buffered_reader)?;
            // FIXME implement trait std::convert::From<T>
            // FIXME Test offset is even
            offset = u64::from(read_u32(&buffer, byte_order).unwrap());
        }
        if offset < 8 {
            return Err(Error::new(
                InvalidData,
                format!("First IFD offset is smaller than header size: {offset}"),
            ));
        }

        loop {
            println!("IFD offset: {offset}");

            buffered_reader.seek(SeekFrom::Start(offset))?;

            /*
             * From TIFF 6.0 Specification, page 14
             *
             * Image File Directory
             *
             * An Image File Directory (IFD) consists of a 2-byte count of the number of directory
             * entries (i.e., the number of fields), followed by a sequence of 12-byte field entries,
             * followed by a 4-byte offset of the next IFD (or 0 if none). (Do not forget to write the
             * 4 bytes of 0 after the last IFD.)
             *
             * There must be at least 1 IFD in a TIFF file and each IFD must have at least one entry.
             */
            let number_of_fields: u16;
            {
                let buffer: [u8; 2] = read(&mut buffered_reader)?;
                // FIXME implement trait std::convert::From<T>
                number_of_fields = read_u16(&buffer, byte_order).unwrap();
            }

            println!("Number of fields: {number_of_fields}");

            for _i in 0..number_of_fields {
                let mut entry: IfdEntry = IfdEntry::new();

                /*
                 * TIFF 6.0 Specification uses the terms "IFD Entry" and "field" with the same meaning, this
                 * is sometimes confusing.
                 */

                /*
                 * IFD Entry
                 *
                 * Each 12-byte IFD entry has the following format:
                 *
                 * Bytes 0-1 The Tag that identifies the field.
                 */
                {
                    let buffer: [u8; 2] = read(&mut buffered_reader)?;
                    // FIXME implement trait std::convert::From<T>
                    entry.tag = read_u16(&buffer, byte_order).unwrap();
                }

                // Bytes 2-3 The field Type.
                {
                    let buffer: [u8; 2] = read(&mut buffered_reader)?;
                    // FIXME implement trait std::convert::From<T>
                    let field_type: usize = usize::from(read_u16(&buffer, byte_order).unwrap());

                    if (1..13).contains(&field_type) {
                        entry.field_type = TYPES[field_type];
                    } else {
                        // See below. TYPES[0] == Unexpected
                        entry.field_type = TYPES[0];
                    }
                }

                // Bytes 4-7 The number of values, Count of the indicated Type.
                {
                    let buffer: [u8; 4] = read(&mut buffered_reader)?;
                    // FIXME implement trait std::convert::From<T>
                    entry.number_of_values = read_u32(&buffer, byte_order).unwrap();
                }

                /*
                 * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the field.
                 *            The Value is expected to begin on a word boundary; the corresponding
                 *            Value Offset will thus be an even number. This file offset may point
                 *            anywhere in the file, even after the image data.
                 */
                {
                    let buffer: [u8; 4] = read(&mut buffered_reader)?;
                    // FIXME implement trait std::convert::From<T>
                    // FIXME Test offset is even
                    entry.offset = u64::from(read_u32(&buffer, byte_order).unwrap());
                }

                println!("Tag: {}", entry.tag);
                println!("Field type: {:?}", entry.field_type());
                println!("Number of values: {}", entry.number_of_values);
                println!(
                    "Value size: {}",
                    entry.field_type.size_in_bytes * entry.number_of_values
                );
                println!("Value offset: {}", entry.offset);
            }

            {
                let buffer: [u8; 4] = read(&mut buffered_reader)?;
                // FIXME implement trait std::convert::From<T>
                offset = u64::from(read_u32(&buffer, byte_order).unwrap());
            }

            if offset == 0 {
                break;
            }
        }
    } else {
        return Err(Error::new(InvalidData, "Please specify a file"));
    }

    Ok(())
}

fn read<const BYTES2READ: usize>(r: &mut BufReader<File>) -> Result<[u8; BYTES2READ], Error> {
    let mut buffer: [u8; BYTES2READ] = [0u8; BYTES2READ];
    let bytes_read: usize = r.read(&mut buffer)?;
    if bytes_read != BYTES2READ {
        return Err(Error::new(
            UnexpectedEof,
            format!("Tried to read {BYTES2READ} bytes, found only {bytes_read} bytes available"),
        ));
    }
    Ok(buffer)
}

struct IfdEntry {
    tag: u16,
    field_type: Type,
    number_of_values: u32,
    offset: u64,
}

impl IfdEntry {
    fn new() -> IfdEntry {
        IfdEntry {
            tag: 0,
            field_type: Type {
                field_type: EnumType::Unknown,
                size_in_bytes: 0,
            },
            number_of_values: 0,
            offset: 0,
        }
    }

    fn field_type(&self) -> EnumType {
        self.field_type.field_type
    }
}

/*
 * From TIFF 6.0 Specification, page 14
 *
 * Types
 *
 * The field types and their sizes are:
 *  1 = BYTE 8-bit unsigned integer.
 *  2 = ASCII 8-bit byte that contains a 7-bit ASCII code; the last byte must be NUL (binary zero).
 *  3 = SHORT 16-bit (2-byte) unsigned integer.
 *  4 = LONG 32-bit (4-byte) unsigned integer.
 *  5 = RATIONAL Two LONGs: the first represents the numerator of a fraction; the second, the denominator.
 *  6 = SBYTE An 8-bit signed (twos-complement) integer.
 *  7 = UNDEFINED An 8-bit byte that may contain anything, depending on the definition of the field.
 *  8 = SSHORT A 16-bit (2-byte) signed (twos-complement) integer.
 *  9 = SLONG A 32-bit (4-byte) signed (twos-complement) integer.
 * 10 = SRATIONAL Two SLONG’s: the first represents the numerator of a fraction, the second the denominator.
 * 11 = FLOAT Single precision (4-byte) IEEE format.
 * 12 = DOUBLE Double precision (8-byte) IEEE format.
 *
 * Warning: It is possible that other TIFF field types will be added in the future. Readers should
 *          skip over fields containing an unexpected field type.
 */

/*
 * FIXME is there a better way to do this?
 *
 * In order to replicate the behavior of Java enums, Rust needs a combination of enum (for match)
 * and struct (to acess property "bytes")
 */
const TYPES: [Type; 13] = [
    Type {
        field_type: EnumType::Unexpected,
        size_in_bytes: 1,
    },
    Type {
        field_type: EnumType::Byte,
        size_in_bytes: 1,
    },
    Type {
        field_type: EnumType::Ascii,
        size_in_bytes: 1,
    },
    Type {
        field_type: EnumType::Short,
        size_in_bytes: 2,
    },
    Type {
        field_type: EnumType::Long,
        size_in_bytes: 4,
    },
    Type {
        field_type: EnumType::Rational,
        size_in_bytes: 8,
    },
    Type {
        field_type: EnumType::Sbyte,
        size_in_bytes: 1,
    },
    Type {
        field_type: EnumType::Undefined,
        size_in_bytes: 1,
    },
    Type {
        field_type: EnumType::Sshort,
        size_in_bytes: 2,
    },
    Type {
        field_type: EnumType::Slong,
        size_in_bytes: 4,
    },
    Type {
        field_type: EnumType::Srational,
        size_in_bytes: 8,
    },
    Type {
        field_type: EnumType::Float,
        size_in_bytes: 4,
    },
    Type {
        field_type: EnumType::Double,
        size_in_bytes: 8,
    },
];

#[derive(Clone, Copy)]
struct Type {
    field_type: EnumType,
    size_in_bytes: u32,
}

#[derive(Clone, Copy, Debug)]
enum EnumType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    Sbyte,
    Undefined,
    Sshort,
    Slong,
    Srational,
    Float,
    Double,
    Unknown,
    Unexpected,
}
