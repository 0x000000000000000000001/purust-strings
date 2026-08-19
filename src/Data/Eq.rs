pub fn Data_Eq_eqIntImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.unwrap_int() == a1.unwrap_int();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqNumberImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.unwrap_number() == a1.unwrap_number();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqCharImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.unwrap_char() == a1.unwrap_char();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqStringImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.unwrap_string() == a1.unwrap_string();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqBooleanImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.unwrap_bool() == a1.unwrap_bool();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let arr1 = a0.unwrap_array();
    let arr2 = a1.unwrap_array();
    if arr1.len() != arr2.len() {
        return crate::mk_bool(false);
    }
    for (x, y) in arr1.iter().zip(arr2.iter()) {
        let res = f.unwrap_func()(x.clone()).unwrap_func()(y.clone());
        if !res.unwrap_bool() {
            return crate::mk_bool(false);
        }
    }
    crate::mk_bool(true)
}
