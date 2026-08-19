pub fn Data_Semigroup_concatString(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    crate::mk_string(&format!("{}{}", a0.unwrap_string(), a1.unwrap_string()))
}

pub fn Data_Semigroup_concatArray(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType {
    let mut vec = a0.unwrap_array().to_vec();
    vec.extend(a1.unwrap_array().to_vec());
    crate::mk_array(vec)
}
