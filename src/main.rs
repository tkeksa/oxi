use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use bytes::Buf;
use getopts::Options;
use hexplay::{HexViewBuilder, CODEPAGE_ASCII};

mod boards;

fn usage(opts: &Options) {
    let brief = r#"OLinuXino EEPROM board info

oxi [-h] [-x] [-i I2CBUS]"#;

    print!("{}", opts.usage(brief));
}

macro_rules! i2c_dev_fmt {
    () => {
        "/dev/i2c-{}"
    };
}
const I2C_DEV_MIN: u8 = 0;
const I2C_DEV_MAX: u8 = 255;

fn find_i2c_devs() -> Vec<u8> {
    (I2C_DEV_MIN..=I2C_DEV_MAX)
        .filter(|i| Path::new(&format!(i2c_dev_fmt!(), i)).exists())
        .collect()
}

macro_rules! dev_eeprom_fmt {
    () => {
        "/sys/bus/i2c/devices/{}-0050//eeprom"
    };
}
fn find_eeproms(devs: &[u8]) -> Vec<u8> {
    devs.iter()
        .filter(|&i| Path::new(&format!(dev_eeprom_fmt!(), i)).exists())
        .cloned()
        .collect()
}

const MEM_LEN: usize = 256;
const HEADER: u32 = 0x4F4C_AA55;

fn parse_buf(buf: &[u8]) {
    let mut buf = buf;

    let hdr = buf.get_u32_le();
    println!(
        "Header  : {:#010X} - {}",
        hdr,
        if hdr == HEADER { "OK" } else { "invalid" }
    );

    let id = buf.get_u32_le();
    println!(
        "ID      : {:4} - {}",
        id,
        boards::IDS.get(&id).unwrap_or(&"unknown")
    );

    let rev_maj = buf.get_u8() as char;
    let rev_min = buf.get_u8();
    let rev_min = if (rev_min > 0) && (rev_min < 10) {
        format!("{}", rev_min)
    } else {
        "".to_string()
    };
    println!("Revision: {}{}", rev_maj, rev_min);

    let serial = buf.get_u32_le();
    println!("Serial  : {:#010X}", serial);

    let storage = buf.get_u8();
    let size = buf.get_u8();
    let ram = buf.get_u8();
    let grade = buf.get_u8();
    let size = match size {
        x if x < 10 => (1u16 << x, "B"),
        x if x < 20 => (1u16 << (x - 10), "KB"),
        x if x < 30 => (1u16 << (x - 20), "MB"),
        x => (1u16 << (x - 30), "GB"),
    };
    let storage = match storage {
        0x00 => "none".to_string(),
        0x65 => format!("eMMC {}{}", size.0, size.1),
        0x6E => format!("NAND {}{}", size.0, size.1),
        0x73 => format!("SPI Flash {}{}", size.0, size.1),
        _ => "unknown".to_string(),
    };
    let ram = match ram {
        x if x < 10 => (1u16 << x, "B"),
        x if x < 20 => (1u16 << (x - 10), "KB"),
        x if x < 30 => (1u16 << (x - 20), "MB"),
        x => (1u16 << (x - 30), "GB"),
    };

    let grade = match grade {
        0x00 => "commercial (0-70) degrees Celsius",
        0x01 => "industrial (-45+85) degrees Celsius",
        _ => "unknown",
    };
    println!("Storage : {}", storage);
    println!("RAM     : {}{}", ram.0, ram.1);
    println!("Grade   : {}", grade);

    let mut mac = [0u8; 12];
    buf.copy_to_slice(&mut mac);
    let mac = mac
        .chunks(2)
        .map(|x| format!("{}{}", x[0] as char, x[1] as char))
        .collect::<Vec<_>>()
        .join(":");
    println!("MAC     : {}", mac);
}

fn main() -> Result<(), io::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let mut opts = Options::new();
    opts.optopt("i", "i2c", "Number of the I2C bus", "I2CBUS");
    opts.optflag("x", "hex", "Hexadecimal dump");
    opts.optflag("h", "help", "Help");
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f);
            usage(&opts);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        usage(&opts);
        return Ok(());
    }

    let eeprom_i2c = if let Some(i) = matches
        .opt_get::<u8>("i")
        .expect("Invalid I2CBUS number format")
    {
        i
    } else {
        // find EEPROM
        let devs = find_i2c_devs();
        let eeproms = find_eeproms(&devs);
        if let Some(&i) = eeproms.get(0) {
            i
        } else {
            eprintln!("EEPROM not found");
            std::process::exit(1);
        }
    };
    let eeprom_dev = format!(dev_eeprom_fmt!(), eeprom_i2c);
    let mut file =
        File::open(&eeprom_dev).map_err(|e| io::Error::new(e.kind(), eeprom_dev.as_str()))?;
    let mut buf = [0u8; MEM_LEN];
    file.read_exact(&mut buf)?;
    drop(file);

    println!("OLinuXino EEPROM content ({})", eeprom_dev);
    if matches.opt_present("x") {
        let hex = HexViewBuilder::new(&buf).codepage(CODEPAGE_ASCII).finish();
        hex.print()?;
        println!();
    } else {
        parse_buf(&buf);
    }

    Ok(())
}
