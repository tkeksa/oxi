use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDS: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(7739, "A20-OLinuXino-LIME");
        m.insert(7743, "A20-OLinuXino-LIME-n4GB");
        m.insert(8934, "A20-OLinuXino-LIME-n8G");
        m.insert(9076, "A20-OLinuXino-LIME-s16MB");
        m.insert(9160, "A20-OLinuXino-LIME-e4GB");
        m.insert(9516, "A20-OLinuXino-LIME-e16Gs16M");
        m.insert(9696, "A20-OLinuXino-LIME-e4Gs16M");
        m.insert(9211, "T2-OLinuXino-LIME-IND");
        m.insert(9215, "T2-OLinuXino-LIME-s16MB-IND");
        m.insert(9219, "T2-OLinuXino-LIME-e4G-IND");
        m.insert(9734, "T2-OLinuXino-LIME-e4Gs16M-IND");
        m.insert(10481, "T2-OLinuXino-LIME-e8Gs16M-IND");
        m.insert(7701, "A20-OLinuXino-LIME2");
        m.insert(8340, "A20-OLinuXino-LIME2-e4GB");
        m.insert(9166, "A20-OLinuXino-LIME2-e16GB");
        m.insert(7624, "A20-OLinuXino-LIME2-n4GB");
        m.insert(8910, "A20-OLinuXino-LIME2-n8GB");
        m.insert(8946, "A20-OLinuXino-LIME2-s16M");
        m.insert(9604, "A20-OLinuXino-LIME2-e16Gs16M");
        m.insert(9613, "A20-OLinuXino-LIME2-e4Gs16M");
        m.insert(9905, "A20-OLinuXino-LIME2-G2");
        m.insert(9239, "T2-OLinuXino-LIME2-IND");
        m.insert(9247, "T2-OLinuXino-LIME2-s16M-IND");
        m.insert(9243, "T2-OLinuXino-LIME2-e8Gs16M-IND");
        m.insert(4614, "A20-OLinuXino-MICRO");
        m.insert(4615, "A20-OLinuXino-MICRO-n4GB");
        m.insert(8661, "A20-OLinuXino-MICRO-e4GB-IND");
        m.insert(8828, "A20-OLinuXino-MICRO-IND");
        m.insert(8832, "A20-OLinuXino-MICRO-e4GB");
        m.insert(8918, "A20-OLinuXino-MICRO-n8G");
        m.insert(9042, "A20-OLinuXino-MICRO-e16G");
        m.insert(9231, "A20-OLinuXino-MICRO-s16M");
        m.insert(9684, "A20-OLinuXino-MICRO-e4Gs16M");
        m.insert(9689, "A20-OLinuXino-MICRO-e16Gs16M");
        m.insert(9223, "T2-OLinuXino-MICRO-IND");
        m.insert(9227, "T2-OLinuXino-MICRO-e4G-IND");
        m.insert(9235, "T2-OLinuXino-MICRO-s16M-IND");
        m.insert(9739, "T2-OLinuXino-MICRO-e4Gs16M-IND");
        m.insert(9789, "T2-OLinuXino-MICRO-e8Gs16M-IND");
        m.insert(4673, "A20-SOM-n4GB");
        m.insert(7664, "A20-SOM");
        m.insert(8849, "A20-SOM-IND");
        m.insert(8922, "A20-SOM-n8GB");
        m.insert(9155, "A20-SOM-e16GB");
        m.insert(9148, "A20-SOM-e16GB-IND");
        m.insert(9047, "A20-SOM-e16Gs16M");
        m.insert(9259, "T2-SOM-IND");
        m.insert(9827, "T2-SOM-e8Gs16M-IND");
        m.insert(8991, "A20-SOM204-1G");
        m.insert(8958, "A20-SOM204-1Gs16Me16G-MC");
        m.insert(10257, "A20-SOM204-1G-M");
        m.insert(10157, "T2-SOM204-1Gs16Me4G-C-I");
        m.insert(10234, "T2-SOM204-1Gs16Me8G-MC-I");
        m.insert(10238, "T2-SOM204-1G-I");
        m
    };
}
