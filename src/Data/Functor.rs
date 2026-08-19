pub fn Data_Functor_arrayMap(mut f: crate::UnknownType, mut arr: crate::UnknownType) -> crate::UnknownType {
    let mut result = Vec::new();
    let a = arr.unwrap_array();
    for i in 0..a.len() {
        result.push(f.unwrap_func()(a[i].clone()));
    }
    crate::mk_array(result)
}
