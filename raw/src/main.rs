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

use data::Tag;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData};
use tiff_reader::{Ifd, Offset, TiffReader};

// TODO process SubIFDs
fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut tiff_reader: TiffReader<BufReader<File>> =
            TiffReader::new(BufReader::new(File::open(file_name)?))?;
        let ifds: Vec<Ifd> = tiff_reader.read()?;
        if let Some(field) = ifds[0].fields.get(&Tag::SubIFDs) {
            // TODO treat count > 1
            let offset: Offset = tiff_reader.to_offset(&field.raw_data[0..4])?;
            let ifd: Ifd = tiff_reader.process_ifd(offset)?;
            for key in ifd.fields.keys() {
                println!("{key:?}");
                if let Some(field) = ifd.fields.get(key) {
                    println!("\tType: {:?}", field.type_);
                    println!("\tNumber of values: {}", field.count);
                    println!("\tValue: {:?}", field.raw_data);
                }
            }
        }
        /*
        for ifd in ifds {
            for key in ifd.fields.keys() {
                println!("{key:?}");
                if let Some(field) = ifd.fields.get(key) {
                    println!("\tType: {:?}", field.type_);
                    println!("\tNumber of values: {}", field.count);
                    println!("\tValue: {:?}", field.raw_data);
                }
            }
        }
         */
    } else {
        return Err(Error::new(InvalidData, "Please specify a file"));
    }
    Ok(())
}
