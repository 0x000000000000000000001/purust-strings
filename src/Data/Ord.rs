pub fn Data_Ord_ordIntImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_int.unwrap();
    let b = a1.init_int.unwrap();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordNumberImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_number.unwrap();
    let b = a1.init_number.unwrap();
    if a < b { lt }
    else if a > b { gt }
    else { eq }
}

pub fn Data_Ord_ordCharImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_char.unwrap();
    let b = a1.init_char.unwrap();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordStringImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_string.as_ref().unwrap();
    let b = a1.init_string.as_ref().unwrap();
    match a.cmp(b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordBooleanImpl(mut lt: crate::UnknownType, mut eq: crate::UnknownType, mut gt: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_bool.unwrap();
    let b = a1.init_bool.unwrap();
    match a.cmp(&b) {
        std::cmp::Ordering::Less => lt,
        std::cmp::Ordering::Equal => eq,
        std::cmp::Ordering::Greater => gt,
    }
}

pub fn Data_Ord_ordArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let arr1 = a0.init_array.as_ref().unwrap();
    let arr2 = a1.init_array.as_ref().unwrap();
    let len = std::cmp::min(arr1.len(), arr2.len());
    for i in 0..len {
        let res = f.call.as_ref().unwrap()(arr1[i].clone()).call.as_ref().unwrap()(arr2[i].clone());
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
