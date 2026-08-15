pub fn Data_Eq_eqIntImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.init_int.unwrap() == a1.init_int.unwrap();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqNumberImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.init_number.unwrap() == a1.init_number.unwrap();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqCharImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.init_char.unwrap() == a1.init_char.unwrap();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqStringImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.init_string.as_ref().unwrap() == a1.init_string.as_ref().unwrap();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqBooleanImpl(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let eq = a0.init_bool.unwrap() == a1.init_bool.unwrap();
    crate::mk_bool(eq)
}

pub fn Data_Eq_eqArrayImpl(mut f: crate::UnknownType, mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let arr1 = a0.init_array.as_ref().unwrap();
    let arr2 = a1.init_array.as_ref().unwrap();
    if arr1.len() != arr2.len() {
        return crate::mk_bool(false);
    }
    for (x, y) in arr1.iter().zip(arr2.iter()) {
        let res = f.call.as_ref().unwrap()(x.clone()).call.as_ref().unwrap()(y.clone());
        if !res.init_bool.unwrap() {
            return crate::mk_bool(false);
        }
    }
    crate::mk_bool(true)
}
