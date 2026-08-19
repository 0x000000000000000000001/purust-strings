pub fn Data_Int_toStringAs(mut radix_val: crate::UnknownType, mut n_val: crate::UnknownType) -> crate::UnknownType {
    let radix = radix_val.unwrap_int() as u32;
    let n = n_val.unwrap_int();
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
    crate::Value::String(s)
}

pub fn Data_Int_toNumber(mut n_val: crate::UnknownType) -> crate::UnknownType {
    let n = n_val.unwrap_int();
    crate::Value::Number(n as f64)
}

pub fn Data_Int_quot(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_int();
    let b = a1.unwrap_int();
    crate::mk_int(a / b)
}

pub fn Data_Int_rem(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_int();
    let b = a1.unwrap_int();
    crate::mk_int(a % b)
}

pub fn Data_Int_pow(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let a = a0.unwrap_int();
    let b = a1.unwrap_int();
    crate::mk_int(a.pow(b as u32))
}

pub fn Data_Int_fromNumberImpl(
    mut just: crate::UnknownType,
    mut nothing: crate::UnknownType,
    mut n: crate::UnknownType,
) -> crate::UnknownType {
    let num = n.unwrap_number();
    if num.is_finite() && num.fract() == 0.0 && num >= (-2147483648.0) && num <= (2147483647.0) {
        let int_val = num as i64;
        crate::UnknownType::new(crate::Record_a {
            tag: "Just",
            vals: Some(std::rc::Rc::new(vec![crate::mk_int(int_val)])),
            ..Default::default()
        })
    } else {
        eprintln!("fromNumberImpl returning Nothing for num={}", num);
        crate::UnknownType::new(crate::Record_a {
            tag: "Nothing",
            vals: None,
            ..Default::default()
        })
    }
}
