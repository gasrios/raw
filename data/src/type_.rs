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

/*
 * 𝐅𝐫𝐨𝐦 𝐓𝐈𝐅𝐅 𝟔.𝟎 𝐒𝐩𝐞𝐜𝐢𝐟𝐢𝐜𝐚𝐭𝐢𝐨𝐧, 𝐩𝐚𝐠𝐞 𝟏𝟒
 *
 * 𝑇𝑦𝑝𝑒𝑠
 *
 * 𝑇ℎ𝑒 𝑓𝑖𝑒𝑙𝑑 𝑡𝑦𝑝𝑒𝑠 𝑎𝑛𝑑 𝑡ℎ𝑒𝑖𝑟 𝑠𝑖𝑧𝑒𝑠 𝑎𝑟𝑒:
 *  1 = 𝐵𝑌𝑇𝐸 8-𝑏𝑖𝑡 𝑢𝑛𝑠𝑖𝑔𝑛𝑒𝑑 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 *  2 = 𝐴𝑆𝐶𝐼𝐼 8-𝑏𝑖𝑡 𝑏𝑦𝑡𝑒 𝑡ℎ𝑎𝑡 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑠 𝑎 7-𝑏𝑖𝑡 𝐴𝑆𝐶𝐼𝐼 𝑐𝑜𝑑𝑒; 𝑡ℎ𝑒 𝑙𝑎𝑠𝑡 𝑏𝑦𝑡𝑒 𝑚𝑢𝑠𝑡 𝑏𝑒 𝑁𝑈𝐿 (𝑏𝑖𝑛𝑎𝑟𝑦 𝑧𝑒𝑟𝑜).
 *  3 = 𝑆𝐻𝑂𝑅𝑇 16-𝑏𝑖𝑡 (2-𝑏𝑦𝑡𝑒) 𝑢𝑛𝑠𝑖𝑔𝑛𝑒𝑑 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 *  4 = 𝐿𝑂𝑁𝐺 32-𝑏𝑖𝑡 (4-𝑏𝑦𝑡𝑒) 𝑢𝑛𝑠𝑖𝑔𝑛𝑒𝑑 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 *  5 = 𝑅𝐴𝑇𝐼𝑂𝑁𝐴𝐿 𝑇𝑤𝑜 𝐿𝑂𝑁𝐺𝑠: 𝑡ℎ𝑒 𝑓𝑖𝑟𝑠𝑡 𝑟𝑒𝑝𝑟𝑒𝑠𝑒𝑛𝑡𝑠 𝑡ℎ𝑒 𝑛𝑢𝑚𝑒𝑟𝑎𝑡𝑜𝑟 𝑜𝑓 𝑎 𝑓𝑟𝑎𝑐𝑡𝑖𝑜𝑛; 𝑡ℎ𝑒 𝑠𝑒𝑐𝑜𝑛𝑑, 𝑡ℎ𝑒
 *      𝑑𝑒𝑛𝑜𝑚𝑖𝑛𝑎𝑡𝑜𝑟.
 *  6 = 𝑆𝐵𝑌𝑇𝐸 𝐴𝑛 8-𝑏𝑖𝑡 𝑠𝑖𝑔𝑛𝑒𝑑 (𝑡𝑤𝑜𝑠-𝑐𝑜𝑚𝑝𝑙𝑒𝑚𝑒𝑛𝑡) 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 *  7 = 𝑈𝑁𝐷𝐸𝐹𝐼𝑁𝐸𝐷 𝐴𝑛 8-𝑏𝑖𝑡 𝑏𝑦𝑡𝑒 𝑡ℎ𝑎𝑡 𝑚𝑎𝑦 𝑐𝑜𝑛𝑡𝑎𝑖𝑛 𝑎𝑛𝑦𝑡ℎ𝑖𝑛𝑔, 𝑑𝑒𝑝𝑒𝑛𝑑𝑖𝑛𝑔 𝑜𝑛 𝑡ℎ𝑒 𝑑𝑒𝑓𝑖𝑛𝑖𝑡𝑖𝑜𝑛 𝑜𝑓 𝑡ℎ𝑒
 *      𝑓𝑖𝑒𝑙𝑑.
 *  8 = 𝑆𝑆𝐻𝑂𝑅𝑇 𝐴 16-𝑏𝑖𝑡 (2-𝑏𝑦𝑡𝑒) 𝑠𝑖𝑔𝑛𝑒𝑑 (𝑡𝑤𝑜𝑠-𝑐𝑜𝑚𝑝𝑙𝑒𝑚𝑒𝑛𝑡) 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 *  9 = 𝑆𝐿𝑂𝑁𝐺 𝐴 32-𝑏𝑖𝑡 (4-𝑏𝑦𝑡𝑒) 𝑠𝑖𝑔𝑛𝑒𝑑 (𝑡𝑤𝑜𝑠-𝑐𝑜𝑚𝑝𝑙𝑒𝑚𝑒𝑛𝑡) 𝑖𝑛𝑡𝑒𝑔𝑒𝑟.
 * 10 = 𝑆𝑅𝐴𝑇𝐼𝑂𝑁𝐴𝐿 𝑇𝑤𝑜 𝑆𝐿𝑂𝑁𝐺’𝑠: 𝑡ℎ𝑒 𝑓𝑖𝑟𝑠𝑡 𝑟𝑒𝑝𝑟𝑒𝑠𝑒𝑛𝑡𝑠 𝑡ℎ𝑒 𝑛𝑢𝑚𝑒𝑟𝑎𝑡𝑜𝑟 𝑜𝑓 𝑎 𝑓𝑟𝑎𝑐𝑡𝑖𝑜𝑛, 𝑡ℎ𝑒 𝑠𝑒𝑐𝑜𝑛𝑑 𝑡ℎ𝑒
 *      𝑑𝑒𝑛𝑜𝑚𝑖𝑛𝑎𝑡𝑜𝑟.
 * 11 = 𝐹𝐿𝑂𝐴𝑇 𝑆𝑖𝑛𝑔𝑙𝑒 𝑝𝑟𝑒𝑐𝑖𝑠𝑖𝑜𝑛 (4-𝑏𝑦𝑡𝑒) 𝐼𝐸𝐸𝐸 𝑓𝑜𝑟𝑚𝑎𝑡.
 * 12 = 𝐷𝑂𝑈𝐵𝐿𝐸 𝐷𝑜𝑢𝑏𝑙𝑒 𝑝𝑟𝑒𝑐𝑖𝑠𝑖𝑜𝑛 (8-𝑏𝑦𝑡𝑒) 𝐼𝐸𝐸𝐸 𝑓𝑜𝑟𝑚𝑎𝑡.
 *
 * 𝑊𝑎𝑟𝑛𝑖𝑛𝑔: 𝐼𝑡 𝑖𝑠 𝑝𝑜𝑠𝑠𝑖𝑏𝑙𝑒 𝑡ℎ𝑎𝑡 𝑜𝑡ℎ𝑒𝑟 𝑇𝐼𝐹𝐹 𝑓𝑖𝑒𝑙𝑑 𝑡𝑦𝑝𝑒𝑠 𝑤𝑖𝑙𝑙 𝑏𝑒 𝑎𝑑𝑑𝑒𝑑 𝑖𝑛 𝑡ℎ𝑒 𝑓𝑢𝑡𝑢𝑟𝑒. 𝑅𝑒𝑎𝑑𝑒𝑟𝑠 𝑠ℎ𝑜𝑢𝑙𝑑
 *          𝑠𝑘𝑖𝑝 𝑜𝑣𝑒𝑟 𝑓𝑖𝑒𝑙𝑑𝑠 𝑐𝑜𝑛𝑡𝑎𝑖𝑛𝑖𝑛𝑔 𝑎𝑛 𝑢𝑛𝑒𝑥𝑝𝑒𝑐𝑡𝑒𝑑 𝑓𝑖𝑒𝑙𝑑 𝑡𝑦𝑝𝑒.
 */

pub type Byte = u8;
pub type Ascii = char;
pub type Short = u16;
pub type Long = u32;
// TODO pub type Rational = ;
pub type Sbyte = i8;
pub type Undefined = u8;
pub type Sshort = i16;
pub type Slong = i32;
// TODO pub type Srational = ;
pub type Float = f32;
pub type Double = f64;

impl Type {
    #[must_use]
    pub const fn new(type_: u16) -> Type {
        match type_ {
            0 => Type::Unknown,
            1 => Type::Byte(1),
            2 => Type::Ascii(1),
            3 => Type::Short(2),
            4 => Type::Long(4),
            5 => Type::Rational(8),
            6 => Type::Sbyte(1),
            7 => Type::Undefined(1),
            8 => Type::Sshort(2),
            9 => Type::Slong(4),
            10 => Type::Srational(8),
            11 => Type::Float(4),
            12 => Type::Double(8),
            _ => Type::Unexpected,
        }
    }

    /*
     * I really wish I could just write _(size) => *size or access the data as a property of the
     * enum, like type_.size, without this boilerplate code.
     */
    #[must_use]
    pub const fn size(&self) -> u32 {
        match &self {
            Type::Byte(size)
            | Type::Ascii(size)
            | Type::Sbyte(size)
            | Type::Undefined(size)
            | Type::Short(size)
            | Type::Sshort(size)
            | Type::Long(size)
            | Type::Slong(size)
            | Type::Float(size)
            | Type::Rational(size)
            | Type::Srational(size)
            | Type::Double(size) => *size,
            _ => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Unknown,
    Byte(u32),
    Ascii(u32),
    Short(u32),
    Long(u32),
    Rational(u32),
    Sbyte(u32),
    Undefined(u32),
    Sshort(u32),
    Slong(u32),
    Srational(u32),
    Float(u32),
    Double(u32),
    Unexpected,
}
