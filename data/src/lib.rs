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

use std::collections::HashMap;

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

// TODO Tag and Type should be in separate modules and files

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
 *  5 = RATIONAL Two LONGs: the first represents the numerator of a fraction; the second, the
 *      denominator.
 *  6 = SBYTE An 8-bit signed (twos-complement) integer.
 *  7 = UNDEFINED An 8-bit byte that may contain anything, depending on the definition of the
 *      field.
 *  8 = SSHORT A 16-bit (2-byte) signed (twos-complement) integer.
 *  9 = SLONG A 32-bit (4-byte) signed (twos-complement) integer.
 * 10 = SRATIONAL Two SLONG’s: the first represents the numerator of a fraction, the second the
 *      denominator.
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
    ShadowScale,
    DNGPrivateData,
    CalibrationIlluminant1,
    CalibrationIlluminant2,
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
