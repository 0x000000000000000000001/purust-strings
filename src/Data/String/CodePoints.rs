pub fn Data_String_CodePoints__unsafeCodePointAt0(mut fallback: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(s)
}

pub fn Data_String_CodePoints__codePointAt(mut fallback: crate::UnknownType, mut _just: crate::UnknownType, mut _nothing: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut n: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(n).call.as_ref().unwrap()(s)
}

pub fn Data_String_CodePoints__countPrefix(mut fallback: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut pred: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(pred).call.as_ref().unwrap()(s)
}

pub fn Data_String_CodePoints__fromCodePointArray(mut singleton: crate::UnknownType, mut cps: crate::UnknownType) -> crate::UnknownType {
    let arr = cps.init_array.as_ref().unwrap();
    let mut s = String::new();
    for cp_val in arr.iter() {
        let string_val = singleton.call.as_ref().unwrap()(cp_val.clone());
        s.push_str(string_val.init_string.as_ref().unwrap());
    }
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}

pub fn Data_String_CodePoints__singleton(mut fallback: crate::UnknownType, mut cp: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(cp)
}

pub fn Data_String_CodePoints__take(mut fallback: crate::UnknownType, mut n: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(n).call.as_ref().unwrap()(s)
}

pub fn Data_String_CodePoints__toCodePointArray(mut fallback: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    fallback.call.as_ref().unwrap()(s)
}
