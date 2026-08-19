// FFI implementation for Data.String.Common in Rust

pub fn Data_String_Common__localeCompare(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a_val: crate::UnknownType, mut b_val: crate::UnknownType) -> crate::UnknownType {
    let a = a_val.unwrap_string();
    let b = b_val.unwrap_string();
    match a.cmp(b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_String_Common_replace(mut pat_val: crate::UnknownType, mut rep_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.unwrap_string();
    let rep = rep_val.unwrap_string();
    let s = s_val.unwrap_string();
    let replaced = s.replacen(pat, rep, 1);
    crate::Value::String(replaced)
}

pub fn Data_String_Common_replaceAll(mut pat_val: crate::UnknownType, mut rep_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.unwrap_string();
    let rep = rep_val.unwrap_string();
    let s = s_val.unwrap_string();
    let replaced = s.replace(pat, rep);
    crate::Value::String(replaced)
}

pub fn Data_String_Common_split(mut pat_val: crate::UnknownType, mut s_val: crate::UnknownType) -> crate::UnknownType {
    let pat = pat_val.unwrap_string();
    let s = s_val.unwrap_string();
    let parts: Vec<crate::UnknownType> = if pat.is_empty() {
        if s.is_empty() {
            vec![]
        } else {
            s.chars().map(|c| crate::Value::String(c.to_string())).collect()
        }
    } else {
        s.split(pat).map(|part| crate::Value::String(part.to_string())).collect()
    };
    crate::Value::Array(std::rc::Rc::new(parts))
}

pub fn Data_String_Common_toLower(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let lower = s.to_lowercase();
    crate::Value::String(lower)
}

pub fn Data_String_Common_toUpper(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let upper = s.to_uppercase();
    crate::Value::String(upper)
}

pub fn Data_String_Common_trim(mut s_val: crate::UnknownType) -> crate::UnknownType {
    let s = s_val.unwrap_string();
    let trimmed = s.trim().to_string();
    crate::Value::String(trimmed)
}

pub fn Data_String_Common_joinWith(mut sep_val: crate::UnknownType, mut arr_val: crate::UnknownType) -> crate::UnknownType {
    let sep = sep_val.unwrap_string();
    let arr = arr_val.unwrap_array();
    let strings: Vec<&str> = arr.iter().map(|item| item.unwrap_string().as_str()).collect();
    let joined = strings.join(sep);
    crate::Value::String(joined)
}
