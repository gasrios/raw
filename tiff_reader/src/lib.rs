/*
 * © 𝟐𝟎𝟐𝟑 𝐆𝐮𝐢𝐥𝐡𝐞𝐫𝐦𝐞 𝐑𝐢𝐨𝐬 𝐀𝐥𝐥 𝐑𝐢𝐠𝐡𝐭𝐬 𝐑𝐞𝐬𝐞𝐫𝐯𝐞𝐝
 *
 * 𝑇ℎ𝑖𝑠 𝑝𝑟𝑜𝑔𝑟𝑎𝑚 𝑖𝑠 𝑓𝑟𝑒𝑒 𝑠𝑜𝑓𝑡𝑤𝑎𝑟𝑒: 𝑦𝑜𝑢 𝑐𝑎𝑛 𝑟𝑒𝑑𝑖𝑠𝑡𝑟𝑖𝑏𝑢𝑡𝑒 𝑖𝑡 𝑎𝑛𝑑/𝑜𝑟 𝑚𝑜𝑑𝑖𝑓𝑦 𝑖𝑡 𝑢𝑛𝑑𝑒𝑟 𝑡ℎ𝑒 𝑡𝑒𝑟𝑚𝑠 𝑜𝑓 𝑡ℎ𝑒
 * 𝐺𝑁𝑈 𝐺𝑒𝑛𝑒𝑟𝑎𝑙 𝑃𝑢𝑏𝑙𝑖𝑐 𝐿𝑖𝑐𝑒𝑛𝑠𝑒 𝑎𝑠 𝑝𝑢𝑏𝑙𝑖𝑠ℎ𝑒𝑑 𝑏𝑦 𝑡ℎ𝑒 𝐹𝑟𝑒𝑒 𝑆𝑜𝑓𝑡𝑤𝑎𝑟𝑒 𝐹𝑜𝑢𝑛𝑑𝑎𝑡𝑖𝑜𝑛, 𝑣𝑒𝑟𝑠𝑖𝑜𝑛 3 𝑜𝑓 𝑡ℎ𝑒
 * 𝐿𝑖𝑐𝑒𝑛𝑠𝑒.
 *
 * 𝑇ℎ𝑖𝑠 𝑝𝑟𝑜𝑔𝑟𝑎𝑚 𝑖𝑠 𝑑𝑖𝑠𝑡𝑟𝑖𝑏𝑢𝑡𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 ℎ𝑜𝑝𝑒 𝑡ℎ𝑎𝑡 𝑖𝑡 𝑤𝑖𝑙𝑙 𝑏𝑒 𝑢𝑠𝑒𝑓𝑢𝑙, 𝑏𝑢𝑡 𝑊𝐼𝑇𝐻𝑂𝑈𝑇 𝐴𝑁𝑌 𝑊𝐴𝑅𝑅𝐴𝑁𝑇𝑌;
 * 𝑤𝑖𝑡ℎ𝑜𝑢𝑡 𝑒𝑣𝑒𝑛 𝑡ℎ𝑒 𝑖𝑚𝑝𝑙𝑖𝑒𝑑 𝑤𝑎𝑟𝑟𝑎𝑛𝑡𝑦 𝑜𝑓 𝑀𝐸𝑅𝐶𝐻𝐴𝑁𝑇𝐴𝐵𝐼𝐿𝐼𝑇𝑌 𝑜𝑟 𝐹𝐼𝑇𝑁𝐸𝑆𝑆 𝐹𝑂𝑅 𝐴 𝑃𝐴𝑅𝑇𝐼𝐶𝑈𝐿𝐴𝑅 𝑃𝑈𝑅𝑃𝑂𝑆𝐸. 𝑆𝑒𝑒
 * 𝑡ℎ𝑒 𝐺𝑁𝑈 𝐺𝑒𝑛𝑒𝑟𝑎𝑙 𝑃𝑢𝑏𝑙𝑖𝑐 𝐿𝑖𝑐𝑒𝑛𝑠𝑒 𝑓𝑜𝑟 𝑚𝑜𝑟𝑒 𝑑𝑒𝑡𝑎𝑖𝑙𝑠.
 *
 * 𝑌𝑜𝑢 𝑠ℎ𝑜𝑢𝑙𝑑 ℎ𝑎𝑣𝑒 𝑟𝑒𝑐𝑒𝑖𝑣𝑒𝑑 𝑎 𝑐𝑜𝑝𝑦 𝑜𝑓 𝑡ℎ𝑒 𝐺𝑁𝑈 𝐺𝑒𝑛𝑒𝑟𝑎𝑙 𝑃𝑢𝑏𝑙𝑖𝑐 𝐿𝑖𝑐𝑒𝑛𝑠𝑒 𝑎𝑙𝑜𝑛𝑔 𝑤𝑖𝑡ℎ 𝑡ℎ𝑖𝑠 𝑝𝑟𝑜𝑔𝑟𝑎𝑚. 𝐼𝑓
 * 𝑛𝑜𝑡, 𝑠𝑒𝑒 ℎ𝑡𝑡𝑝://𝑤𝑤𝑤.𝑔𝑛𝑢.𝑜𝑟𝑔/𝑙𝑖𝑐𝑒𝑛𝑠𝑒𝑠/.
 */

use data::tag::Tag;
use data::type_::{Byte, Type};
use std::collections::HashMap;
use std::io::{Error, ErrorKind::InvalidData, ErrorKind::UnexpectedEof, Read, Seek, SeekFrom};
use Endianness::{BigEndian, LittleEndian};

pub type Offset = u64;

/*
 * 𝐅𝐫𝐨𝐦 𝐃𝐢𝐠𝐢𝐭𝐚𝐥 𝐍𝐞𝐠𝐚𝐭𝐢𝐯𝐞 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐕𝐞𝐫𝐬𝐢𝐨𝐧 𝟏.𝟒.𝟎.𝟎, 𝐩𝐚𝐠𝐞 𝟏𝟑
 *
 * 𝐷𝑁𝐺 𝑟𝑒𝑐𝑜𝑚𝑚𝑒𝑛𝑑𝑠 𝑡ℎ𝑒 𝑢𝑠𝑒 𝑜𝑓 𝑆𝑢𝑏𝐼𝐹𝐷 𝑡𝑟𝑒𝑒𝑠, 𝑎𝑠 𝑑𝑒𝑠𝑐𝑟𝑖𝑏𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 𝑇𝐼𝐹𝐹-𝐸𝑃 𝑠𝑝𝑒𝑐𝑖𝑓𝑖𝑐𝑎𝑡𝑖𝑜𝑛. 𝑆𝑢𝑏𝐼𝐹𝐷 𝑐ℎ𝑎𝑖𝑛𝑠
 * 𝑎𝑟𝑒 𝑛𝑜𝑡 𝑠𝑢𝑝𝑝𝑜𝑟𝑡𝑒𝑑.
 *
 * 𝑇ℎ𝑒 ℎ𝑖𝑔ℎ𝑒𝑠𝑡-𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛 𝑎𝑛𝑑 𝑞𝑢𝑎𝑙𝑖𝑡𝑦 𝐼𝐹𝐷 𝑠ℎ𝑜𝑢𝑙𝑑 𝑢𝑠𝑒 𝑁𝑒𝑤𝑆𝑢𝑏𝐹𝑖𝑙𝑒𝑇𝑦𝑝𝑒 𝑒𝑞𝑢𝑎𝑙 𝑡𝑜 0. 𝑅𝑒𝑑𝑢𝑐𝑒𝑑 𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛
 * (𝑜𝑟 𝑞𝑢𝑎𝑙𝑖𝑡𝑦) 𝑡ℎ𝑢𝑚𝑏𝑛𝑎𝑖𝑙𝑠 𝑜𝑟 𝑝𝑟𝑒𝑣𝑖𝑒𝑤𝑠, 𝑖𝑓 𝑎𝑛𝑦, 𝑠ℎ𝑜𝑢𝑙𝑑 𝑢𝑠𝑒 𝑁𝑒𝑤𝑆𝑢𝑏𝐹𝑖𝑙𝑒𝑇𝑦𝑝𝑒 𝑒𝑞𝑢𝑎𝑙 𝑡𝑜 1 (𝑓𝑜𝑟 𝑎 𝑝𝑟𝑖𝑚𝑎𝑟𝑦
 * 𝑝𝑟𝑒𝑣𝑖𝑒𝑤) 𝑜𝑟 10001.𝐻 (𝑓𝑜𝑟 𝑎𝑛 𝑎𝑙𝑡𝑒𝑟𝑛𝑎𝑡𝑒 𝑝𝑟𝑒𝑣𝑖𝑒𝑤).
 *
 * 𝐷𝑁𝐺 𝑟𝑒𝑐𝑜𝑚𝑚𝑒𝑛𝑑𝑠, 𝑏𝑢𝑡 𝑑𝑜𝑒𝑠 𝑛𝑜𝑡 𝑟𝑒𝑞𝑢𝑖𝑟𝑒, 𝑡ℎ𝑎𝑡 𝑡ℎ𝑒 𝑓𝑖𝑟𝑠𝑡 𝐼𝐹𝐷 𝑐𝑜𝑛𝑡𝑎𝑖𝑛 𝑎 𝑙𝑜𝑤-𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛 𝑡ℎ𝑢𝑚𝑏𝑛𝑎𝑖𝑙, 𝑎𝑠
 * 𝑑𝑒𝑠𝑐𝑟𝑖𝑏𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 𝑇𝐼𝐹𝐹-𝐸𝑃 𝑠𝑝𝑒𝑐𝑖𝑓𝑖𝑐𝑎𝑡𝑖𝑜𝑛.
 *
 * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅/𝐄𝐏, 𝐩𝐚𝐠𝐞 𝟏𝟏
 *
 * 𝐴 𝑆𝑢𝑏𝐼𝐹𝐷𝑠 𝑡𝑎𝑔 𝑖𝑛 𝑡ℎ𝑒 0𝑡ℎ 𝐼𝐹𝐷 𝑖𝑠 𝑢𝑠𝑒𝑑 𝑡𝑜 𝑝𝑜𝑖𝑛𝑡 𝑡𝑜 𝑡ℎ𝑒 𝑐𝑜𝑚𝑝𝑟𝑒𝑠𝑠𝑒𝑑 𝑓𝑢𝑙𝑙-𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛 𝑖𝑚𝑎𝑔𝑒. 𝐼𝑓 𝑡ℎ𝑒
 * 𝑓𝑢𝑙𝑙-𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛 𝑖𝑚𝑎𝑔𝑒 𝑖𝑠 𝑠𝑡𝑜𝑟𝑒𝑑 𝑢𝑛𝑐𝑜𝑚𝑝𝑟𝑒𝑠𝑠𝑒𝑑 𝑎𝑠 𝑎 𝑏𝑎𝑠𝑒𝑙𝑖𝑛𝑒-𝑟𝑒𝑎𝑑𝑎𝑏𝑙𝑒 𝑇𝐼𝐹𝐹 𝑖𝑚𝑎𝑔𝑒, 𝑡ℎ𝑒 𝑓𝑢𝑙𝑙-
 * 𝑟𝑒𝑠𝑜𝑙𝑢𝑡𝑖𝑜𝑛 𝑖𝑚𝑎𝑔𝑒 𝑐𝑜𝑢𝑙𝑑 𝑏𝑒 𝑠𝑡𝑜𝑟𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 0𝑡ℎ 𝐼𝐹𝐷. 𝐻𝑜𝑤𝑒𝑣𝑒𝑟, 𝑇𝐼𝐹𝐹/𝐸𝑃 𝑟𝑒𝑐𝑜𝑚𝑚𝑒𝑛𝑑𝑠 𝑡ℎ𝑎𝑡 𝑎 𝑡ℎ𝑢𝑚𝑏𝑛𝑎𝑖𝑙
 * 𝑖𝑚𝑎𝑔𝑒 𝑏𝑒 𝑠𝑡𝑜𝑟𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 0𝑡ℎ 𝐼𝐹𝐷.
 */
pub struct Dng {
    pub ifd0: Ifd,
    pub hires_ifd: Ifd,
}

pub struct Ifd {
    pub fields: HashMap<Tag, Field>,
    pub offset: Offset,
}

pub struct Field {
    pub type_: Type,
    pub count: u32,
    pub endianness: Endianness,
    pub raw_data: Vec<u8>,
}

pub struct TiffReader<R> {
    reader: R,
    endianness: Endianness,
}

#[derive(Clone, Copy)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

impl<R: Read + Seek> TiffReader<R> {
    /// # Errors
    ///
    /// Only those caused by the underlying reader
    pub fn new(reader: R) -> Result<TiffReader<R>, Error> {
        Ok(TiffReader {
            reader,
            // It doesn't matter which value we initialize endianness with, process_header will set
            // it right later.
            endianness: BigEndian,
        })
    }

    /// # Panics
    ///
    /// Only when underlying reader panics
    ///
    /// # Errors
    ///
    /// Only those caused by the underlying reader, plus nonconformance to DNG 1.4.0.0
    pub fn read_dng(&mut self) -> Result<Dng, Error> {
        let offset: Offset = self.process_header()?;

        let ifd0 = self.process_ifd(offset)?;

        /*
         * 𝐅𝐫𝐨𝐦 𝐃𝐢𝐠𝐢𝐭𝐚𝐥 𝐍𝐞𝐠𝐚𝐭𝐢𝐯𝐞 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐕𝐞𝐫𝐬𝐢𝐨𝐧 𝟏.𝟒.𝟎.𝟎, 𝐩𝐚𝐠𝐞 𝟏𝟑
         *
         * 𝐷𝑁𝐺 𝑟𝑒𝑐𝑜𝑚𝑚𝑒𝑛𝑑𝑠 𝑡ℎ𝑒 𝑢𝑠𝑒 𝑜𝑓 𝑆𝑢𝑏𝐼𝐹𝐷 𝑡𝑟𝑒𝑒𝑠, 𝑎𝑠 𝑑𝑒𝑠𝑐𝑟𝑖𝑏𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 𝑇𝐼𝐹𝐹-𝐸𝑃 𝑠𝑝𝑒𝑐𝑖𝑓𝑖𝑐𝑎𝑡𝑖𝑜𝑛.
         * 𝑆𝑢𝑏𝐼𝐹𝐷 𝑐ℎ𝑎𝑖𝑛𝑠 𝑎𝑟𝑒 𝑛𝑜𝑡 𝑠𝑢𝑝𝑝𝑜𝑟𝑡𝑒𝑑.
         */
        if ifd0.offset != 0 {
            return Err(Error::new(
                InvalidData,
                "DNG recommends the use of SubIFD trees, as described in the TIFF-EP specification. SubIFD chains are not supported.",
            ));
        }

        if let Some(field) = ifd0.fields.get(&Tag::SubIFDs) {
            if usize::try_from(field.count * field.type_.size()).unwrap() > 4 {
                /*
                 * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅/𝐄𝐏, 𝐩𝐚𝐠𝐞 𝟏𝟐
                 *
                 * 𝑇ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑤𝑖𝑙𝑙 𝑐𝑜𝑛𝑡𝑎𝑖𝑛 𝑡ℎ𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 𝑡𝑜 𝑡ℎ𝑒 “𝑡𝑟𝑒𝑒𝑑” 𝐼𝐹𝐷 𝑖𝑡𝑠𝑒𝑙𝑓 𝑖𝑓 𝑁=1, 𝑜𝑡ℎ𝑒𝑟𝑤𝑖𝑠𝑒 𝑡ℎ𝑒
                 * 𝑉𝑎𝑙𝑢𝑒 𝑤𝑖𝑙𝑙 𝑐𝑜𝑛𝑡𝑎𝑖𝑛 𝑎𝑛 𝑜𝑓𝑓𝑠𝑒𝑡 𝑡𝑜 𝑎 𝑙𝑜𝑐𝑎𝑡𝑖𝑜𝑛 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑖𝑛𝑔 𝑎𝑛 𝑎𝑟𝑟𝑎𝑦 𝑜𝑓 𝑜𝑓𝑓𝑠𝑒𝑡𝑠 𝑡𝑜 𝑒𝑎𝑐ℎ
                 * 𝐼𝐹𝐷 𝑏𝑒𝑖𝑛𝑔 “𝑡𝑟𝑒𝑒𝑑” 𝑓𝑟𝑜𝑚 𝑡ℎ𝑒 𝑐𝑢𝑟𝑟𝑒𝑛𝑡 𝐼𝐹𝐷. 𝑇ℎ𝑖𝑠 𝑎𝑟𝑟𝑎𝑦 𝑜𝑓 𝑜𝑓𝑓𝑠𝑒𝑡𝑠 𝑤𝑖𝑙𝑙 𝑐𝑜𝑛𝑡𝑎𝑖𝑛 𝑁
                 * 𝑒𝑛𝑡𝑟𝑖𝑒𝑠, 𝑖.𝑒. 𝑜𝑓𝑓𝑠𝑒𝑡 𝑝𝑜𝑖𝑛𝑡𝑒𝑟𝑠 𝑡𝑜 𝑁 𝐼𝐹𝐷𝑠. 𝑪𝒖𝒓𝒓𝒆𝒏𝒕𝒍𝒚, 𝑵=1, 𝒂𝒏𝒅 𝒕𝒉𝒆 𝑽𝒂𝒍𝒖𝒆 𝒄𝒐𝒏𝒕𝒂𝒊𝒏𝒔
                 * 𝒕𝒉𝒆 𝒐𝒇𝒇𝒔𝒆𝒕 𝒕𝒐 𝒕𝒉𝒆 𝑰𝑭𝑫 𝒄𝒐𝒏𝒕𝒂𝒊𝒏𝒊𝒏𝒈 𝒕𝒉𝒆 𝒇𝒖𝒍𝒍 𝒓𝒆𝒔𝒐𝒍𝒖𝒕𝒊𝒐𝒏 𝒊𝒎𝒂𝒈𝒆.
                 *
                 * 𝑈𝑠𝑎𝑔𝑒: 𝐼𝐹𝐷0
                 */
                return Err(Error::new(
                    InvalidData,
                    "\"SubIFDs\" should only contain the offset to the IFD containing the full resolution image.",
                ));
            }
            let offset: Offset = self.offset(&field.raw_data[0..4])?;
            Ok(Dng {
                ifd0,
                hires_ifd: self.process_ifd(offset)?,
            })
        } else {
            Err(Error::new(
                InvalidData,
                "TIFF/EP recommends that a thumbnail image be stored in the 0th IFD",
            ))
        }
    }

    /// # Errors
    ///
    /// Only those caused by the underlying reader, plus nonconformance to DNG 1.4.0.0
    pub fn read(&mut self) -> Result<Vec<Ifd>, Error> {
        let mut offset: Offset = self.process_header()?;
        let mut ifds: Vec<Ifd> = Vec::<Ifd>::new();
        loop {
            let ifd = self.process_ifd(offset)?;

            /*
             * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟒
             *
             * 𝐴𝑛 𝐼𝑚𝑎𝑔𝑒 𝐹𝑖𝑙𝑒 𝐷𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦 (𝐼𝐹𝐷) 𝑐𝑜𝑛𝑠𝑖𝑠𝑡𝑠 𝑜𝑓 (...) 𝑓𝑜𝑙𝑙𝑜𝑤𝑒𝑑 𝑏𝑦 𝑎 4-𝑏𝑦𝑡𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 𝑜𝑓 𝑡ℎ𝑒
             * 𝑛𝑒𝑥𝑡 𝐼𝐹𝐷 (𝑜𝑟 0 𝑖𝑓 𝑛𝑜𝑛𝑒).
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
         * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟑
         *
         * 𝐼𝑚𝑎𝑔𝑒 𝐹𝑖𝑙𝑒 𝐻𝑒𝑎𝑑𝑒𝑟
         *
         * 𝐴 𝑇𝐼𝐹𝐹 𝑓𝑖𝑙𝑒 𝑏𝑒𝑔𝑖𝑛𝑠 𝑤𝑖𝑡ℎ 𝑎𝑛 8-𝑏𝑦𝑡𝑒 𝑖𝑚𝑎𝑔𝑒 𝑓𝑖𝑙𝑒 ℎ𝑒𝑎𝑑𝑒𝑟, 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑖𝑛𝑔 𝑡ℎ𝑒 𝑓𝑜𝑙𝑙𝑜𝑤𝑖𝑛𝑔
         * 𝑖𝑛𝑓𝑜𝑟𝑚𝑎𝑡𝑖𝑜𝑛:
         *
         * 𝐵𝑦𝑡𝑒𝑠 0-1: 𝑇ℎ𝑒 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟 𝑢𝑠𝑒𝑑 𝑤𝑖𝑡ℎ𝑖𝑛 𝑡ℎ𝑒 𝑓𝑖𝑙𝑒. 𝐿𝑒𝑔𝑎𝑙 𝑣𝑎𝑙𝑢𝑒𝑠 𝑎𝑟𝑒:
         *            “𝐼𝐼” (4949.𝐻)
         *            “𝑀𝑀” (4𝐷4𝐷.𝐻)
         *
         *            𝐼𝑛 𝑡ℎ𝑒 “𝐼𝐼” 𝑓𝑜𝑟𝑚𝑎𝑡, 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟 𝑖𝑠 𝑎𝑙𝑤𝑎𝑦𝑠 𝑓𝑟𝑜𝑚 𝑡ℎ𝑒 𝑙𝑒𝑎𝑠𝑡 𝑠𝑖𝑔𝑛𝑖𝑓𝑖𝑐𝑎𝑛𝑡 𝑏𝑦𝑡𝑒 𝑡𝑜
         *            𝑡ℎ𝑒 𝑚𝑜𝑠𝑡 𝑠𝑖𝑔𝑛𝑖𝑓𝑖𝑐𝑎𝑛𝑡 𝑏𝑦𝑡𝑒, 𝑓𝑜𝑟 𝑏𝑜𝑡ℎ 16-𝑏𝑖𝑡 𝑎𝑛𝑑 32-𝑏𝑖𝑡 𝑖𝑛𝑡𝑒𝑔𝑒𝑟𝑠 𝑇ℎ𝑖𝑠 𝑖𝑠 𝑐𝑎𝑙𝑙𝑒𝑑
         *            𝑙𝑖𝑡𝑡𝑙𝑒-𝑒𝑛𝑑𝑖𝑎𝑛 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟. 𝐼𝑛 𝑡ℎ𝑒 “𝑀𝑀” 𝑓𝑜𝑟𝑚𝑎𝑡, 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟 𝑖𝑠 𝑎𝑙𝑤𝑎𝑦𝑠 𝑓𝑟𝑜𝑚 𝑚𝑜𝑠𝑡
         *            𝑠𝑖𝑔𝑛𝑖𝑓𝑖𝑐𝑎𝑛𝑡 𝑡𝑜 𝑙𝑒𝑎𝑠𝑡 𝑠𝑖𝑔𝑛𝑖𝑓𝑖𝑐𝑎𝑛𝑡, 𝑓𝑜𝑟 𝑏𝑜𝑡ℎ 16-𝑏𝑖𝑡 𝑎𝑛𝑑 32-𝑏𝑖𝑡 𝑖𝑛𝑡𝑒𝑔𝑒𝑟𝑠. 𝑇ℎ𝑖𝑠
         *            𝑖𝑠 𝑐𝑎𝑙𝑙𝑒𝑑 𝑏𝑖𝑔-𝑒𝑛𝑑𝑖𝑎𝑛 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟.
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
         * 𝐵𝑦𝑡𝑒𝑠 2-3: 𝐴𝑛 𝑎𝑟𝑏𝑖𝑡𝑟𝑎𝑟𝑦 𝑏𝑢𝑡 𝑐𝑎𝑟𝑒𝑓𝑢𝑙𝑙𝑦 𝑐ℎ𝑜𝑠𝑒𝑛 𝑛𝑢𝑚𝑏𝑒𝑟 (42) 𝑡ℎ𝑎𝑡 𝑓𝑢𝑟𝑡ℎ𝑒𝑟 𝑖𝑑𝑒𝑛𝑡𝑖𝑓𝑖𝑒𝑠 𝑡ℎ𝑒
         *            𝑓𝑖𝑙𝑒 𝑎𝑠 𝑎 𝑇𝐼𝐹𝐹 𝑓𝑖𝑙𝑒.
         *
         *            𝑇ℎ𝑒 𝑏𝑦𝑡𝑒 𝑜𝑟𝑑𝑒𝑟 𝑑𝑒𝑝𝑒𝑛𝑑𝑠 𝑜𝑛 𝑡ℎ𝑒 𝑣𝑎𝑙𝑢𝑒 𝑜𝑓 𝐵𝑦𝑡𝑒𝑠 0-1.
         */
        let version: i16 = self.read_i16()?;
        if version != 42 {
            return Err(Error::new(
            InvalidData,
            format!("Failed to further identify the file as a TIFF file, was expecting 42, found {version}"),
        ));
        }

        /*
         * 𝐵𝑦𝑡𝑒𝑠 4-7: 𝑇ℎ𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 (𝑖𝑛 𝑏𝑦𝑡𝑒𝑠) 𝑜𝑓 𝑡ℎ𝑒 𝑓𝑖𝑟𝑠𝑡 𝐼𝐹𝐷. 𝑇ℎ𝑒 𝑑𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦 𝑚𝑎𝑦 𝑏𝑒 𝑎𝑡 𝑎𝑛𝑦 𝑙𝑜𝑐𝑎𝑡𝑖𝑜𝑛
         *            𝑖𝑛 𝑡ℎ𝑒 𝑓𝑖𝑙𝑒 𝑎𝑓𝑡𝑒𝑟 𝑡ℎ𝑒 ℎ𝑒𝑎𝑑𝑒𝑟 𝑏𝑢𝑡 𝑚𝑢𝑠𝑡 𝑏𝑒𝑔𝑖𝑛 𝑜𝑛 𝑎 𝑤𝑜𝑟𝑑 𝑏𝑜𝑢𝑛𝑑𝑎𝑟𝑦. 𝐼𝑛
         *            𝑝𝑎𝑟𝑡𝑖𝑐𝑢𝑙𝑎𝑟, 𝑎𝑛 𝐼𝑚𝑎𝑔𝑒 𝐹𝑖𝑙𝑒 𝐷𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦 𝑚𝑎𝑦 𝑓𝑜𝑙𝑙𝑜𝑤 𝑡ℎ𝑒 𝑖𝑚𝑎𝑔𝑒 𝑑𝑎𝑡𝑎 𝑖𝑡 𝑑𝑒𝑠𝑐𝑟𝑖𝑏𝑒𝑠.
         *            𝑅𝑒𝑎𝑑𝑒𝑟𝑠 𝑚𝑢𝑠𝑡 𝑓𝑜𝑙𝑙𝑜𝑤 𝑡ℎ𝑒 𝑝𝑜𝑖𝑛𝑡𝑒𝑟𝑠 𝑤ℎ𝑒𝑟𝑒𝑣𝑒𝑟 𝑡ℎ𝑒𝑦 𝑚𝑎𝑦 𝑙𝑒𝑎𝑑.
         *
         *            𝑇ℎ𝑒 𝑡𝑒𝑟𝑚 𝑏𝑦𝑡𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 𝑖𝑠 𝑎𝑙𝑤𝑎𝑦𝑠 𝑢𝑠𝑒𝑑 𝑖𝑛 𝑡ℎ𝑖𝑠 𝑑𝑜𝑐𝑢𝑚𝑒𝑛𝑡 𝑡𝑜 𝑟𝑒𝑓𝑒𝑟 𝑡𝑜 𝑎 𝑙𝑜𝑐𝑎𝑡𝑖𝑜𝑛
         *            𝑤𝑖𝑡ℎ 𝑟𝑒𝑠𝑝𝑒𝑐𝑡 𝑡𝑜 𝑡ℎ𝑒 𝑏𝑒𝑔𝑖𝑛𝑛𝑖𝑛𝑔 𝑜𝑓 𝑡ℎ𝑒 𝑇𝐼𝐹𝐹 𝑓𝑖𝑙𝑒. 𝑇ℎ𝑒 𝑓𝑖𝑟𝑠𝑡 𝑏𝑦𝑡𝑒 𝑜𝑓 𝑡ℎ𝑒 𝑓𝑖𝑙𝑒
         *            ℎ𝑎𝑠 𝑎𝑛 𝑜𝑓𝑓𝑠𝑒𝑡 𝑜𝑓 0.
         */
        let offset: Offset = self.read_offset()?;

        /*
         * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟒
         *
         * 𝑇ℎ𝑒𝑟𝑒 𝑚𝑢𝑠𝑡 𝑏𝑒 𝑎𝑡 𝑙𝑒𝑎𝑠𝑡 1 𝐼𝐹𝐷 𝑖𝑛 𝑎 𝑇𝐼𝐹𝐹 𝑓𝑖𝑙𝑒 𝑎𝑛𝑑 𝑒𝑎𝑐ℎ 𝐼𝐹𝐷 𝑚𝑢𝑠𝑡 ℎ𝑎𝑣𝑒 𝑎𝑡 𝑙𝑒𝑎𝑠𝑡 𝑜𝑛𝑒 𝑒𝑛𝑡𝑟𝑦.
         *
         * As a side effect, we also fail here if offset == 0, that is, there are no IFDs in the
         * file.
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
         * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟒
         *
         * 𝐼𝑚𝑎𝑔𝑒 𝐹𝑖𝑙𝑒 𝐷𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦
         *
         * 𝐴𝑛 𝐼𝑚𝑎𝑔𝑒 𝐹𝑖𝑙𝑒 𝐷𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦 (𝐼𝐹𝐷) 𝑐𝑜𝑛𝑠𝑖𝑠𝑡𝑠 𝑜𝑓 𝑎 2-𝑏𝑦𝑡𝑒 𝑐𝑜𝑢𝑛𝑡 𝑜𝑓 𝑡ℎ𝑒 𝑛𝑢𝑚𝑏𝑒𝑟 𝑜𝑓 𝑑𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦
         * 𝑒𝑛𝑡𝑟𝑖𝑒𝑠 (𝑖.𝑒., 𝑡ℎ𝑒 𝑛𝑢𝑚𝑏𝑒𝑟 𝑜𝑓 𝑓𝑖𝑒𝑙𝑑𝑠), 𝑓𝑜𝑙𝑙𝑜𝑤𝑒𝑑 𝑏𝑦 𝑎 𝑠𝑒𝑞𝑢𝑒𝑛𝑐𝑒 𝑜𝑓 12-𝑏𝑦𝑡𝑒 𝑓𝑖𝑒𝑙𝑑 𝑒𝑛𝑡𝑟𝑖𝑒𝑠,
         * 𝑓𝑜𝑙𝑙𝑜𝑤𝑒𝑑 𝑏𝑦 𝑎 4-𝑏𝑦𝑡𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 𝑜𝑓 𝑡ℎ𝑒 𝑛𝑒𝑥𝑡 𝐼𝐹𝐷 (𝑜𝑟 0 𝑖𝑓 𝑛𝑜𝑛𝑒). (𝐷𝑜 𝑛𝑜𝑡 𝑓𝑜𝑟𝑔𝑒𝑡 𝑡𝑜 𝑤𝑟𝑖𝑡𝑒 𝑡ℎ𝑒
         * 4 𝑏𝑦𝑡𝑒𝑠 𝑜𝑓 0 𝑎𝑓𝑡𝑒𝑟 𝑡ℎ𝑒 𝑙𝑎𝑠𝑡 𝐼𝐹𝐷.)
         *
         * 𝑇ℎ𝑒𝑟𝑒 𝑚𝑢𝑠𝑡 𝑏𝑒 𝑎𝑡 𝑙𝑒𝑎𝑠𝑡 1 𝐼𝐹𝐷 𝑖𝑛 𝑎 𝑇𝐼𝐹𝐹 𝑓𝑖𝑙𝑒 𝑎𝑛𝑑 𝑒𝑎𝑐ℎ 𝐼𝐹𝐷 𝑚𝑢𝑠𝑡 ℎ𝑎𝑣𝑒 𝑎𝑡 𝑙𝑒𝑎𝑠𝑡 𝑜𝑛𝑒 𝑒𝑛𝑡𝑟𝑦.
         */
        let number_of_fields: u16 = self.read_u16()?;

        let mut fields: HashMap<Tag, Field> = HashMap::<Tag, Field>::new();
        for _i in 0..number_of_fields {
            /*
             * 𝐼𝐹𝐷 𝐸𝑛𝑡𝑟𝑦
             *
             * 𝐸𝑎𝑐ℎ 12-𝑏𝑦𝑡𝑒 𝐼𝐹𝐷 𝑒𝑛𝑡𝑟𝑦 ℎ𝑎𝑠 𝑡ℎ𝑒 𝑓𝑜𝑙𝑙𝑜𝑤𝑖𝑛𝑔 𝑓𝑜𝑟𝑚𝑎𝑡:
             *
             * 𝐵𝑦𝑡𝑒𝑠 0-1 𝑇ℎ𝑒 𝑇𝑎𝑔 𝑡ℎ𝑎𝑡 𝑖𝑑𝑒𝑛𝑡𝑖𝑓𝑖𝑒𝑠 𝑡ℎ𝑒 𝑓𝑖𝑒𝑙𝑑.
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
             * 𝐵𝑦𝑡𝑒𝑠 2-3 𝑇ℎ𝑒 𝑓𝑖𝑒𝑙𝑑 𝑇𝑦𝑝𝑒.
             */
            let type_: Type = self.read_type()?;

            /*
             * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟒
             *
             * 𝑊𝑎𝑟𝑛𝑖𝑛𝑔: 𝐼𝑡 𝑖𝑠 𝑝𝑜𝑠𝑠𝑖𝑏𝑙𝑒 𝑡ℎ𝑎𝑡 𝑜𝑡ℎ𝑒𝑟 𝑇𝐼𝐹𝐹 𝑓𝑖𝑒𝑙𝑑 𝑡𝑦𝑝𝑒𝑠 𝑤𝑖𝑙𝑙 𝑏𝑒 𝑎𝑑𝑑𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 𝑓𝑢𝑡𝑢𝑟𝑒.
             * 𝑅𝑒𝑎𝑑𝑒𝑟𝑠 𝑠ℎ𝑜𝑢𝑙𝑑 𝑠𝑘𝑖𝑝 𝑜𝑣𝑒𝑟 𝑓𝑖𝑒𝑙𝑑𝑠 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑖𝑛𝑔 𝑎𝑛 𝑢𝑛𝑒𝑥𝑝𝑒𝑐𝑡𝑒𝑑 𝑓𝑖𝑒𝑙𝑑 𝑡𝑦𝑝𝑒.
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
             * 𝐵𝑦𝑡𝑒𝑠 4-7 𝑇ℎ𝑒 𝑛𝑢𝑚𝑏𝑒𝑟 𝑜𝑓 𝑣𝑎𝑙𝑢𝑒𝑠, 𝐶𝑜𝑢𝑛𝑡 𝑜𝑓 𝑡ℎ𝑒 𝑖𝑛𝑑𝑖𝑐𝑎𝑡𝑒𝑑 𝑇𝑦𝑝𝑒.
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
             * 𝐵𝑦𝑡𝑒𝑠 8-11 𝑇ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑂𝑓𝑓𝑠𝑒𝑡, 𝑡ℎ𝑒 𝑓𝑖𝑙𝑒 𝑜𝑓𝑓𝑠𝑒𝑡 (𝑖𝑛 𝑏𝑦𝑡𝑒𝑠) 𝑜𝑓 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑓𝑜𝑟 𝑡ℎ𝑒 𝑓𝑖𝑒𝑙𝑑.
             *
             * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟓
             *
             * 𝑉𝑎𝑙𝑢𝑒/𝑂𝑓𝑓𝑠𝑒𝑡
             *
             * 𝑇𝑜 𝑠𝑎𝑣𝑒 𝑡𝑖𝑚𝑒 𝑎𝑛𝑑 𝑠𝑝𝑎𝑐𝑒 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑂𝑓𝑓𝑠𝑒𝑡 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑠 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑖𝑛𝑠𝑡𝑒𝑎𝑑 𝑜𝑓 𝑝𝑜𝑖𝑛𝑡𝑖𝑛𝑔 𝑡𝑜
             * 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑖𝑓 𝑎𝑛𝑑 𝑜𝑛𝑙𝑦 𝑖𝑓 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑓𝑖𝑡𝑠 𝑖𝑛𝑡𝑜 4 𝑏𝑦𝑡𝑒𝑠. 𝐼𝑓 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑖𝑠 𝑠ℎ𝑜𝑟𝑡𝑒𝑟 𝑡ℎ𝑎𝑛 4
             * 𝑏𝑦𝑡𝑒𝑠, 𝑖𝑡 𝑖𝑠 𝑙𝑒𝑓𝑡-𝑗𝑢𝑠𝑡𝑖𝑓𝑖𝑒𝑑 𝑤𝑖𝑡ℎ𝑖𝑛 𝑡ℎ𝑒 4-𝑏𝑦𝑡𝑒 𝑉𝑎𝑙𝑢𝑒 𝑂𝑓𝑓𝑠𝑒𝑡, 𝑖.𝑒., 𝑠𝑡𝑜𝑟𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒
             * 𝑙𝑜𝑤𝑒𝑟-𝑛𝑢𝑚𝑏𝑒𝑟𝑒𝑑 𝑏𝑦𝑡𝑒𝑠. 𝑊ℎ𝑒𝑡ℎ𝑒𝑟 𝑡ℎ𝑒 𝑉𝑎𝑙𝑢𝑒 𝑓𝑖𝑡𝑠 𝑤𝑖𝑡ℎ𝑖𝑛 4 𝑏𝑦𝑡𝑒𝑠 𝑖𝑠 𝑑𝑒𝑡𝑒𝑟𝑚𝑖𝑛𝑒𝑑 𝑏𝑦 𝑡ℎ𝑒
             * 𝑇𝑦𝑝𝑒 𝑎𝑛𝑑 𝐶𝑜𝑢𝑛𝑡 𝑜𝑓 𝑡ℎ𝑒 𝑓𝑖𝑒𝑙𝑑.
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
                endianness: self.endianness,
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
     * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟑
     *
     * 𝑇ℎ𝑒 𝑑𝑖𝑟𝑒𝑐𝑡𝑜𝑟𝑦 𝑚𝑎𝑦 𝑏𝑒 𝑎𝑡 𝑎𝑛𝑦 𝑙𝑜𝑐𝑎𝑡𝑖𝑜𝑛 𝑖𝑛 𝑡ℎ𝑒 𝑓𝑖𝑙𝑒 𝑎𝑓𝑡𝑒𝑟 𝑡ℎ𝑒 ℎ𝑒𝑎𝑑𝑒𝑟 𝑏𝑢𝑡 𝑚𝑢𝑠𝑡 𝑏𝑒𝑔𝑖𝑛 𝑜𝑛 𝑎 𝑤𝑜𝑟𝑑
     * 𝑏𝑜𝑢𝑛𝑑𝑎𝑟𝑦.
     */
    fn read_offset(&mut self) -> Result<Offset, Error> {
        let offset: Offset = Offset::from(self.read_u32()?);
        if offset % 2 == 1 {
            return Err(Error::new(
                InvalidData,
                format!("Value offset is odd and therefore not a word boundary: {offset}"),
            ));
        }
        Ok(offset)
    }

    fn offset(&mut self, buffer: &[u8]) -> Result<Offset, Error> {
        let offset: Offset = Offset::from(match self.endianness {
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
        });
        if offset % 2 == 1 {
            return Err(Error::new(
                InvalidData,
                format!("Value offset is odd and therefore not a word boundary: {offset}"),
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

pub fn to_byte(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_ascii(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_short(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_long(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_rational(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_sbyte(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_sshort(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_slong(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_srational(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_float(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}

pub fn to_double(field: &Field) {
    let mut buffer: Vec<Byte> = Vec::<Byte>::new();
    for byte in &*field.raw_data {
        buffer.push(*byte);
    }
    match field.endianness {
        LittleEndian => (),
        BigEndian => println!("BigEndian"),
    }
}
