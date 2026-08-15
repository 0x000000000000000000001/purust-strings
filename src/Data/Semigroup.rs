pub fn Data_Semigroup_concatString(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    crate::mk_string(&format!("{}{}", a0.init_string.as_ref().unwrap(), a1.init_string.as_ref().unwrap()))
}

pub fn Data_Semigroup_concatArray(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let mut vec = a0.init_array.as_ref().unwrap().to_vec();
    vec.extend(a1.init_array.as_ref().unwrap().to_vec());
    crate::mk_array(vec)
}
