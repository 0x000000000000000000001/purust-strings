pub fn Data_EuclideanRing_intDegree(mut a0: crate::UnknownType) -> crate::UnknownType {
    let x = a0.init_int.unwrap();
    let abs_x = x.abs();
    crate::mk_int(std::cmp::min(abs_x, 2147483647))
}

pub fn Data_EuclideanRing_intDiv(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let x = a0.init_int.unwrap();
    let y = a1.init_int.unwrap();
    if y == 0 {
        return crate::mk_int(0);
    }
    // div_euclid handles negative operands differently than Math.floor sometimes?
    // Let's implement the exact logic from PureScript's JS:
    // y > 0 ? Math.floor(x / y) : -Math.floor(x / -y)
    let res = if y > 0 {
        // Rust's div_euclid for positive divisor behaves like Math.floor
        x.div_euclid(y)
    } else {
        -x.div_euclid(-y)
    };
    crate::mk_int(res)
}

pub fn Data_EuclideanRing_intMod(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let x = a0.init_int.unwrap();
    let y = a1.init_int.unwrap();
    if y == 0 {
        return crate::mk_int(0);
    }
    let yy = y.abs();
    let res = ((x % yy) + yy) % yy;
    crate::mk_int(res)
}

pub fn Data_EuclideanRing_numDiv(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let x = a0.init_number.unwrap();
    let y = a1.init_number.unwrap();
    crate::mk_number(x / y)
}
