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

// TODO
// use std::io::{Error, ErrorKind};

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
pub const BYTE: u16 = 1;
pub const ASCII: u16 = 2;
pub const SHORT: u16 = 3;
pub const LONG: u16 = 4;
// TODO pub const RATIONAL: u16 = 5;
pub const SBYTE: u16 = 6;
pub const UNDEFINED: u16 = 7;
pub const SSHORT: u16 = 8;
pub const SLONG: u16 = 9;
// TODO pub const SRATIONAL: u16 = 10;
pub const FLOAT: u16 = 11;
pub const DOUBLE: u16 = 12;

pub type Byte = u8;
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

/// # Errors
///
/// If the number is not a valid type in TIFF
#[must_use]
pub fn type_size(type_: u16) -> usize {
    match type_ {
        ASCII | BYTE | SBYTE | UNDEFINED => 1,
        SHORT | SSHORT => 2,
        FLOAT | LONG | SLONG => 4,
        DOUBLE => 8,
        _ => 0,
    }
}

// TODO
/*pub fn type_size(type_: u16) -> Result<u32, Error> {
    match type_ {
        ASCII | BYTE | SBYTE | UNDEFINED => Ok(1),
        SHORT | SSHORT => Ok(2),
        FLOAT | LONG | SLONG => Ok(4),
        DOUBLE => Ok(8),
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Not a valid type: {type_}")))
    }
}*/

impl Tag {
    #[must_use]
    pub const fn new(tag: u16) -> Tag {
        match tag {
            // Digital Negative Specification, Version 1.4.0.0, page 18
            254 => Tag::NewSubFileType,

            // TIFF 6.0 Specification, page 18
            256 => Tag::ImageWidth,

            // TIFF 6.0 Specification, page 18
            257 => Tag::ImageLength,

            // TIFF 6.0 Specification, page 29
            258 => Tag::BitsPerSample,

            // Digital Negative Specification, Version 1.4.0.0, page 19
            259 => Tag::Compression,

            // Digital Negative Specification, Version 1.4.0.0, page 20
            262 => Tag::PhotometricInterpretation,

            // Digital Negative Specification, Version 1.4.0.0, page 35
            271 => Tag::Make,

            // Digital Negative Specification, Version 1.4.0.0, page 35
            272 => Tag::Model,

            // TIFF 6.0 Specification, page 19
            273 => Tag::StripOffsets,

            /*
             * TIFF 6.0 Specification, page 36
             * TIFF/EP, page 23
             * Digital Negative Specification, Version 1.4.0.0, page 20
             */
            274 => Tag::Orientation,

            // TIFF 6.0 Specification, page 39
            277 => Tag::SamplesPerPixel,

            // TIFF 6.0 Specification, page 19
            278 => Tag::RowsPerStrip,

            // TIFF 6.0 Specification, page 19
            279 => Tag::StripByteCounts,

            // TIFF 6.0 Specification, page 19
            284 => Tag::PlanarConfiguration,

            // TIFF 6.0 Specification, page 39
            305 => Tag::Software,

            // TIFF 6.0 Specification, page 31
            306 => Tag::DateTime,

            // TIFF 6.0 Specification, page 28
            315 => Tag::Artist,

            // TIFF/EP, page 21
            330 => Tag::SubIFDs,

            // Digital Negative Specification, Version 1.4.0.0, page 14
            700 => Tag::XMP,

            33432 => Tag::Copyright,

            // Digital Negative Specification, Version 1.4.0.0, page 14
            34665 => Tag::ExifIFD,

            37393 => Tag::ImageNumber,

            // Digital Negative Specification, Version 1.4.0.0, page 22
            50706 => Tag::DNGVersion,

            // Digital Negative Specification, Version 1.4.0.0, page 22
            50707 => Tag::DNGBackwardVersion,

            // Digital Negative Specification, Version 1.4.0.0, page 23
            50708 => Tag::UniqueCameraModel,

            50709 => Tag::LocalizedCameraModel,

            /*
             * Digital Negative Specification, Version 1.4.0.0,  page 27
             * See chapter 5, “Mapping Raw Values to Linear Reference Values” on page 77 for
             * details of the processing model.
             */
            50714 => Tag::BlackLevel,

            /*
             * Digital Negative Specification, Version 1.4.0.0,  page 29
             * See chapter 5, “Mapping Raw Values to Linear Reference Values” on page 77 for
             * details of the processing model.
             */
            50717 => Tag::WhiteLevel,

            // Digital Negative Specification, Version 1.4.0.0,  page 29
            50718 => Tag::DefaultScale,

            // Digital Negative Specification, Version 1.4.0.0,  page 30
            50719 => Tag::DefaultCropOrigin,

            /*
             * Digital Negative Specification, Version 1.4.0.0,  page 31
             * http://www.barrypearson.co.uk/articles/dng/specification.htm
             */
            50720 => Tag::DefaultCropSize,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 32
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50721 => Tag::ColorMatrix1,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 33
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50722 => Tag::ColorMatrix2,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 34
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50723 => Tag::CameraCalibration1,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 34
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50724 => Tag::CameraCalibration2,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 36
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50727 => Tag::AnalogBalance,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 37
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             */
            50728 => Tag::AsShotNeutral,

            // Digital Negative Specification, Version 1.4.0.0, page 38
            50730 => Tag::BaselineExposure,

            // Digital Negative Specification, Version 1.4.0.0, page 38
            50731 => Tag::BaselineNoise,

            // Digital Negative Specification, Version 1.4.0.0, page 39
            50732 => Tag::BaselineSharpness,

            50733 => Tag::BayerGreenSplit,

            // Digital Negative Specification, Version 1.4.0.0, page 40
            50734 => Tag::LinearResponseLimit,

            50735 => Tag::CameraSerialNumber,

            50736 => Tag::LensInfo,

            // Digital Negative Specification, Version 1.4.0.0, page 42
            50738 => Tag::AntiAliasStrength,

            // Digital Negative Specification, Version 1.4.0.0, page 42
            50739 => Tag::ShadowScale,

            // Digital Negative Specification, Version 1.4.0.0, page 43
            50740 => Tag::DNGPrivateData,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 31
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             *
             * Exif, page 55
             */
            50778 => Tag::CalibrationIlluminant1,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 32
             *
             * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details
             * of the color-processing model.
             *
             * Exif, page 55
             */
            50779 => Tag::CalibrationIlluminant2,

            // Digital Negative Specification, Version 1.4.0.0,  page 30
            50780 => Tag::BestQualityScale,

            // Digital Negative Specification, Version 1.4.0.0, page 45
            50781 => Tag::RawDataUniqueID,

            50827 => Tag::OriginalRawFileName,

            50931 => Tag::CameraCalibrationSignature,

            50932 => Tag::ProfileCalibrationSignature,

            50936 => Tag::ProfileName,

            50941 => Tag::ProfileEmbedPolicy,

            50942 => Tag::ProfileCopyright,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 58
             * Application is described in detail in Chapter 6.
             */
            50964 => Tag::ForwardMatrix1,

            /*
             * Digital Negative Specification, Version 1.4.0.0, page 59
             * Application is described in detail in Chapter 6.
             */
            50965 => Tag::ForwardMatrix2,

            50966 => Tag::PreviewApplicationName,

            50967 => Tag::PreviewApplicationVersion,

            // Digital Negative Specification, Version 1.4.0.0, page 61
            50969 => Tag::PreviewSettingsDigest,

            // Digital Negative Specification, Version 1.4.0.0, page 61
            50970 => Tag::PreviewColorSpace,

            50971 => Tag::PreviewDateTime,

            50972 => Tag::RawImageDigest,

            // Digital Negative Specification, Version 1.4.0.0, page 67
            51041 => Tag::NoiseProfile,

            _ => Tag::Unknown,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    Unknown,
    NewSubFileType,
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    Make,
    Model,
    StripOffsets,
    Orientation,
    SamplesPerPixel,
    RowsPerStrip,
    StripByteCounts,
    PlanarConfiguration,
    Software,
    DateTime,
    Artist,
    SubIFDs,
    XMP,
    Copyright,
    ExifIFD,
    ImageNumber,
    DNGVersion,
    DNGBackwardVersion,
    UniqueCameraModel,
    LocalizedCameraModel,
    BlackLevel,
    WhiteLevel,
    DefaultScale,
    DefaultCropOrigin,
    DefaultCropSize,
    ColorMatrix1,
    ColorMatrix2,
    CameraCalibration1,
    CameraCalibration2,
    AnalogBalance,
    AsShotNeutral,
    BaselineExposure,
    BaselineNoise,
    BaselineSharpness,
    BayerGreenSplit,
    LinearResponseLimit,
    CameraSerialNumber,
    LensInfo,
    AntiAliasStrength,
    ShadowScale,
    DNGPrivateData,
    CalibrationIlluminant1,
    CalibrationIlluminant2,
    BestQualityScale,
    RawDataUniqueID,
    OriginalRawFileName,
    CameraCalibrationSignature,
    ProfileCalibrationSignature,
    ProfileName,
    ProfileEmbedPolicy,
    ProfileCopyright,
    ForwardMatrix1,
    ForwardMatrix2,
    PreviewApplicationName,
    PreviewApplicationVersion,
    PreviewSettingsDigest,
    PreviewColorSpace,
    PreviewDateTime,
    RawImageDigest,
    NoiseProfile,
}
