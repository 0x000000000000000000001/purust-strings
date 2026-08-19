pub fn Data_Enum_toCharCode(mut a0: crate::UnknownType) -> crate::UnknownType {
    if a0.init_char.is_none() {
        panic!("Data_Enum_toCharCode expected char, got tag: {}", a0.tag);
    }
    let c = a0.unwrap_char();
    crate::mk_int(c as u32 as i64)
}

pub fn Data_Enum_fromCharCode(mut a0: crate::UnknownType) -> crate::UnknownType {
    let i = a0.unwrap_int();
    // charToEnum already does bounds checking, so this should always be safe.
    // Use unwrap_or to fallback gracefully if some bounds logic is weird.
    let c = std::char::from_u32(i as u32).unwrap_or('\u{FFFD}');
    crate::Value::Char(c)
}
