// FFI implementation for Data.Show in Rust

pub fn Data_Show_showStringImpl(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    // In JS this uses JSON.stringify(s). For Rust, we can just format it with debug formatting
    // or manually add quotes. Debug formatting `{:?}` is close enough to JSON.stringify for basic strings.
    let stringified = format!("{:?}", s);
    crate::Value::String(stringified)
}

pub fn Data_Show_showIntImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::Value::String(a0.unwrap_int().to_string())
}

pub fn Data_Show_showNumberImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::Value::String(a0.unwrap_number().to_string())
}

pub fn Data_Show_showCharImpl(mut a0: crate::UnknownType) -> crate::UnknownType {
    let c = a0.unwrap_char();
    crate::Value::String(format!("'{}'", c))
}

pub fn Data_Show_showArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType) -> crate::UnknownType {
    let arr = a0.unwrap_array();
    let mut s = String::from("[");
    for (i, x) in arr.iter().enumerate() {
        if i > 0 {
            s.push_str(",");
        }
        let res = f.unwrap_func()(x.clone());
        s.push_str(&res.unwrap_string());
    }
    s.push_str("]");
    crate::Value::String(s)
}
