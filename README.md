# raw

This is **not** a production ready raw reader, just a personal project I started while studying digital image processing. The [first version](https://github.com/raw-tiff) was written in Java, then migrated to [Rust](https://www.rust-lang.org/) because, you guessed it, I am now studying Rust.

If you need a complete, fully functional library, check [dcraw](https://www.cybercom.net/~dcoffin/dcraw/) out.

Currently the only format supported is uncompressed, linear (demosaiced) [Adobe Digital Negative (DNG)](https://helpx.adobe.com/camera-raw/digital-negative.html). Reading TIFF metadata is also supported.

I decided to support DNG first because, unlike other formats such as Canon's CR2 or Nikon's .NEF, DNG has a specification publicly available. Also, virtually all widely used raw formats are TIFF-based, like DNG, so if you can read it, you are more than halfway done reading any raw file format.

You can use [Adobe Digital Negative Converter](https://helpx.adobe.com/camera-raw/using/adobe-dng-converter.html) to convert other raw formats to DNG.

## Before you begin

Keep in mind TIFF is a decades old file format that has been receiving extensions for as long as has existed. It's full of idiosyncrasies and I strongly encourage you to read the following specifications before proceeding:

* TIFF Revision 6.0 Final - June 3, 1992
* TIFF Technical Note 1: TIFF Trees
* ISO 12234-2:2001, Electronic still-picture imaging – Removable memory – Part 2: TIFF/EP image data format
* Digital Negative (DNG) Specification Version 1.4.0.0
_____
## Copyright & License

The following copyright notice applies to all files in this repository, unless otherwise indicated in the file.

### © 2023 Guilherme Rios All Rights Reserved

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.
