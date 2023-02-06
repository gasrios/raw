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

use data::{Tag, Type};
use std::collections::HashMap;
use std::io::{Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read, Seek, SeekFrom};
use Endianness::{BigEndian, LittleEndian};

pub type Offset = u64;

pub struct Ifd {
    pub fields: HashMap<Tag, Field>,
    pub offset: Offset,
}

pub struct Field {
    pub type_: Type,
    pub count: u32,
    pub raw_data: Vec<u8>,
}

enum Endianness {
    BigEndian,
    LittleEndian,
}

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
    pub fn read(&mut self) -> Result<Vec<Ifd>, Error> {
        let mut offset: Offset = self.process_header()?;
        let mut ifds: Vec<Ifd> = Vec::<Ifd>::new();
        loop {
            let ifd = self.process_ifd(offset)?;

            /*
             * From TIFF 6.0 Specification, page 14
             *
             * An Image File Directory (IFD) consists of (...) followed by a 4-byte offset of
             * the next IFD (or 0 if none).
             */
            offset = ifd.offset;
            ifds.push(ifd);
            if offset == 0 {
                break;
            }
        }
        Ok(ifds)
    }

    fn process_header(&mut self) -> Result<Offset, Error> {
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
        let buffer: [u8; 2] = self.read_to_stack()?;
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

    fn process_ifd(&mut self, offset: Offset) -> Result<Ifd, Error> {
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

        let mut fields: HashMap<Tag, Field> = HashMap::<Tag, Field>::new();
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
             * TODO we do not need to know or process all tags, remove the ones we don't care about
             * uncomment this after testing is done.
            if tag == Tag::Unknown {
                break;
            }
             */

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
                    format!("Invalid field type: {type_:?}",),
                ));
            }

            /*
             * Bytes 4-7 The number of values, Count of the indicated Type.
             */
            let count: u32 = self.read_u32()?;

            if count < 1 {
                return Err(Error::new(
                    InvalidData,
                    format!("Field should have at least one value: {count}"),
                ));
            }

            let raw_data: Vec<u8>;
            let size: usize = usize::try_from(count * type_.size()).unwrap();

            /*
             * Bytes 8-11 The Value Offset, the file offset (in bytes) of the Value for the
             * field.
             *
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
            if size > 4 {
                let offset: Offset = self.read_offset()?;
                let current_offset: Offset = self.reader.stream_position()?;
                self.reader.seek(SeekFrom::Start(offset))?;
                raw_data = self.read_to_heap(size)?;
                self.reader.seek(SeekFrom::Start(current_offset))?;
            } else {
                raw_data = self.read_to_heap(size)?;
                self.reader
                    .seek(SeekFrom::Current((4 - size).try_into().unwrap()))?;
            }

            let field: Field = Field {
                type_,
                count,
                raw_data,
            };
            fields.insert(tag, field);
        }

        Ok(Ifd {
            fields,
            offset: self.read_offset()?,
        })
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
        let buffer: [u8; 2] = self.read_to_stack()?;
        Ok(match self.endianness {
            LittleEndian => (i16::from(buffer[1]) << 8) + i16::from(buffer[0]),
            BigEndian => (i16::from(buffer[0]) << 8) + i16::from(buffer[1]),
        })
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let buffer: [u8; 2] = self.read_to_stack()?;
        Ok(match self.endianness {
            LittleEndian => (u16::from(buffer[1]) << 8) + u16::from(buffer[0]),
            BigEndian => (u16::from(buffer[0]) << 8) + u16::from(buffer[1]),
        })
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        let buffer: [u8; 4] = self.read_to_stack()?;
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

    /*
     * This may be overoptimizing, but I already had a function to read fixed size arrays before I
     * realized I would also need one to read vectors. Or I might trust std::io::BufReader and only
     * do fixed size reading.
     *
     * I'm keeping both for the time being, but may remove one in case it looks like it became
     * redundant and offers no benefit.
     */
    fn read_to_stack<const SIZE: usize>(&mut self) -> Result<[u8; SIZE], Error> {
        let mut buffer: [u8; SIZE] = [0u8; SIZE];
        self.read_to(&mut buffer)?;
        Ok(buffer)
    }

    fn read_to_heap(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        /*
         * See https://rust-lang.github.io/rust-clippy/master/index.html#uninit_vec
         * See https://doc.rust-lang.org/std/vec/struct.Vec.html#method.spare_capacity_mut
         *
         * This is a Rust hack, but it is OK, because we do not read data, we just want to
         * make sure the vector has the right size, so we can read stuff into it.
         */
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        buffer.spare_capacity_mut();
        unsafe {
            buffer.set_len(size);
        }
        self.read_to(&mut buffer)?;
        Ok(buffer)
    }

    fn read_to(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let bytes_read: usize = self.reader.read(buffer)?;
        if bytes_read != buffer.len() {
            return Err(Error::new(
                UnexpectedEof,
                format!(
                    "Tried to read {} bytes, found only {bytes_read} bytes available",
                    buffer.len()
                ),
            ));
        }
        Ok(())
    }
}
