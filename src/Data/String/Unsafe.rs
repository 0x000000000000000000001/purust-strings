pub fn Data_String_Unsafe_charAt(mut idx_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let idx = idx_val.unwrap_int() as usize;
    let s = s_val.unwrap_string();
    if let Some(c) = s.chars().nth(idx) {
        crate::Value::Char(c)
    } else {
        panic!("charAt out of bounds");
    }
}

pub fn Data_String_Unsafe_char(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    if let Some(c) = s.chars().next() {
        crate::Value::Char(c)
    } else {
        panic!("char out of bounds");
    }
}
