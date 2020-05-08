#!perl

use warnings;
use integer;
use strict;

print <<HDR;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDS: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
HDR

while (<>) {
    next unless /OLINUXINO_BOARD_\w+\s*\(\s*(\d+)\s*,\s*"([^"]+)"/;
    print qq/        m.insert($1, "$2");\n/;
}

print <<FTR;
        m
    };
}
FTR
