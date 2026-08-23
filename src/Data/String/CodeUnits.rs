

// FFI implementation for Data.String.CodeUnits in Rust

pub fn Data_String_CodeUnits__charAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut i_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let i = i_val.unwrap_int() as usize;
    let s = s_val.unwrap_string();
    if i < s.chars().count() {
        let c = s.chars().nth(i).unwrap();
        let char_val = crate::Value::Char(c);
        just.unwrap_func1()(char_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__indexOf(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.unwrap_string();
    let s = s_val.unwrap_string();
    if let Some(byte_idx) = s.find(x) {
        let char_idx = s[..byte_idx].chars().count() as i64;
        let int_val = crate::Value::Int(char_idx);
        just.unwrap_func1()(int_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__indexOfStartingAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut startAt_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.unwrap_string();
    let start_char_idx = startAt_val.unwrap_int() as usize;
    let s = s_val.unwrap_string();
    
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
        let int_val = crate::Value::Int(char_idx);
        just.unwrap_func1()(int_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__lastIndexOf(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.unwrap_string();
    let s = s_val.unwrap_string();
    if let Some(byte_idx) = s.rfind(x) {
        let char_idx = s[..byte_idx].chars().count() as i64;
        let int_val = crate::Value::Int(char_idx);
        just.unwrap_func1()(int_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__lastIndexOfStartingAt(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut x_val: crate::UnknownType, mut startAt_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let x = x_val.unwrap_string();
    let startAt_i64 = startAt_val.unwrap_int();
    let s = s_val.unwrap_string();
    
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
        let int_val = crate::Value::Int(char_idx as i64);
        just.unwrap_func1()(int_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits__toChar(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 1 {
        let char_val = crate::Value::Char(chars[0]);
        just.unwrap_func1()(char_val)
    } else {
        nothing
    }
}

pub fn Data_String_CodeUnits_countPrefix(mut p_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let mut count = 0;
    for c in s.chars() {
        let char_val = crate::Value::Char(c);
        let p_res = p_val.unwrap_func1()(char_val);
        if p_res.unwrap_bool() {
            count += 1;
        } else {
            break;
        }
    }
    crate::Value::Int(count)
}

pub fn Data_String_CodeUnits_drop(mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.unwrap_int() as usize;
    let s = s_val.unwrap_string();
    let dropped = s.chars().skip(n).collect::<String>();
    crate::Value::String(dropped)
}

pub fn Data_String_CodeUnits_fromCharArray(mut a_val: crate::UnknownType) -> crate::UnknownType {
    let arr = a_val.unwrap_array();
    let mut s = String::new();
    for v in arr.iter() {
        s.push(v.unwrap_char());
    }
    crate::Value::String(s)
}

pub fn Data_String_CodeUnits_length(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let len = s.chars().count() as i64;
    crate::Value::Int(len)
}

pub fn Data_String_CodeUnits_singleton(mut c_val: crate::UnknownType) -> crate::UnknownType {
    let c = c_val.unwrap_char();
    crate::Value::String(c.to_string())
}

pub fn Data_String_CodeUnits_slice(mut b_val: crate::UnknownType, mut e_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let b_i64 = b_val.unwrap_int();
    let e_i64 = e_val.unwrap_int();
    let b = if b_i64 < 0 { 0 } else { b_i64 as usize };
    let e = if e_i64 < 0 { 0 } else { e_i64 as usize };
    
    let s = s_val.unwrap_string();
    
    let len = s.chars().count();
    let start = if b > len { len } else { b };
    let end = if e > len { len } else { e };
    let sliced = if start >= end { "".to_string() } else { s.chars().skip(start).take(end - start).collect::<String>() };
    crate::Value::String(sliced)
}

pub fn Data_String_CodeUnits_splitAt(mut i_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let i_i64 = i_val.unwrap_int();
    let i = if i_i64 < 0 { 0 } else { i_i64 as usize };
    let s = s_val.unwrap_string();
    let before_str = s.chars().take(i).collect::<String>();
    let after_str = s.chars().skip(i).collect::<String>();
    let before = crate::Value::String(before_str);
    let after = crate::Value::String(after_str);
    crate::Value::Record_after_before(perceus_ptr::PerceusPtr::new(crate::Record_after_before { after, before }))
}

pub fn Data_String_CodeUnits_take(mut n_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.unwrap_int() as usize;
    let s = s_val.unwrap_string();
    let taken = s.chars().take(n).collect::<String>();
    crate::Value::String(taken)
}

pub fn Data_String_CodeUnits_toCharArray(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let arr = s.chars().map(|c| crate::Value::Char(c)).collect::<Vec<_>>();
    crate::Value::Array(std::rc::Rc::new(arr))
}
