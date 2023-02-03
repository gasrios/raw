/*
 * © 2023 Guilherme Rios All Rights Reserved
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License as published by the Free Software Foundation, version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
 * the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If
 * not, see http://www.gnu.org/licenses/.
 */

use data::{IfdEntry, Offset, Tag, Type};
use std::io::{Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read, Seek, SeekFrom};
use Endianness::{BigEndian, LittleEndian};

pub struct TiffReader<R> {
    reader: R,
    endianness: Endianness,
}

impl<R: Read + Seek> TiffReader<R> {
    /// # Errors
    ///
    /// TODO add docs
    pub fn new(reader: R) -> Result<TiffReader<R>, Error> {
        Ok(TiffReader {
            reader,
            // It does not matter which value we initialize endianness with, process_header() will
            // set the right value later.
            endianness: BigEndian,
        })
    }

    /// # Errors
    ///
    /// TODO add docs
    pub fn process_header(&mut self) -> Result<Offset, Error> {
        /*
         * From TIFF 6.0 Specification, page 13
         *
         * Image File Header
         *
         * A TIFF file begins with an 8-byte image file header, containing the following
         * information:
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
        let buffer: [u8; 2] = self.read()?;
        if buffer[0] == 0x49 && buffer[1] == 0x49 {
            self.endianness = LittleEndian;
        } else if buffer[0] == 0x4D && buffer[1] == 0x4D {
            self.endianness = BigEndian;
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
        let version: i16 = self.read_i16()?;
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
         *
         * Note: under the hood, tiff_reader converts offset from u32 to u64, which is the type
         *       std::io::BufReader expects.
         */
        let offset: Offset = self.read_offset()?;

        /*
         * From TIFF 6.0 Specification, page 14: "There must be at least 1 IFD in a TIFF file and
         * each IFD must have at least one entry."
         *
         * As a side effect, we also fail here if offset == 0, that is, there are no IFDs in the
         * file.
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

    /// # Errors
    ///
    /// TODO add docs
    ///
    /// # Panics
    ///
    /// TODO add docs
    // TODO do not return offset, struct containing all info (including offset)
    pub fn process_ifd(&mut self, offset: Offset) -> Result<Offset, Error> {
        self.reader.seek(SeekFrom::Start(offset))?;
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
        let number_of_fields: u16 = self.read_u16()?;

        for _i in 0..number_of_fields {
            /*
             * IFD Entry
             *
             * Each 12-byte IFD entry has the following format:
             *
             * Bytes 0-1 The Tag that identifies the field.
             */
            let tag: Tag = self.read_tag()?;

            /*
             * Bytes 2-3 The field Type.
             */
            let type_: Type = self.read_type()?;

            /*
             * From TIFF 6.0 Specification, page 14
             *
             * Warning: It is possible that other TIFF field types will be added in the future.
             *          Readers should skip over fields containing an unexpected field type.
             */
            if type_ == Type::Unexpected {
                break;
            }

            if type_ == Type::Unknown {
                return Err(Error::new(
                    InvalidData,
                    format!("Invalid IFD Entry type: {type_:?}",),
                ));
            }

            /*
             * Bytes 4-7 The number of values, Count of the indicated Type.
             */
            let count: u32 = self.read_u32()?;

            if count < 1 {
                return Err(Error::new(
                    InvalidData,
                    format!("IFD Entry should have at least one value: {count}"),
                ));
            }

            let mut raw_data: Vec<u8>;
            let ifd_entry_size: usize = usize::try_from(count * type_.size_in_bytes()).unwrap();

            /*
             * From TIFF 6.0 Specification, page 15
             *
             * Value/Offset
             *
             * To save time and space the Value Offset contains the Value instead of pointing to
             * the Value if and only if the Value fits into 4 bytes. If the Value is shorter than 4
             * bytes, it is left-justified within the 4-byte Value Offset, i.e., stored in the
             * lower-numbered bytes. Whether the Value fits within 4 bytes is determined by the
             * Type and Count of the field.
             */
            if ifd_entry_size > 4 {
                /*
                 * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the
                 * field.
                 *
                 * Note: under the hood, tiff_reader converts offset from u32 to u64, which is the
                 *       type std::io::BufReader expects.
                 */
                let offset: Offset = self.read_offset()?;
                let current_offset: Offset = self.reader.stream_position()?;
                self.reader.seek(SeekFrom::Start(offset))?;

                /*
                 * See https://rust-lang.github.io/rust-clippy/master/index.html#uninit_vec
                 * See https://doc.rust-lang.org/std/vec/struct.Vec.html#method.spare_capacity_mut
                 *
                 * This is a Rust hack, but it is OK, because we do not read data, we just want to
                 * make sure the vector has the right size, so we can read stuff into it.
                 */
                raw_data = Vec::with_capacity(ifd_entry_size);
                raw_data.spare_capacity_mut();
                unsafe {
                    raw_data.set_len(ifd_entry_size);
                }

                let bytes_read: usize = self.reader.read(&mut raw_data[..])?;
                if bytes_read != ifd_entry_size {
                    return Err(Error::new(
                        UnexpectedEof,
                        format!("Tried to read {ifd_entry_size} bytes, found only {bytes_read} bytes available"),
                    ));
                }

                self.reader.seek(SeekFrom::Start(current_offset))?;
            } else {
                /*
                 * See https://rust-lang.github.io/rust-clippy/master/index.html#uninit_vec
                 * See https://doc.rust-lang.org/std/vec/struct.Vec.html#method.spare_capacity_mut
                 *
                 * This is a Rust hack, but it is OK, because we do not read data, we just want to
                 * make sure the vector has the right size, so we can read stuff into it.
                 */
                raw_data = Vec::with_capacity(ifd_entry_size);
                raw_data.spare_capacity_mut();
                unsafe {
                    raw_data.set_len(ifd_entry_size);
                }

                let bytes_read: usize = self.reader.read(&mut raw_data[..])?;
                if bytes_read != ifd_entry_size {
                    return Err(Error::new(
                        UnexpectedEof,
                        format!("Tried to read {ifd_entry_size} bytes, found only {bytes_read} bytes available"),
                    ));
                }
                self.reader
                    .seek(SeekFrom::Current((4 - ifd_entry_size).try_into().unwrap()))?;
            }

            let entry: IfdEntry = IfdEntry {
                tag,
                type_,
                count,
                raw_data: &raw_data,
            };
            println!("Tag: {:?}", entry.tag);
            println!("\tType: {:?}", entry.type_);
            println!("\tNumber of values: {}", entry.count);
            println!("\tValue: {:?}", entry.raw_data);
        }

        let offset: Offset = self.read_offset()?;

        Ok(offset)
    }

    /*
     * From TIFF 6.0 Specification, page 13
     *
     * The directory may be at any location in the file after the header but must begin on
     * a word boundary.
     */
    fn read_offset(&mut self) -> Result<Offset, Error> {
        let offset: Offset = Offset::from(self.read_u32()?);
        if offset % 2 == 1 {
            return Err(Error::new(
                InvalidData,
                format!(
                    "Value offset is odd and therefore not a word boundary: {}",
                    offset
                ),
            ));
        }
        Ok(offset)
    }

    fn read_tag(&mut self) -> Result<Tag, Error> {
        Ok(Tag::new(self.read_u16()?))
    }

    fn read_type(&mut self) -> Result<Type, Error> {
        Ok(Type::new(self.read_u16()?))
    }

    fn read_i16(&mut self) -> Result<i16, Error> {
        let buffer: [u8; 2] = self.read()?;
        Ok(match self.endianness {
            LittleEndian => (i16::from(buffer[1]) << 8) + i16::from(buffer[0]),
            BigEndian => (i16::from(buffer[0]) << 8) + i16::from(buffer[1]),
        })
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let buffer: [u8; 2] = self.read()?;
        Ok(match self.endianness {
            LittleEndian => (u16::from(buffer[1]) << 8) + u16::from(buffer[0]),
            BigEndian => (u16::from(buffer[0]) << 8) + u16::from(buffer[1]),
        })
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        let buffer: [u8; 4] = self.read()?;
        Ok(match self.endianness {
            LittleEndian => {
                (u32::from(buffer[3]) << 24)
                    + (u32::from(buffer[2]) << 16)
                    + (u32::from(buffer[1]) << 8)
                    + u32::from(buffer[0])
            }
            BigEndian => {
                (u32::from(buffer[0]) << 24)
                    + (u32::from(buffer[1]) << 16)
                    + (u32::from(buffer[2]) << 8)
                    + u32::from(buffer[3])
            }
        })
    }

    fn read<const SIZE: usize>(&mut self) -> Result<[u8; SIZE], Error> {
        let mut buffer: [u8; SIZE] = [0u8; SIZE];
        let bytes_read: usize = self.reader.read(&mut buffer)?;
        if bytes_read != SIZE {
            return Err(Error::new(
                UnexpectedEof,
                format!("Tried to read {SIZE} bytes, found only {bytes_read} bytes available"),
            ));
        }
        Ok(buffer)
    }
}

#[derive(Debug)]
enum Endianness {
    BigEndian,
    LittleEndian,
}
