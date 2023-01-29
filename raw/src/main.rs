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

use data::{IfdEntry, Offset, Tag, Type};
use endianness::{ByteOrder, ByteOrder::BigEndian, ByteOrder::LittleEndian};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData, Seek, SeekFrom};

fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut reader: BufReader<File> = BufReader::new(File::open(file_name)?);

        // It does not matter which value we initialize byte_order with, we just need something so
        // it can be passed as parameter, below. process_header() will set the right value later.
        let mut byte_order: ByteOrder = BigEndian;

        let mut offset: Offset = process_header(&mut reader, &mut byte_order)?;
        loop {
            offset = process_ifd(&mut reader, byte_order, offset)?;

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
) -> Result<Offset, Error> {
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
    let offset: Offset = tiff_reader::read_offset(reader, *byte_order)?;

    /*
     * From TIFF 6.0 Specification, page 14: "There must be at least 1 IFD in a TIFF file and each
     * IFD must have at least one entry."
     *
     * As a side effect, we also fail here if offset == 0, that is, there are no IFDs in the file.
     *
     */
    if offset < 8 {
        return Err(Error::new(
            InvalidData,
            format!("First IFD offset is smaller than header size: {offset}"),
        ));
    }

    Ok(offset)
}

fn process_ifd(
    reader: &mut BufReader<File>,
    byte_order: ByteOrder,
    offset: Offset,
) -> Result<Offset, Error> {
    reader.seek(SeekFrom::Start(offset))?;
    /*
     * Note: TIFF 6.0 Specification uses the terms "IFD Entry" and "field" with the same
     * meaning, this is sometimes confusing.
     */

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
        /*
         * IFD Entry
         *
         * Each 12-byte IFD entry has the following format:
         *
         * Bytes 0-1 The Tag that identifies the field.
         */
        let mut entry: IfdEntry =
            IfdEntry::new(Tag::new(tiff_reader::read_u16(reader, byte_order)?));

        /*
         * Bytes 2-3 The field Type.
         */
        entry.type_ = Type::new(tiff_reader::read_u16(reader, byte_order)?);

        /*
         * From TIFF 6.0 Specification, page 14
         *
         * Warning: It is possible that other TIFF field types will be added in the future. Readers should
         *          skip over fields containing an unexpected field type.
         */
        if entry.type_ == Type::Unexpected(0) {
            break;
        }

        if entry.type_ == Type::Unknown(0) {
            return Err(Error::new(
                InvalidData,
                format!("Invalid IFD Entry type: {:?}", entry.type_),
            ));
        }

        /*
         * Bytes 4-7 The number of values, Count of the indicated Type.
         */
        entry.count = tiff_reader::read_u32(reader, byte_order)?;

        if entry.count < 1 {
            return Err(Error::new(
                InvalidData,
                format!("IFD Entry should have at least one value: {}", entry.count),
            ));
        }

        /*
         * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the field.
         *
         * Note: under the hood, tiff_reader converts offset from u32 to u64, which is the type
         *       std::io::BufReader expects.
         */
        entry.offset = tiff_reader::read_offset(reader, byte_order)?;

        println!("Tag: {:?}", entry.tag);
        println!("\tType: {:?}", entry.type_);
        println!("\tNumber of values: {}", entry.count);

        /*
         * The Value is expected to begin on a word boundary; the corresponding Value Offset will
         * thus be an even number. This file offset may point anywhere in the file, even after the
         * image data.
         *
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
        if entry.size_in_bytes() < 5 {
            // TODO read those values
            match entry.type_ {
                Type::Byte(_type_size) => println!("\tType: Byte"),
                Type::Ascii(_type_size) => println!("\tType: Ascii"),
                Type::Short(_type_size) => println!("\tType: Short"),
                Type::Long(_type_size) => println!("\tType: Long"),
                Type::Rational(_type_size) => println!("\tType: Rational"),
                Type::Sbyte(_type_size) => println!("\tType: Sbyte"),
                Type::Undefined(_type_size) => println!("\tType: Undefined"),
                Type::Sshort(_type_size) => println!("\tType: Sshort"),
                Type::Slong(_type_size) => println!("\tType: Slong"),
                Type::Srational(_type_size) => println!("\tType: Srational"),
                Type::Float(_type_size) => println!("\tType: Float"),
                Type::Double(_type_size) => println!("\tType: Double"),
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

    let offset: Offset = tiff_reader::read_offset(reader, byte_order)?;
    /*
     * From TIFF 6.0 Specification, page 13
     *
     * The directory may be at any location in the file after the header but must begin on
     * a word boundary.
     */
    if offset % 2 == 1 {
        return Err(Error::new(
            InvalidData,
            format!("Value offset is odd and therefore not a word boundary: {offset}"),
        ));
    }

    Ok(offset)
}
