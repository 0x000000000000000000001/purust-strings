pub fn Data_Ord_ordIntImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_int();
    let b = a1.unwrap_int();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordNumberImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_number();
    let b = a1.unwrap_number();
    if a < b { lt }
    else if a > b { gt }
    else { eq }
}

pub fn Data_Ord_ordCharImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_char();
    let b = a1.unwrap_char();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordStringImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_string();
    let b = a1.unwrap_string();
    match a.cmp(b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordBooleanImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_bool();
    let b = a1.unwrap_bool();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let arr1 = a0.unwrap_array();
    let arr2 = a1.unwrap_array();
    let len = std::cmp::min(arr1.len(), arr2.len());
    for i in 0..len {
        let res = f.unwrap_func()(arr1[i].clone()).unwrap_func()(arr2[i].clone());
        if res.tag == "LT" {
            return res;
        } else if res.tag == "GT" {
            return res;
        }
    }
    match arr1.len().cmp(&arr2.len()) {
        std::cmp::Ordering::Less => crate::UnknownType::new(crate::Record_a { tag: "LT", ..Default::default() }),
        std::cmp::Ordering::Equal => crate::UnknownType::new(crate::Record_a { tag: "EQ", ..Default::default() }),
        std::cmp::Ordering::Greater => crate::UnknownType::new(crate::Record_a { tag: "GT", ..Default::default() }),
    }
}
