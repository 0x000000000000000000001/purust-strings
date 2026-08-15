pub fn Data_Functor_arrayMap(mut f: crate::UnknownType, mut arr: crate::UnknownType) -> crate::UnknownType {
    let mut result = Vec::new();
    let a = arr.init_array.as_ref().unwrap();
    for i in 0..a.len() {
        result.push(f.call.as_ref().unwrap()(a[i].clone()));
    }
    crate::mk_array(result)
}
