pub fn Data_String_CodePoints__unsafeCodePointAt0(mut fallback: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(c) = s.chars().next() {
        crate::UnknownType::new(crate::Record_a { init_int: Some(c as u32 as i64), ..Default::default() })
    } else {
        panic!("unsafeCodePointAt0 out of bounds");
    }
}

pub fn Data_String_CodePoints__codePointAt(mut _fallback: crate::UnknownType, mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.init_int.unwrap();
    if n < 0 {
        return nothing;
    }
    let n = n as usize;
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(c) = s.chars().nth(n) {
        let cp = crate::UnknownType::new(crate::Record_a { init_int: Some(c as u32 as i64), ..Default::default() });
        crate::UnknownType::new(crate::Record_a {
            tag: just.tag,
            vals: Some(std::rc::Rc::new(vec![cp])),
            ..Default::default()
        })
    } else {
        nothing
    }
}

pub fn Data_String_CodePoints__countPrefix(mut _fallback: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut pred: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let mut count = 0;
    for c in s.chars() {
        let cp = crate::UnknownType::new(crate::Record_a { init_int: Some(c as u32 as i64), ..Default::default() });
        let is_match = pred.call.as_ref().unwrap()(cp);
        if is_match.init_bool.unwrap() {
            count += 1;
        } else {
            break;
        }
    }
    crate::UnknownType::new(crate::Record_a { init_int: Some(count), ..Default::default() })
}

pub fn Data_String_CodePoints__fromCodePointArray(mut _singleton: crate::UnknownType, mut cps: crate::UnknownType) -> crate::UnknownType {
    let arr = cps.init_array.as_ref().unwrap();
    let mut s = String::new();
    for cp_val in arr.iter() {
        let cp_i64 = cp_val.init_int.unwrap();
        if let Some(c) = std::char::from_u32(cp_i64 as u32) {
            s.push(c);
        } else {
            s.push(std::char::REPLACEMENT_CHARACTER);
        }
    }
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}

pub fn Data_String_CodePoints__singleton(mut _fallback: crate::UnknownType, mut cp: crate::UnknownType) -> crate::UnknownType {
    let cp_i64 = cp.init_int.unwrap();
    let s = if let Some(c) = std::char::from_u32(cp_i64 as u32) {
        c.to_string()
    } else {
        std::char::REPLACEMENT_CHARACTER.to_string()
    };
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}

pub fn Data_String_CodePoints__take(mut _fallback: crate::UnknownType, mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n_i64 = n_val.init_int.unwrap();
    let n = if n_i64 < 0 { 0 } else { n_i64 as usize };
    let s = s_val.init_string.as_ref().unwrap();
    let taken = s.chars().take(n).collect::<String>();
    crate::UnknownType::new(crate::Record_a { init_string: Some(taken), ..Default::default() })
}

pub fn Data_String_CodePoints__toCodePointArray(mut _fallback: crate::UnknownType, mut _unsafeCodePointAt0: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let arr: Vec<crate::UnknownType> = s.chars().map(|c| {
        crate::UnknownType::new(crate::Record_a { init_int: Some(c as u32 as i64), ..Default::default() })
    }).collect();
    crate::UnknownType::new(crate::Record_a { init_array: Some(std::rc::Rc::new(arr)), ..Default::default() })
}
