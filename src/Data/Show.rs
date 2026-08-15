// FFI implementation for Data.Show in Rust

pub fn Data_Show_showStringImpl(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    // In JS this uses JSON.stringify(s). For Rust, we can just format it with debug formatting
    // or manually add quotes. Debug formatting `{:?}` is close enough to JSON.stringify for basic strings.
    let stringified = format!("{:?}", s);
    crate::UnknownType::new(crate::Record_a { init_string: Some(stringified), ..Default::default() })
}

pub fn Data_Show_showIntImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_string: Some(a0.init_int.unwrap().to_string()), ..Default::default() })
}

pub fn Data_Show_showNumberImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { init_string: Some(a0.init_number.unwrap().to_string()), ..Default::default() })
}

pub fn Data_Show_showCharImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    let c = a0.init_char.unwrap();
    crate::UnknownType::new(crate::Record_a { init_string: Some(format!("'{}'", c)), ..Default::default() })
}

pub fn Data_Show_showArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType) -> crate::UnknownType {
    let arr = a0.init_array.as_ref().unwrap();
    let mut s = String::from("[");
    for (i, x) in arr.iter().enumerate() {
        if i > 0 {
            s.push_str(",");
        }
        let res = f.call.as_ref().unwrap()(x.clone());
        s.push_str(res.init_string.as_ref().unwrap());
    }
    s.push_str("]");
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}
