// TODO Tag and Type should be in separate modules and files

#[derive(Clone, Copy, Debug)]
pub enum Tag {
    // Digital Negative Specification, Version 1.4.0.0, page 18
    NewSubFileType,
    Unknown,
}

/*
 * TODO
 *
 * Unknown tag: 256
 * Unknown tag: 257
 * Unknown tag: 258
 * Unknown tag: 259
 * Unknown tag: 262
 * Unknown tag: 271
 * Unknown tag: 272
 * Unknown tag: 273
 * Unknown tag: 274
 * Unknown tag: 277
 * Unknown tag: 278
 * Unknown tag: 279
 * Unknown tag: 284
 * Unknown tag: 305
 * Unknown tag: 306
 * Unknown tag: 315
 * Unknown tag: 330
 * Unknown tag: 700
 * Unknown tag: 33432
 * Unknown tag: 34665
 * Unknown tag: 37393
 * Unknown tag: 50706
 * Unknown tag: 50707
 * Unknown tag: 50708
 * Unknown tag: 50709
 * Unknown tag: 50721
 * Unknown tag: 50722
 * Unknown tag: 50723
 * Unknown tag: 50724
 * Unknown tag: 50727
 * Unknown tag: 50728
 * Unknown tag: 50730
 * Unknown tag: 50731
 * Unknown tag: 50732
 * Unknown tag: 50734
 * Unknown tag: 50735
 * Unknown tag: 50736
 * Unknown tag: 50739
 * Unknown tag: 50740
 * Unknown tag: 50778
 * Unknown tag: 50779
 * Unknown tag: 50781
 * Unknown tag: 50827
 * Unknown tag: 50931
 * Unknown tag: 50932
 * Unknown tag: 50936
 * Unknown tag: 50941
 * Unknown tag: 50942
 * Unknown tag: 50964
 * Unknown tag: 50965
 * Unknown tag: 50966
 * Unknown tag: 50967
 * Unknown tag: 50969
 * Unknown tag: 50970
 * Unknown tag: 50971
 * Unknown tag: 50972
 * Unknown tag: 51041
 */

#[must_use]
pub const fn tag(tag: u16) -> Tag {
    match tag {
        254 => Tag::NewSubFileType,
        _ => Tag::Unknown,
    }
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
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Unknown,
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    Sbyte,
    Undefined,
    Sshort,
    Slong,
    Srational,
    Float,
    Double,
    Unexpected,
}

#[must_use]
pub const fn type_(type_: u16) -> Type {
    match type_ {
        0 => Type::Unknown,
        1 => Type::Byte,
        2 => Type::Ascii,
        3 => Type::Short,
        4 => Type::Long,
        5 => Type::Rational,
        6 => Type::Sbyte,
        7 => Type::Undefined,
        8 => Type::Sshort,
        9 => Type::Slong,
        10 => Type::Srational,
        11 => Type::Float,
        12 => Type::Double,
        _ => Type::Unexpected,
    }
}

#[must_use]
pub const fn type_size(type_: Type) -> u32 {
    match type_ {
        Type::Unknown | Type::Unexpected => 0,
        Type::Byte | Type::Ascii | Type::Sbyte | Type::Undefined => 1,
        Type::Short | Type::Sshort => 2,
        Type::Long | Type::Slong | Type::Float => 4,
        Type::Rational | Type::Srational | Type::Double => 8,
    }
}
