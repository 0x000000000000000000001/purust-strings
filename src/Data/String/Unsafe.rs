pub fn Data_String_Unsafe_charAt(mut idx_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let idx = idx_val.init_int.unwrap() as usize;
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(c) = s.chars().nth(idx) {
        crate::UnknownType::new(crate::Record_a { init_char: Some(c), ..Default::default() })
    } else {
        panic!("charAt out of bounds");
    }
}

pub fn Data_String_Unsafe_char(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(c) = s.chars().next() {
        crate::UnknownType::new(crate::Record_a { init_char: Some(c), ..Default::default() })
    } else {
        panic!("char out of bounds");
    }
}
