pub fn Data_Int_toStringAs(mut radix_val: crate::UnknownType, mut n_val: crate::UnknownType) -> crate::UnknownType {
    let radix = radix_val.init_int.unwrap() as u32;
    let n = n_val.init_int.unwrap();
    let s = if radix == 16 {
        format!("{:x}", n)
    } else if radix == 10 {
        format!("{}", n)
    } else if radix == 8 {
        format!("{:o}", n)
    } else if radix == 2 {
        format!("{:b}", n)
    } else {
        panic!("Unsupported radix: {}", radix);
    };
    crate::UnknownType::new(crate::Record_a { init_string: Some(s), ..Default::default() })
}

pub fn Data_Int_toNumber(mut n_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.init_int.unwrap();
    crate::UnknownType::new(crate::Record_a { init_number: Some(n as f64), ..Default::default() })
}

pub fn Data_Int_quot(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_int.unwrap();
    let b = a1.init_int.unwrap();
    crate::mk_int(a / b)
}

pub fn Data_Int_rem(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_int.unwrap();
    let b = a1.init_int.unwrap();
    crate::mk_int(a % b)
}

pub fn Data_Int_pow(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.init_int.unwrap();
    let b = a1.init_int.unwrap();
    crate::mk_int(a.pow(b as u32))
}
