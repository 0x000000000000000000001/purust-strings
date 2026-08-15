pub fn Test_Assert_assertImpl(mut msg_val: crate::UnknownType, mut success_val: crate::UnknownType) -> crate::UnknownType {
    if !success_val.init_bool.unwrap() {
        eprintln!("Assertion failed: {}", msg_val.init_string.as_ref().unwrap());
    }
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Test_Assert_checkThrows(mut fn_val: crate::UnknownType) -> crate::UnknownType {
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        fn_val.call.as_ref().unwrap()(crate::UnknownType::new(crate::Record_a { ..Default::default() }))
    }));
    match res {
        Ok(_) => crate::mk_bool(false),
        Err(_) => crate::mk_bool(true),
    }
}
