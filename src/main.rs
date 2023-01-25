use endianness::{ByteOrder, ByteOrder::BigEndian, ByteOrder::LittleEndian};
use std::env::args;
use std::fs::File;
use std::io::{
    BufReader, Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read, Seek, SeekFrom,
};
use Type::{
    Ascii, Byte, Double, Float, Long, Rational, Sbyte, Short, Slong, Srational, Sshort, Undefined,
    Unexpected, Unknown,
};

fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut reader: BufReader<File> = BufReader::new(File::open(file_name)?);

        // It does not matter which value we initialize those with, we just need something so they
        // can be passed as parameters, below. process_header() will set the right values later.
        let mut byte_order: ByteOrder = BigEndian;
        let mut offset: u64 = 0;

        process_header(&mut reader, &mut byte_order, &mut offset)?;

        loop {
            if offset % 2 == 1 {
                return Err(Error::new(
                    InvalidData,
                    format!("Value offset is odd and therefore not a word boundary: {offset}"),
                ));
            }

            println!("IFD offset: {offset}");

            process_ifd(&mut reader, byte_order, &mut offset)?;

            /*
             * From TIFF 6.0 Specification, page 14
             *
             * An Image File Directory (IFD) consists of (...) followed by a 4-byte offset of the
             * next IFD (or 0 if none).
             */
            if offset == 0 {
                break;
            }
        }
    } else {
        return Err(Error::new(InvalidData, "Please specify a file"));
    }

    Ok(())
}

fn process_header(
    reader: &mut BufReader<File>,
    byte_order: &mut ByteOrder,
    offset: &mut u64,
) -> Result<(), Error> {
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
    let buffer: [u8; 2] = read(reader)?;
    if buffer[0] == 0x49 && buffer[1] == 0x49 {
        *byte_order = LittleEndian;
    } else if buffer[0] == 0x4D && buffer[1] == 0x4D {
        *byte_order = BigEndian;
    } else {
        return Err(Error::new(
            InvalidData,
            format!(
                "Invalid byte order specification: {:?}. Legal values are “II” (4949.H) and “MM” (4D4D.H)",
                &buffer
            ),
        ));
    }

    /*
     * Bytes 2-3: An arbitrary but carefully chosen number (42) that further identifies the
     *            file as a TIFF file.
     *
     *            The byte order depends on the value of Bytes 0-1.
     */
    let version: i16 = read_i16(reader, *byte_order)?;
    if version != 42 {
        return Err(Error::new(
            InvalidData,
            format!("Failed to further identify the file as a TIFF file, was expecting 42, found {version}"),
        ));
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
    *offset = u64::from(read_u32(reader, *byte_order)?);

    if *offset < 8 {
        return Err(Error::new(
            InvalidData,
            format!("First IFD offset is smaller than header size: {}", *offset),
        ));
    }

    Ok(())
}

fn process_ifd(
    reader: &mut BufReader<File>,
    byte_order: ByteOrder,
    offset: &mut u64,
) -> Result<(), Error> {
    reader.seek(SeekFrom::Start(*offset))?;

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
    let number_of_fields: u16 = read_u16(reader, byte_order)?;

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
        // TODO process numeric value of tag
        entry.tag = read_u16(reader, byte_order)?;

        /*
         * Bytes 2-3 The field Type.
         */
        let type_: usize = usize::from(read_u16(reader, byte_order)?);

        if (1..13).contains(&type_) {
            entry.type_ = TYPES[type_];
        } else {
            // See below. TYPES[13] == Unexpected
            entry.type_ = TYPES[13];
        }

        /*
         * Bytes 4-7 The number of values, Count of the indicated Type.
         */
        entry.count = read_u32(reader, byte_order)?;

        /*
         * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the field.
         *            The Value is expected to begin on a word boundary; the corresponding
         *            Value Offset will thus be an even number. This file offset may point
         *            anywhere in the file, even after the image data.
         */
        entry.offset = u64::from(read_u32(reader, byte_order)?);

        println!("Tag: {}", entry.tag);
        println!("\tType: {:?}", entry.type_());
        println!("\tNumber of values: {}", entry.count);

        /*
         * From TIFF 6.0 Specification, page 15
         *
         * Value/Offset
         *
         * To save time and space the Value Offset contains the Value instead of pointing to the
         * Value if and only if the Value fits into 4 bytes. If the Value is shorter than 4 bytes,
         * it is left-justified within the 4-byte Value Offset, i.e., stored in the lower-numbered
         * bytes. Whether the Value fits within 4 bytes is determined by the Type and Count of the
         * field.
         */
        if entry.type_.size_in_bytes * entry.count < 5 {
            // TODO read those values
            match entry.type_() {
                Byte => println!("\tType: Byte"),
                Ascii => println!("\tType: Ascii"),
                Short => println!("\tType: Short"),
                Long => println!("\tType: Long"),
                Rational => println!("\tType: Rational"),
                Sbyte => println!("\tType: Sbyte"),
                Undefined => println!("\tType: Undefined"),
                Sshort => println!("\tType: Sshort"),
                Slong => println!("\tType: Slong"),
                Srational => println!("\tType: Srational"),
                Float => println!("\tType: Float"),
                Double => println!("\tType: Double"),
                _ => println!("\tType: Other"),
            }
        } else {
            if entry.offset % 2 == 1 {
                return Err(Error::new(
                    InvalidData,
                    format!(
                        "Value offset is odd and therefore not a word boundary: {}",
                        entry.offset
                    ),
                ));
            }
            println!("\tValue offset: {}", entry.offset);
        }
    }

    /*
     * From TIFF 6.0 Specification, page 13
     *
     * The directory may be at any location in the file after the header but must begin on
     * a word boundary.
     */
    *offset = u64::from(read_u32(reader, byte_order)?);

    Ok(())
}

fn read_i16(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<i16, Error> {
    let buffer: [u8; 2] = read(reader)?;
    Ok(endianness::read_i16(&buffer, byte_order).unwrap())
}

fn read_u16(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<u16, Error> {
    let buffer: [u8; 2] = read(reader)?;
    Ok(endianness::read_u16(&buffer, byte_order).unwrap())
}

fn read_u32(reader: &mut BufReader<File>, byte_order: ByteOrder) -> Result<u32, Error> {
    let buffer: [u8; 4] = read(reader)?;
    Ok(endianness::read_u32(&buffer, byte_order).unwrap())
}

fn read<const BYTES2READ: usize>(reader: &mut BufReader<File>) -> Result<[u8; BYTES2READ], Error> {
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

struct IfdEntry {
    tag: u16,
    type_: Type_,
    count: u32,
    offset: u64,
}

impl IfdEntry {
    fn new() -> IfdEntry {
        IfdEntry {
            tag: 0,
            // See below. TYPES[0] == Unknown
            type_: TYPES[0],
            count: 0,
            offset: 0,
        }
    }

    fn type_(&self) -> Type {
        self.type_.type_
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

#[derive(Clone, Copy, Debug)]
enum Type {
    Unknown,
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
    Unexpected,
}

/*
 * In order to replicate the behavior of Java enums, Rust needs a combination of enum (for match)
 * and struct (to acess property "size_in_bytes")
 *
 * FIXME is there a better way to do this?
 */
const TYPES: [Type_; 14] = [
    Type_ {
        type_: Unknown,
        size_in_bytes: 0,
    },
    Type_ {
        type_: Byte,
        size_in_bytes: 1,
    },
    Type_ {
        type_: Ascii,
        size_in_bytes: 1,
    },
    Type_ {
        type_: Short,
        size_in_bytes: 2,
    },
    Type_ {
        type_: Long,
        size_in_bytes: 4,
    },
    Type_ {
        type_: Rational,
        size_in_bytes: 8,
    },
    Type_ {
        type_: Sbyte,
        size_in_bytes: 1,
    },
    Type_ {
        type_: Undefined,
        size_in_bytes: 1,
    },
    Type_ {
        type_: Sshort,
        size_in_bytes: 2,
    },
    Type_ {
        type_: Slong,
        size_in_bytes: 4,
    },
    Type_ {
        type_: Srational,
        size_in_bytes: 8,
    },
    Type_ {
        type_: Float,
        size_in_bytes: 4,
    },
    Type_ {
        type_: Double,
        size_in_bytes: 8,
    },
    Type_ {
        type_: Unexpected,
        size_in_bytes: 1,
    },
];

#[derive(Clone, Copy)]
struct Type_ {
    type_: Type,
    size_in_bytes: u32,
}
