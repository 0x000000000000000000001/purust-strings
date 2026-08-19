pub fn Data_Bounded_bottomChar() -> crate::UnknownType {
    crate::Value::Char('\u{0}')
}

pub fn Data_Bounded_topChar() -> crate::UnknownType {
    // Rust char cannot represent 65535 (\u{FFFF}) because it's not a valid scalar value or rather because... wait, 0xFFFF is a valid scalar value!
    // Wait, D800 to DFFF are surrogates (not valid scalar values). FFFF is a valid scalar value!
    crate::Value::Char('\u{FFFF}')
}

pub fn Data_Bounded_bottomInt() -> crate::UnknownType {
    crate::Value::Int(-2147483648)
}

pub fn Data_Bounded_topInt() -> crate::UnknownType {
    crate::Value::Int(2147483647)
}

pub fn Data_Bounded_bottomNumber() -> crate::UnknownType {
    crate::Value::Number(std::f64::NEG_INFINITY)
}

pub fn Data_Bounded_topNumber() -> crate::UnknownType {
    crate::Value::Number(std::f64::INFINITY)
}
