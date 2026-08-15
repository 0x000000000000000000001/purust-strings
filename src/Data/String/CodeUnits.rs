

// FFI implementation for Data.String.CodeUnits in Rust

pub fn Data_String_CodeUnits__charAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut i_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let i = i_val.init_int.unwrap() as usize;
    let s = s_val.init_string.as_ref().unwrap();
    if i < s.chars().count() {
        let c = s.chars().nth(i).unwrap();
        let char_val = crate::UnknownType::new(crate::Record_a { init_char: Some(c), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![char_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__indexOf(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.init_string.as_ref().unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(byte_idx) = s.find(x) {
        let char_idx = s[..byte_idx].chars().count() as i64;
        let int_val = crate::UnknownType::new(crate::Record_a { init_int: Some(char_idx), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![int_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__indexOfStartingAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut startAt_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.init_string.as_ref().unwrap();
    let start_char_idx = startAt_val.init_int.unwrap() as usize;
    let s = s_val.init_string.as_ref().unwrap();
    
    let byte_idx_opt = s.char_indices().nth(start_char_idx).map(|(i, _)| i);
    let search_slice = if let Some(idx) = byte_idx_opt {
        &s[idx..]
    } else if start_char_idx == s.chars().count() {
        ""
    } else {
        return nothing;
    };
    
    if let Some(match_byte_idx) = search_slice.find(x) {
        let absolute_byte_idx = byte_idx_opt.unwrap_or(0) + match_byte_idx;
        let char_idx = s[..absolute_byte_idx].chars().count() as i64;
        let int_val = crate::UnknownType::new(crate::Record_a { init_int: Some(char_idx), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![int_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__lastIndexOf(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.init_string.as_ref().unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    if let Some(byte_idx) = s.rfind(x) {
        let char_idx = s[..byte_idx].chars().count() as i64;
        let int_val = crate::UnknownType::new(crate::Record_a { init_int: Some(char_idx), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![int_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__lastIndexOfStartingAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut startAt_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.init_string.as_ref().unwrap();
    let startAt_i64 = startAt_val.init_int.unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    
    let len = s.chars().count() as i64;
    if startAt_i64 < 0 || startAt_i64 > len {
        return nothing;
    }
    let start_char_idx = startAt_i64 as usize;
    
    let mut last_match_char_idx = None;
    for (byte_idx, _) in s.match_indices(x) {
        let char_idx = s[..byte_idx].chars().count();
        if char_idx <= start_char_idx {
            last_match_char_idx = Some(char_idx);
        }
    }
    
    if let Some(char_idx) = last_match_char_idx {
        let int_val = crate::UnknownType::new(crate::Record_a { init_int: Some(char_idx as i64), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![int_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__toChar(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 1 {
        let char_val = crate::UnknownType::new(crate::Record_a { init_char: Some(chars[0]), ..Default::default() });
        crate::UnknownType::new(crate::Record_a { tag: "Just", vals: Some(std::rc::Rc::new(vec![char_val])), ..Default::default() })
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits_countPrefix(mut p_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let mut count = 0;
    for c in s.chars() {
        let char_val = crate::UnknownType::new(crate::Record_a { init_char: Some(c), ..Default::default() });
        let p_res = p_val.call.as_ref().unwrap()(char_val);
        if p_res.init_bool.unwrap() {
            count += 1;
        } else {
            break;
        }
    }
    crate::UnknownType::new(crate::Record_a { init_int: Some(count), ..Default::default() })
}

pub fn Data_String_CodeUnits_drop(mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.init_int.unwrap() as usize;
    let s = s_val.init_string.as_ref().unwrap();
    let dropped = s.chars().skip(n).collect::<String>();
    crate::UnknownType::new(crate::Record_a { init_string: Some(dropped), ..Default::default() })
}

pub fn Data_String_CodeUnits_fromCharArray(mut a_val: crate::UnknownType) -> crate::UnknownType {
    let arr = a_val.init_array.as_ref().unwrap();
    let mut s = String::new();
    for v in arr.iter() {
        s.push(v.init_char.unwrap());
    }
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}

pub fn Data_String_CodeUnits_length(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let len = s.chars().count() as i64;
    crate::UnknownType::new(crate::Record_a { init_int: Some(len), ..Default::default() })
}

pub fn Data_String_CodeUnits_singleton(mut c_val: crate::UnknownType) -> crate::UnknownType {
    let c = c_val.init_char.unwrap();
    crate::UnknownType::new(crate::Record_a { init_string: Some(c.to_string()), ..Default::default() })
}

pub fn Data_String_CodeUnits_slice(mut b_val: crate::UnknownType, mut e_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let b_i64 = b_val.init_int.unwrap();
    let e_i64 = e_val.init_int.unwrap();
    let b = if b_i64 < 0 { 0 } else { b_i64 as usize };
    let e = if e_i64 < 0 { 0 } else { e_i64 as usize };
    
    let s = s_val.init_string.as_ref().unwrap();
    
    let len = s.chars().count();
    let start = if b > len { len } else { b };
    let end = if e > len { len } else { e };
    let sliced = if start >= end { "".to_string() } else { s.chars().skip(start).take(end - start).collect::<String>() };
    crate::UnknownType::new(crate::Record_a { init_string: Some(sliced), ..Default::default() })
}

pub fn Data_String_CodeUnits_splitAt(mut i_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let i_i64 = i_val.init_int.unwrap();
    let i = if i_i64 < 0 { 0 } else { i_i64 as usize };
    let s = s_val.init_string.as_ref().unwrap();
    let before_str = s.chars().take(i).collect::<String>();
    let after_str = s.chars().skip(i).collect::<String>();
    let before = crate::UnknownType::new(crate::Record_a { init_string: Some(before_str), ..Default::default() });
    let after = crate::UnknownType::new(crate::Record_a { init_string: Some(after_str), ..Default::default() });
    crate::UnknownType::new(crate::Record_a { before: Some(before), after: Some(after), ..Default::default() })
}

pub fn Data_String_CodeUnits_take(mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.init_int.unwrap() as usize;
    let s = s_val.init_string.as_ref().unwrap();
    let taken = s.chars().take(n).collect::<String>();
    crate::UnknownType::new(crate::Record_a { init_string: Some(taken), ..Default::default() })
}

pub fn Data_String_CodeUnits_toCharArray(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let arr = s.chars().map(|c| crate::UnknownType::new(crate::Record_a { init_char: Some(c), ..Default::default() })).collect::<Vec<_>>();
    crate::UnknownType::new(crate::Record_a { init_array: Some(std::rc::Rc::new(arr)), ..Default::default() })
}
