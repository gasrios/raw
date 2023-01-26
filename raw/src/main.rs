use data::{Tag, Type};
use endianness::{ByteOrder, ByteOrder::BigEndian, ByteOrder::LittleEndian};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData, Seek, SeekFrom};

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
    let buffer: [u8; 2] = tiff_reader::read(reader)?;
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
    let version: i16 = tiff_reader::read_i16(reader, *byte_order)?;
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
    *offset = u64::from(tiff_reader::read_u32(reader, *byte_order)?);

    /*
     * From TIFF 6.0 Specification, page 14: "There must be at least 1 IFD in a TIFF file and each
     * IFD must have at least one entry."
     *
     * As a side effect, we also fail here if offset == 0, that is, there are no IFDs in the file.
     *
     */
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
    let number_of_fields: u16 = tiff_reader::read_u16(reader, byte_order)?;

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
        let tag: u16 = tiff_reader::read_u16(reader, byte_order)?;
        entry.tag = data::tag(tag);

        /*
         * Bytes 2-3 The field Type.
         */
        entry.type_ = data::type_(tiff_reader::read_u16(reader, byte_order)?);

        /*
         * Bytes 4-7 The number of values, Count of the indicated Type.
         */
        entry.count = tiff_reader::read_u32(reader, byte_order)?;

        /*
         * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the field.
         *            The Value is expected to begin on a word boundary; the corresponding
         *            Value Offset will thus be an even number. This file offset may point
         *            anywhere in the file, even after the image data.
         */
        entry.offset = u64::from(tiff_reader::read_u32(reader, byte_order)?);

        println!("Tag: {:?}", entry.tag);
        println!("\tType: {:?}", entry.type_);
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
        if data::type_size(entry.type_) * entry.count < 5 {
            // TODO read those values
            match entry.type_ {
                Type::Byte => println!("\tType: Byte"),
                Type::Ascii => println!("\tType: Ascii"),
                Type::Short => println!("\tType: Short"),
                Type::Long => println!("\tType: Long"),
                Type::Rational => println!("\tType: Rational"),
                Type::Sbyte => println!("\tType: Sbyte"),
                Type::Undefined => println!("\tType: Undefined"),
                Type::Sshort => println!("\tType: Sshort"),
                Type::Slong => println!("\tType: Slong"),
                Type::Srational => println!("\tType: Srational"),
                Type::Float => println!("\tType: Float"),
                Type::Double => println!("\tType: Double"),
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
    *offset = u64::from(tiff_reader::read_u32(reader, byte_order)?);

    Ok(())
}

struct IfdEntry {
    tag: Tag,
    type_: Type,
    count: u32,
    offset: u64,
}

impl IfdEntry {
    fn new() -> IfdEntry {
        IfdEntry {
            tag: Tag::Unknown,
            type_: Type::Unknown,
            count: 0,
            offset: 0,
        }
    }
}
