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

use data::type_::Type;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData};
use tiff_reader::{Dng, Field, TiffReader};

fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut tiff_reader: TiffReader<BufReader<File>> =
            TiffReader::new(BufReader::new(File::open(file_name)?))?;
        let dng: Dng = tiff_reader.read_dng()?;
        for key in dng.ifd0.fields.keys() {
            if let Some(field) = dng.ifd0.fields.get(key) {
                println!("ifd0.{key:?}");
                print_field(field);
            }
        }
        for key in dng.hires_ifd.fields.keys() {
            if let Some(field) = dng.hires_ifd.fields.get(key) {
                println!("hires_ifd.{key:?}");
                print_field(field);
            }
        }
    } else {
        return Err(Error::new(InvalidData, "Please specify a file"));
    }
    Ok(())
}

pub fn print_field(field: &Field) {
    println!("\tType: {:?}", field.type_);
    println!("\tNumber of values: {}", field.count);
    println!("\tValue: {:?}", field.raw_data);
    match field.type_ {
        Type::Byte(_size) => tiff_reader::to_byte(field),
        Type::Ascii(_size) => tiff_reader::to_ascii(field),
        Type::Short(_size) => tiff_reader::to_short(field),
        Type::Long(_size) => tiff_reader::to_long(field),
        Type::Rational(_size) => tiff_reader::to_rational(field),
        Type::Sbyte(_size) => tiff_reader::to_sbyte(field),
        Type::Undefined(_size) => tiff_reader::to_byte(field),
        Type::Sshort(_size) => tiff_reader::to_sshort(field),
        Type::Slong(_size) => tiff_reader::to_slong(field),
        Type::Srational(_size) => tiff_reader::to_srational(field),
        Type::Float(_size) => tiff_reader::to_float(field),
        Type::Double(_size) => tiff_reader::to_double(field),
        _ => (),
    }
}
