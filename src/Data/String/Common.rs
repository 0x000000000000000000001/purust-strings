// FFI implementation for Data.String.Common in Rust

pub fn Data_String_Common__localeCompare(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a_val: crate::UnknownType, mut b_val: crate::UnknownType) -> crate::UnknownType {
    let a = a_val.init_string.as_ref().unwrap();
    let b = b_val.init_string.as_ref().unwrap();
    match a.cmp(b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_String_Common_replace(mut pat_val: crate::UnknownType, mut rep_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.init_string.as_ref().unwrap();
    let rep = rep_val.init_string.as_ref().unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    let replaced = s.replacen(pat, rep, 1);
    crate::UnknownType::new(crate::Record_a { init_string: Some(replaced), ..Default::default() })
}

pub fn Data_String_Common_replaceAll(mut pat_val: crate::UnknownType, mut rep_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.init_string.as_ref().unwrap();
    let rep = rep_val.init_string.as_ref().unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    let replaced = s.replace(pat, rep);
    crate::UnknownType::new(crate::Record_a { init_string: Some(replaced), ..Default::default() })
}

pub fn Data_String_Common_split(mut pat_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.init_string.as_ref().unwrap();
    let s = s_val.init_string.as_ref().unwrap();
    let parts: Vec<crate::UnknownType> = if pat.is_empty() {
        if s.is_empty() {
            vec![]
        } else {
            s.chars().map(|c| crate::UnknownType::new(crate::Record_a { init_string: Some(c.to_string()), ..Default::default() })).collect()
        }
    } else {
        s.split(pat).map(|part| crate::UnknownType::new(crate::Record_a { init_string: Some(part.to_string()), ..Default::default() })).collect()
    };
    crate::UnknownType::new(crate::Record_a { init_array: Some(std::rc::Rc::new(parts)), ..Default::default() })
}

pub fn Data_String_Common_toLower(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let lower = s.to_lowercase();
    crate::UnknownType::new(crate::Record_a { init_string: Some(lower), ..Default::default() })
}

pub fn Data_String_Common_toUpper(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let upper = s.to_uppercase();
    crate::UnknownType::new(crate::Record_a { init_string: Some(upper), ..Default::default() })
}

pub fn Data_String_Common_trim(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.init_string.as_ref().unwrap();
    let trimmed = s.trim().to_string();
    crate::UnknownType::new(crate::Record_a { init_string: Some(trimmed), ..Default::default() })
}

pub fn Data_String_Common_joinWith(mut sep_val: crate::UnknownType, mut arr_val: crate::UnknownType) -> crate::UnknownType {
    let sep = sep_val.init_string.as_ref().unwrap();
    let arr = arr_val.init_array.as_ref().unwrap();
    let strings: Vec<&str> = arr.iter().map(|item| item.init_string.as_ref().unwrap().as_str()).collect();
    let joined = strings.join(sep);
    crate::UnknownType::new(crate::Record_a { init_string: Some(joined), ..Default::default() })
}
