pub fn Data_Bounded_bottomChar() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_char: Some('\u{0}'), ..Default::default() })
}

pub fn Data_Bounded_topChar() -> crate::UnknownType {
    // Rust char cannot represent 65535 (\u{FFFF}) because it's not a valid scalar value or rather because... wait, 0xFFFF is a valid scalar value!
    // Wait, D800 to DFFF are surrogates (not valid scalar values). FFFF is a valid scalar value!
    crate::UnknownType::new(crate::Record_a { init_char: Some('\u{FFFF}'), ..Default::default() })
}

pub fn Data_Bounded_bottomInt() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_int: Some(-2147483648), ..Default::default() })
}

pub fn Data_Bounded_topInt() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_int: Some(2147483647), ..Default::default() })
}

pub fn Data_Bounded_bottomNumber() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_number: Some(std::f64::NEG_INFINITY), ..Default::default() })
}

pub fn Data_Bounded_topNumber() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_number: Some(std::f64::INFINITY), ..Default::default() })
}
