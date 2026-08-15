use std::rc::Rc;
fn build_regex(r: &crate::UnknownType) -> fancy_regex::Regex {
    let arr = r.init_array.as_ref().unwrap();
    let source = arr[0].init_string.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    
    let mut prepend = String::new();
    if flags.contains('i') { prepend.push('i'); }
    if flags.contains('m') { prepend.push('m'); }
    if flags.contains('s') { prepend.push('s'); }
    
    let pattern = if prepend.is_empty() {
        source.clone()
    } else {
        format!("(?{}){}", prepend, source)
    };
    
    fancy_regex::Regex::new(&pattern).unwrap()
}

pub fn Data_String_Regex_regexImpl(mut left: crate::UnknownType, mut right: crate::UnknownType, mut s1: crate::UnknownType, mut s2: crate::UnknownType) -> crate::UnknownType {
    let source = s1.init_string.as_ref().unwrap();
    let flags = s2.init_string.as_ref().unwrap();
    
    let mut prepend = String::new();
    if flags.contains('i') { prepend.push('i'); }
    if flags.contains('m') { prepend.push('m'); }
    if flags.contains('s') { prepend.push('s'); }
    
    let pattern = if prepend.is_empty() {
        source.clone()
    } else {
        format!("(?{}){}", prepend, source)
    };
    
    match fancy_regex::Regex::new(&pattern) {
        Ok(_) => {
            let arr = vec![s1.clone(), s2.clone()];
            let regex_repr = crate::UnknownType::new(crate::Record_a {
                init_array: Some(Rc::new(arr)),
                ..Default::default()
            });
            crate::UnknownType::new(crate::Record_a {
                tag: right.tag,
                vals: Some(Rc::new(vec![regex_repr])),
                ..Default::default()
            })
        },
        Err(e) => {
            let err_msg = crate::UnknownType::new(crate::Record_a {
                init_string: Some(e.to_string()),
                ..Default::default()
            });
            crate::UnknownType::new(crate::Record_a {
                tag: left.tag,
                vals: Some(Rc::new(vec![err_msg])),
                ..Default::default()
            })
        }
    }
}

pub fn Data_String_Regex_showRegexImpl(mut r: crate::UnknownType) -> crate::UnknownType {
    let arr = r.init_array.as_ref().unwrap();
    let source = arr[0].init_string.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    let formatted = format!("/{}/{}", source, flags);
    crate::UnknownType::new(crate::Record_a {
        init_string: Some(formatted),
        ..Default::default()
    })
}

pub fn Data_String_Regex_source(mut r: crate::UnknownType) -> crate::UnknownType {
    let arr = r.init_array.as_ref().unwrap();
    arr[0].clone()
}

pub fn Data_String_Regex_flagsImpl(mut r: crate::UnknownType) -> crate::UnknownType {
    let arr = r.init_array.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    
    crate::UnknownType::new(crate::Record_a {
        multiline: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('m')), ..Default::default() })),
        ignoreCase: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('i')), ..Default::default() })),
        global: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('g')), ..Default::default() })),
        dotAll: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('s')), ..Default::default() })),
        sticky: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('y')), ..Default::default() })),
        unicode: Some(crate::UnknownType::new(crate::Record_a { init_bool: Some(flags.contains('u')), ..Default::default() })),
        ..Default::default()
    })
}

pub fn Data_String_Regex_test(mut r: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let target = s.init_string.as_ref().unwrap();
    let is_match = re.is_match(target).unwrap();
    crate::UnknownType::new(crate::Record_a {
        init_bool: Some(is_match),
        ..Default::default()
    })
}

pub fn Data_String_Regex__match(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut r: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let target = s.init_string.as_ref().unwrap();
    
    let arr = r.init_array.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    let is_global = flags.contains('g');
    
    if is_global {
        let matches: Vec<_> = re.find_iter(target).map(|m_res| {
            let m = m_res.unwrap();
            let matched_str = m.as_str().to_string();
            let match_val = crate::UnknownType::new(crate::Record_a { init_string: Some(matched_str), ..Default::default() });
            crate::UnknownType::new(crate::Record_a {
                tag: just.tag,
                vals: Some(Rc::new(vec![match_val])),
                ..Default::default()
            })
        }).collect();
        
        if matches.is_empty() {
            nothing.clone()
        } else {
            let res_arr = crate::UnknownType::new(crate::Record_a { init_array: Some(Rc::new(matches)), ..Default::default() });
            crate::UnknownType::new(crate::Record_a {
                tag: just.tag,
                vals: Some(Rc::new(vec![res_arr])),
                ..Default::default()
            })
        }
    } else {
        if let Some(caps) = re.captures(target).unwrap() {
            let mut matches = Vec::new();
            for m_opt in caps.iter() {
                if let Some(m) = m_opt {
                    let matched_str = m.as_str().to_string();
                    let match_val = crate::UnknownType::new(crate::Record_a { init_string: Some(matched_str), ..Default::default() });
                    matches.push(crate::UnknownType::new(crate::Record_a {
                        tag: just.tag,
                        vals: Some(Rc::new(vec![match_val])),
                        ..Default::default()
                    }));
                } else {
                    matches.push(nothing.clone());
                }
            }
            let res_arr = crate::UnknownType::new(crate::Record_a { init_array: Some(Rc::new(matches)), ..Default::default() });
            crate::UnknownType::new(crate::Record_a {
                tag: just.tag,
                vals: Some(Rc::new(vec![res_arr])),
                ..Default::default()
            })
        } else {
            nothing.clone()
        }
    }
}

pub fn Data_String_Regex_replace(mut r: crate::UnknownType, mut s1: crate::UnknownType, mut s2: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let replacement = s1.init_string.as_ref().unwrap();
    let target = s2.init_string.as_ref().unwrap();
    
    let arr = r.init_array.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    let is_global = flags.contains('g');
    
    let result = if is_global {
        re.replace_all(target, replacement.as_str()).to_string()
    } else {
        re.replace(target, replacement.as_str()).to_string()
    };
    
    crate::UnknownType::new(crate::Record_a { init_string: Some(result), ..Default::default() })
}

pub fn Data_String_Regex__replaceBy(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut r: crate::UnknownType, mut f: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let target = s.init_string.as_ref().unwrap();
    
    let arr = r.init_array.as_ref().unwrap();
    let flags = arr[1].init_string.as_ref().unwrap();
    let is_global = flags.contains('g');
    
    let mut result = String::new();
    let mut last_match = 0;
    
    for caps_res in re.captures_iter(target) {
        let caps = caps_res.unwrap();
        let m = caps.get(0).unwrap();
        result.push_str(&target[last_match..m.start()]);
        
        let match_str = m.as_str().to_string();
        let match_val = crate::UnknownType::new(crate::Record_a { init_string: Some(match_str), ..Default::default() });
        
        let mut groups = Vec::new();
        for i in 1..caps.len() {
            if let Some(g) = caps.get(i) {
                let g_str = g.as_str().to_string();
                let g_val = crate::UnknownType::new(crate::Record_a { init_string: Some(g_str), ..Default::default() });
                groups.push(crate::UnknownType::new(crate::Record_a {
                    tag: just.tag,
                    vals: Some(Rc::new(vec![g_val])),
                    ..Default::default()
                }));
            } else {
                groups.push(nothing.clone());
            }
        }
        let groups_arr = crate::UnknownType::new(crate::Record_a { init_array: Some(Rc::new(groups)), ..Default::default() });
        
        let replacement_val = f.call.as_ref().unwrap()(match_val).call.as_ref().unwrap()(groups_arr);
        result.push_str(replacement_val.init_string.as_ref().unwrap());
        
        last_match = m.end();
        
        if !is_global {
            break;
        }
    }
    
    result.push_str(&target[last_match..]);
    crate::UnknownType::new(crate::Record_a { init_string: Some(result), ..Default::default() })
}

pub fn Data_String_Regex__search(mut just: crate::UnknownType, mut nothing: crate::UnknownType, mut r: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let target = s.init_string.as_ref().unwrap();
    
    if let Some(m) = re.find(target).unwrap() {
        let char_idx = target[..m.start()].chars().count() as i64;
        let int_val = crate::UnknownType::new(crate::Record_a { init_int: Some(char_idx), ..Default::default() });
        crate::UnknownType::new(crate::Record_a {
            tag: just.tag,
            vals: Some(Rc::new(vec![int_val])),
            ..Default::default()
        })
    } else {
        nothing.clone()
    }
}

pub fn Data_String_Regex_split(mut r: crate::UnknownType, mut s: crate::UnknownType) -> crate::UnknownType {
    let re = build_regex(&r);
    let target = s.init_string.as_ref().unwrap();
    
    let mut parts = Vec::new();
    let mut last_match = 0;
    
    for m_res in re.find_iter(target) {
        let m = m_res.unwrap();
        parts.push(crate::UnknownType::new(crate::Record_a {
            init_string: Some(target[last_match..m.start()].to_string()),
            ..Default::default()
        }));
        last_match = m.end();
    }
    parts.push(crate::UnknownType::new(crate::Record_a {
        init_string: Some(target[last_match..].to_string()),
        ..Default::default()
    }));
    
    crate::UnknownType::new(crate::Record_a { init_array: Some(Rc::new(parts)), ..Default::default() })
}
