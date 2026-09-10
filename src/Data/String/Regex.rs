use std::rc::Rc;

/// Immutable compiled regex: calls never share JavaScript's mutable lastIndex.
/// Without `u`, the engine sees one scalar per encoded UTF-16 code unit.
/// Unicode mode decodes complete code points before matching. Isolated
/// surrogates in a `u` pattern return Left; isolated surrogates in `u` input or
/// replacements raise an explicit error. They are never replaced with U+FFFD.
/// Pattern syntax and character-class behavior otherwise follow fancy-regex.
#[derive(Clone)]
pub struct Regex {
    compiled: fancy_regex::Regex,
    source: String,
    flags: String,
}

impl Regex {
    fn has(&self, flag: char) -> bool {
        self.flags.contains(flag)
    }

    fn to_engine(&self, text: &str) -> String {
        if self.has('u') {
            regex_unicode_text(text).expect(
                "Data.String.Regex: Unicode mode does not support isolated UTF-16 surrogates",
            )
        } else {
            text.to_string()
        }
    }

    fn from_engine(&self, text: &str) -> String {
        if self.has('u') {
            purust_core::purust_string_from_utf8(text)
        } else {
            text.to_string()
        }
    }

    fn captures<'s>(&self, text: &'s str, start: usize) -> Option<fancy_regex::Captures<'s>> {
        self.compiled
            .captures_from_pos(text, start)
            .unwrap_or_else(|error| panic!("Data.String.Regex: {error}"))
    }
}

fn regex_unicode_text(text: &str) -> Result<String, String> {
    char::decode_utf16(purust_core::purust_string_to_utf16(text))
        .map(|character| {
            character
                .map_err(|_| "Unicode mode does not support isolated UTF-16 surrogates".to_string())
        })
        .collect()
}

// Translate JS UTF-16 escapes to the engine's character representation. A
// backslash escaped by another backslash remains literal pattern text.
fn regex_pattern(source: &str, unicode: bool) -> Result<String, String> {
    let source = if unicode {
        regex_unicode_text(source)?
    } else {
        source.to_string()
    };
    let chars: Vec<char> = source.chars().collect();
    let mut result = String::new();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] != '\\' || index + 1 == chars.len() {
            result.push(chars[index]);
            index += 1;
            continue;
        }
        if chars[index + 1] == 'u' && index + 5 < chars.len() {
            let digits: String = chars[index + 2..index + 6].iter().collect();
            if let Ok(unit) = u16::from_str_radix(&digits, 16) {
                index += 6;
                let character = if !unicode {
                    purust_core::purust_char_from_code_unit(unit)
                } else if (0xd800..=0xdbff).contains(&unit)
                    && index + 5 < chars.len()
                    && chars[index] == '\\'
                    && chars[index + 1] == 'u'
                {
                    let low: String = chars[index + 2..index + 6].iter().collect();
                    let low = u16::from_str_radix(&low, 16).map_err(|error| error.to_string())?;
                    if !(0xdc00..=0xdfff).contains(&low) {
                        return Err("Invalid Unicode surrogate pair".to_string());
                    }
                    index += 6;
                    char::from_u32(
                        0x10000 + (((unit as u32) - 0xd800) << 10) + (low as u32) - 0xdc00,
                    )
                    .unwrap()
                } else {
                    char::from_u32(unit as u32).ok_or_else(|| {
                        "Unicode mode does not support isolated UTF-16 surrogates".to_string()
                    })?
                };
                result.push_str(&format!("\\x{{{:x}}}", character as u32));
                continue;
            }
        }
        result.push(chars[index]);
        result.push(chars[index + 1]);
        index += 2;
    }
    Ok(result)
}

fn regex_next_position(text: &str, position: usize) -> usize {
    position + text[position..].chars().next().map_or(1, char::len_utf8)
}

fn regex_each_match<'s>(
    regex: &Regex,
    text: &'s str,
    global: bool,
    mut visit: impl FnMut(fancy_regex::Captures<'s>),
) {
    let mut position = 0;
    while position <= text.len() {
        let Some(captures) = regex.captures(text, position) else {
            break;
        };
        let matched = captures
            .get(0)
            .expect("Regex captures include the full match");
        if regex.has('y') && matched.start() != position {
            break;
        }
        let next = if matched.start() == matched.end() {
            regex_next_position(text, matched.end())
        } else {
            matched.end()
        };
        visit(captures);
        if !global {
            break;
        }
        position = next;
    }
}

fn regex_display_source(source: &str) -> String {
    if source.is_empty() {
        return "(?:)".to_string();
    }
    let mut result = String::new();
    let mut escaped = false;
    for character in source.chars() {
        match character {
            '/' if !escaped => result.push_str("\\/"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            _ => result.push(character),
        }
        escaped = character == '\\' && !escaped;
    }
    result
}

pub fn Data_String_Regex_regexImpl(
    left: purust_core::Func1<String, Rc<Purs_Data_Either::Either>>,
    right: purust_core::Func1<Rc<Regex>, Rc<Purs_Data_Either::Either>>,
    source: String,
    flags: String,
) -> Rc<Purs_Data_Either::Either> {
    let mut canonical_flags = String::new();
    for flag in flags.chars() {
        if !"gimsuy".contains(flag) || flags.matches(flag).count() != 1 {
            return left(format!("Invalid regular expression flag: {flag}"));
        }
    }
    for flag in "gimsuy".chars() {
        if flags.contains(flag) {
            canonical_flags.push(flag);
        }
    }
    let engine_source = match regex_pattern(&source, flags.contains('u')) {
        Ok(pattern) => pattern,
        Err(error) => return left(purust_core::purust_string_from_utf8(&error)),
    };
    let modifiers: String = "ims".chars().filter(|flag| flags.contains(*flag)).collect();
    let pattern = if modifiers.is_empty() {
        engine_source
    } else {
        format!("(?{modifiers}){engine_source}")
    };
    match fancy_regex::Regex::new(&pattern) {
        Ok(compiled) => right(Rc::new(Regex {
            compiled,
            source: regex_display_source(&source),
            flags: canonical_flags,
        })),
        Err(error) => left(purust_core::purust_string_from_utf8(&error.to_string())),
    }
}

pub fn Data_String_Regex_showRegexImpl(regex: Rc<Regex>) -> String {
    format!("/{}/{}", regex.source, regex.flags)
}

pub fn Data_String_Regex_source(regex: Rc<Regex>) -> String {
    regex.source.clone()
}

pub fn Data_String_Regex_flagsImpl(regex: Rc<Regex>) -> crate::UnknownType {
    crate::Value::Record_dotAll_global_ignoreCase_multiline_sticky_unicode(
        perceus_ptr::PerceusPtr::new(
            crate::Record_dotAll_global_ignoreCase_multiline_sticky_unicode {
                dotAll: Some(crate::Value::Bool(regex.has('s'))),
                global: Some(crate::Value::Bool(regex.has('g'))),
                ignoreCase: Some(crate::Value::Bool(regex.has('i'))),
                multiline: Some(crate::Value::Bool(regex.has('m'))),
                sticky: Some(crate::Value::Bool(regex.has('y'))),
                unicode: Some(crate::Value::Bool(regex.has('u'))),
            },
        ),
    )
}

pub fn Data_String_Regex_test(regex: Rc<Regex>, text: String) -> bool {
    let text = regex.to_engine(&text);
    regex
        .captures(&text, 0)
        .is_some_and(|captures| !regex.has('y') || captures.get(0).unwrap().start() == 0)
}

fn regex_maybe_value(value: Rc<Purs_Data_Maybe::Maybe>) -> crate::UnknownType {
    crate::Value::Class(Rc::new(value))
}

pub fn Data_String_Regex__match(
    just: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Maybe::Maybe>>,
    nothing: Rc<Purs_Data_Maybe::Maybe>,
    regex: Rc<Regex>,
    text: String,
) -> Rc<Purs_Data_Maybe::Maybe> {
    let text = regex.to_engine(&text);
    let mut matches = Vec::new();
    regex_each_match(&regex, &text, regex.has('g'), |captures| {
        let count = if regex.has('g') { 1 } else { captures.len() };
        for index in 0..count {
            let value = captures.get(index).map_or_else(
                || nothing.clone(),
                |matched| just(crate::Value::String(regex.from_engine(matched.as_str()))),
            );
            matches.push(regex_maybe_value(value));
        }
    });
    if matches.is_empty() {
        nothing
    } else {
        just(crate::mk_array(matches))
    }
}

// JavaScript replacement tokens differ from fancy-regex's `$name` syntax.
fn regex_expand_replacement(
    replacement: &str,
    text: &str,
    captures: &fancy_regex::Captures<'_>,
    named_groups: bool,
) -> String {
    let matched = captures.get(0).unwrap();
    let bytes = replacement.as_bytes();
    let mut result = String::new();
    let mut position = 0;
    while position < bytes.len() {
        if bytes[position] != b'$' || position + 1 == bytes.len() {
            let character = replacement[position..].chars().next().unwrap();
            result.push(character);
            position += character.len_utf8();
            continue;
        }
        match bytes[position + 1] {
            b'$' => result.push('$'),
            b'&' => result.push_str(matched.as_str()),
            b'`' => result.push_str(&text[..matched.start()]),
            b'\'' => result.push_str(&text[matched.end()..]),
            b'0'..=b'9' => {
                let mut group = (bytes[position + 1] - b'0') as usize;
                let mut digits = 1;
                if position + 2 < bytes.len() && bytes[position + 2].is_ascii_digit() {
                    let two_digits = group * 10 + (bytes[position + 2] - b'0') as usize;
                    if two_digits > 0 && two_digits < captures.len() {
                        group = two_digits;
                        digits = 2;
                    }
                }
                if group == 0 || group >= captures.len() {
                    result.push('$');
                    position += 1;
                    continue;
                }
                if let Some(group) = captures.get(group) {
                    result.push_str(group.as_str());
                }
                position += 1 + digits;
                continue;
            }
            b'<' if named_groups => {
                if let Some(end) = replacement[position + 2..].find('>') {
                    let end = position + 2 + end;
                    if let Some(group) = captures.name(&replacement[position + 2..end]) {
                        result.push_str(group.as_str());
                    }
                    position = end + 1;
                    continue;
                }
                result.push('$');
                position += 1;
                continue;
            }
            _ => {
                result.push('$');
                position += 1;
                continue;
            }
        }
        position += 2;
    }
    result
}

pub fn Data_String_Regex_replace(regex: Rc<Regex>, replacement: String, text: String) -> String {
    let text = regex.to_engine(&text);
    let replacement = regex.to_engine(&replacement);
    let mut result = String::new();
    let mut last_end = 0;
    let named_groups = regex.compiled.capture_names().any(|name| name.is_some());
    regex_each_match(&regex, &text, regex.has('g'), |captures| {
        let matched = captures.get(0).unwrap();
        result.push_str(&text[last_end..matched.start()]);
        result.push_str(&regex_expand_replacement(
            &replacement,
            &text,
            &captures,
            named_groups,
        ));
        last_end = matched.end();
    });
    result.push_str(&text[last_end..]);
    regex.from_engine(&result)
}

pub fn Data_String_Regex__replaceBy(
    just: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Maybe::Maybe>>,
    nothing: Rc<Purs_Data_Maybe::Maybe>,
    regex: Rc<Regex>,
    replacement: purust_core::Func2<String, crate::UnknownType, String>,
    text: String,
) -> String {
    let text = regex.to_engine(&text);
    let mut result = String::new();
    let mut last_end = 0;
    regex_each_match(&regex, &text, regex.has('g'), |captures| {
        let matched = captures.get(0).unwrap();
        result.push_str(&text[last_end..matched.start()]);
        let groups = (1..captures.len())
            .map(|index| {
                regex_maybe_value(captures.get(index).map_or_else(
                    || nothing.clone(),
                    |group| just(crate::Value::String(regex.from_engine(group.as_str()))),
                ))
            })
            .collect();
        result.push_str(&regex.to_engine(&replacement(
            regex.from_engine(matched.as_str()),
            crate::mk_array(groups),
        )));
        last_end = matched.end();
    });
    result.push_str(&text[last_end..]);
    regex.from_engine(&result)
}

pub fn Data_String_Regex__search(
    just: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Maybe::Maybe>>,
    nothing: Rc<Purs_Data_Maybe::Maybe>,
    regex: Rc<Regex>,
    text: String,
) -> Rc<Purs_Data_Maybe::Maybe> {
    let text = regex.to_engine(&text);
    match regex.captures(&text, 0) {
        Some(captures) => {
            let start = captures.get(0).unwrap().start();
            if regex.has('y') && start != 0 {
                return nothing;
            }
            just(crate::Value::Int(if regex.has('u') {
                text[..start].encode_utf16().count()
            } else {
                text[..start].chars().count()
            } as i64))
        }
        None => nothing,
    }
}

pub fn Data_String_Regex_split(regex: Rc<Regex>, text: String) -> crate::UnknownType {
    let text = regex.to_engine(&text);
    if text.is_empty() {
        return crate::mk_array(if regex.captures(&text, 0).is_some() {
            Vec::new()
        } else {
            vec![crate::Value::String(String::new())]
        });
    }
    let mut parts = Vec::new();
    let mut last_end = 0;
    let mut position = 0;
    // String.split scans all separators, independently of the regex's g/y flags.
    while position < text.len() {
        let Some(captures) = regex.captures(&text, position) else {
            break;
        };
        let matched = captures.get(0).unwrap();
        if matched.start() == matched.end()
            && (matched.end() == last_end || matched.end() == text.len())
        {
            position = regex_next_position(&text, matched.end());
            continue;
        }
        parts.push(crate::Value::String(
            regex.from_engine(&text[last_end..matched.start()]),
        ));
        for index in 1..captures.len() {
            // JS inserts undefined for unmatched split captures, represented by Unit.
            parts.push(captures.get(index).map_or(crate::Value::Unit, |group| {
                crate::Value::String(regex.from_engine(group.as_str()))
            }));
        }
        last_end = matched.end();
        position = last_end;
    }
    parts.push(crate::Value::String(regex.from_engine(&text[last_end..])));
    crate::mk_array(parts)
}
