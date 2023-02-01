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

pub type Offset = u64;

pub struct IfdEntry {
    pub tag: Tag,
    pub type_: Type,
    pub count: u32,
    pub offset: Offset,
}

impl IfdEntry {
    #[must_use]
    pub const fn size_in_bytes(&self) -> u32 {
        self.type_.size_in_bytes() * self.count
    }
}

// TODO Tag and Type should be in separate modules and files

impl Tag {
    #[must_use]
    pub const fn new(tag: u16) -> Tag {
        match tag {
            254 => Tag::NewSubFileType,
            256 => Tag::ImageWidth,
            257 => Tag::ImageLength,
            258 => Tag::BitsPerSample,
            259 => Tag::Compression,
            262 => Tag::PhotometricInterpretation,
            271 => Tag::Make,
            272 => Tag::Model,
            273 => Tag::StripOffsets,
            274 => Tag::Orientation,
            277 => Tag::SamplesPerPixel,
            278 => Tag::RowsPerStrip,
            279 => Tag::StripByteCounts,
            284 => Tag::PlanarConfiguration,
            305 => Tag::Software,
            306 => Tag::DateTime,
            315 => Tag::Artist,
            330 => Tag::SubIFDs,
            700 => Tag::XMP,
            33432 => Tag::Copyright,
            34665 => Tag::ExifIFD,
            37393 => Tag::ImageNumber,
            50706 => Tag::DNGVersion,
            50707 => Tag::DNGBackwardVersion,
            50708 => Tag::UniqueCameraModel,
            50709 => Tag::LocalizedCameraModel,
            50721 => Tag::ColorMatrix1,
            50722 => Tag::ColorMatrix2,
            50723 => Tag::CameraCalibration1,
            50724 => Tag::CameraCalibration2,
            50727 => Tag::AnalogBalance,
            50728 => Tag::AsShotNeutral,
            50730 => Tag::BaselineExposure,
            50731 => Tag::BaselineNoise,
            50732 => Tag::BaselineSharpness,
            50733 => Tag::BayerGreenSplit,
            50734 => Tag::LinearResponseLimit,
            50735 => Tag::CameraSerialNumber,
            50736 => Tag::LensInfo,
            50739 => Tag::ShadowScale,
            50740 => Tag::DNGPrivateData,
            50778 => Tag::CalibrationIlluminant1,
            50779 => Tag::CalibrationIlluminant2,
            50781 => Tag::RawDataUniqueID,
            50827 => Tag::OriginalRawFileName,
            50931 => Tag::CameraCalibrationSignature,
            50932 => Tag::ProfileCalibrationSignature,
            50936 => Tag::ProfileName,
            50941 => Tag::ProfileEmbedPolicy,
            50942 => Tag::ProfileCopyright,
            50964 => Tag::ForwardMatrix1,
            50965 => Tag::ForwardMatrix2,
            50966 => Tag::PreviewApplicationName,
            50967 => Tag::PreviewApplicationVersion,
            50969 => Tag::PreviewSettingsDigest,
            50970 => Tag::PreviewColorSpace,
            50971 => Tag::PreviewDateTime,
            50972 => Tag::RawImageDigest,
            51041 => Tag::NoiseProfile,
            _ => Tag::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum Tag {
    Unknown,
    // Digital Negative Specification, Version 1.4.0.0, page 18
    NewSubFileType,
    // TIFF 6.0 Specification, page 18
    ImageWidth,
    // TIFF 6.0 Specification, page 18
    ImageLength,
    // TIFF 6.0 Specification, page 29
    BitsPerSample,
    // Digital Negative Specification, Version 1.4.0.0, page 19
    Compression,
    // Digital Negative Specification, Version 1.4.0.0, page 20
    PhotometricInterpretation,
    // Digital Negative Specification, Version 1.4.0.0, page 35
    Make,
    // Digital Negative Specification, Version 1.4.0.0, page 35
    Model,
    // TIFF 6.0 Specification, page 19
    StripOffsets,
    /*
     * TIFF 6.0 Specification, page 36
     * TIFF/EP, page 23
     * Digital Negative Specification, Version 1.4.0.0, page 20
     */
    Orientation,
    // TIFF 6.0 Specification, page 39
    SamplesPerPixel,
    // TIFF 6.0 Specification, page 19
    RowsPerStrip,
    // TIFF 6.0 Specification, page 19
    StripByteCounts,
    // TIFF 6.0 Specification, page 19
    PlanarConfiguration,
    // TIFF 6.0 Specification, page 39
    Software,
    // TIFF 6.0 Specification, page 31
    DateTime,
    // TIFF 6.0 Specification, page 28
    Artist,
    // TIFF/EP, page 21
    SubIFDs,
    // Digital Negative Specification, Version 1.4.0.0, page 14
    XMP,
    Copyright,
    // Digital Negative Specification, Version 1.4.0.0, page 14
    ExifIFD,
    ImageNumber,
    // Digital Negative Specification, Version 1.4.0.0, page 22
    DNGVersion,
    // Digital Negative Specification, Version 1.4.0.0, page 22
    DNGBackwardVersion,
    // Digital Negative Specification, Version 1.4.0.0, page 23
    UniqueCameraModel,
    LocalizedCameraModel,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 32
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    ColorMatrix1,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 33
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    ColorMatrix2,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 34
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    CameraCalibration1,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 34
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    CameraCalibration2,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 36
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    AnalogBalance,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 37
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     */
    AsShotNeutral,
    // Digital Negative Specification, Version 1.4.0.0, page 38
    BaselineExposure,
    // Digital Negative Specification, Version 1.4.0.0, page 38
    BaselineNoise,
    // Digital Negative Specification, Version 1.4.0.0, page 39
    BaselineSharpness,
    BayerGreenSplit,
    // Digital Negative Specification, Version 1.4.0.0, page 40
    LinearResponseLimit,
    CameraSerialNumber,
    LensInfo,
    // Digital Negative Specification, Version 1.4.0.0, page 42
    ShadowScale,
    // Digital Negative Specification, Version 1.4.0.0, page 43
    DNGPrivateData,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 31
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     * Exif, page 55
     */
    CalibrationIlluminant1,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 32
     * See chapter 6, “Mapping Camera Color Space to CIE XYZ Space” on page 79 for details of the color-processing model.
     * Exif, page 55
     */
    CalibrationIlluminant2,
    // Digital Negative Specification, Version 1.4.0.0, page 45
    RawDataUniqueID,
    OriginalRawFileName,
    CameraCalibrationSignature,
    ProfileCalibrationSignature,
    ProfileName,
    ProfileEmbedPolicy,
    ProfileCopyright,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 58
     * Application is described in detail in Chapter 6.
     */
    ForwardMatrix1,
    /*
     * Digital Negative Specification, Version 1.4.0.0, page 59
     * Application is described in detail in Chapter 6.
     */
    ForwardMatrix2,
    PreviewApplicationName,
    PreviewApplicationVersion,
    // Digital Negative Specification, Version 1.4.0.0, page 61
    PreviewSettingsDigest,
    // Digital Negative Specification, Version 1.4.0.0, page 61
    PreviewColorSpace,
    PreviewDateTime,
    RawImageDigest,
    // Digital Negative Specification, Version 1.4.0.0, page 67
    NoiseProfile,
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
impl Type {
    #[must_use]
    pub const fn new(type_: u16) -> Type {
        match type_ {
            0 => Type::Unknown(0),
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
            _ => Type::Unexpected(0),
        }
    }

    /*
     * I really wish I could just write _(size_in_bytes) => *size_in_bytes or access the data as a
     * property of the enum, like type_.size_in_bytes, without this boilerplate code.
     */
    #[must_use]
    pub const fn size_in_bytes(&self) -> u32 {
        match &self {
            Type::Unknown(size_in_bytes)
            | Type::Unexpected(size_in_bytes)
            | Type::Byte(size_in_bytes)
            | Type::Ascii(size_in_bytes)
            | Type::Sbyte(size_in_bytes)
            | Type::Undefined(size_in_bytes)
            | Type::Short(size_in_bytes)
            | Type::Sshort(size_in_bytes)
            | Type::Long(size_in_bytes)
            | Type::Slong(size_in_bytes)
            | Type::Float(size_in_bytes)
            | Type::Rational(size_in_bytes)
            | Type::Srational(size_in_bytes)
            | Type::Double(size_in_bytes) => *size_in_bytes,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Unknown(u32),
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
    Unexpected(u32),
}
