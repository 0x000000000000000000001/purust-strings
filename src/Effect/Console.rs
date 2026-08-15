pub fn Effect_Console_log(mut a0: crate::UnknownType) -> crate::UnknownType {
    println!("{}", a0.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_error(mut a0: crate::UnknownType) -> crate::UnknownType {
    eprintln!("{}", a0.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_warn(mut a0: crate::UnknownType) -> crate::UnknownType {
    eprintln!("WARN: {}", a0.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_info(mut a0: crate::UnknownType) -> crate::UnknownType {
    println!("INFO: {}", a0.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_debug(mut a0: crate::UnknownType) -> crate::UnknownType {
    println!("DEBUG: {}", a0.init_string.as_ref().unwrap());
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_clear() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_group(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_groupCollapsed(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_groupEnd() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_time(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_timeEnd(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}

pub fn Effect_Console_timeLog(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { ..Default::default() })
}
