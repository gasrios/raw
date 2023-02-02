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

use data::Offset;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind::InvalidData};
use tiff_reader::TiffReader;

fn main() -> Result<(), Error> {
    if let Some(file_name) = args().nth(1) {
        let mut tiff_reader: TiffReader<BufReader<File>> =
            TiffReader::new(BufReader::new(File::open(file_name)?))?;
        {
            // TODO move this code block to tiff_reader::TiffReader, make if return a data structure
            let mut offset: Offset = tiff_reader.process_header()?;
            loop {
                offset = tiff_reader.process_ifd(offset)?;

                /*
                 * From TIFF 6.0 Specification, page 14
                 *
                 * An Image File Directory (IFD) consists of (...) followed by a 4-byte offset of the
                 * next IFD (or 0 if none).
                 */
                if offset == 0 {
                    break;
                }
            }
        }
    } else {
        return Err(Error::new(InvalidData, "Please specify a file"));
    }

    Ok(())
}
