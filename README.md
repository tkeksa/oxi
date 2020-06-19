# `oxi`

[![Build Status][build-image]][build-link]
[![MIT licensed][license-image]][license-link]

[OLinuXino][OLINUXINO] EEPROM board info

## Resources
- [OLinuXino EEPROM Content][OLIMEX-A20-EEPROM]
- [OLinuXino U-Boot boards.c file with list of boards][OLINUXINO-UBOOT-BOARDS]

##### Generate list of board IDs from OLinuXino U-Boot boards.c file 
`wget -qO- https://raw.githubusercontent.com/OLIMEX/u-boot-olinuxino/release-20200601/board/olimex/common/boards.c | perl extract_board_ids.pl > boards.rs`

## License
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[build-image]: https://github.com/tkeksa/oxi/workflows/ci/badge.svg
[build-link]: https://github.com/tkeksa/oxi/actions
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: http://opensource.org/licenses/MIT
[OLINUXINO]: https://github.com/OLIMEX/OLINUXINO
[OLIMEX-A20-EEPROM]: https://github.com/OLIMEX/OLINUXINO/blob/master/SOFTWARE/A20/A20-eeprom-contents/Olimex-A20-EEPROM-October-2019.pdf
[OLINUXINO-UBOOT-BOARDS]: https://github.com/OLIMEX/u-boot-olinuxino/blob/release-20200601/board/olimex/common/boards.c
