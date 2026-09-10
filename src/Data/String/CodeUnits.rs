// Each internal Rust scalar represents exactly one PureScript UTF-16 code unit.
use std::rc::Rc;
use purust_core::{Func1, Value};
use Purs_Data_Maybe::Maybe;

type StringJust = Func1<Value, Rc<Maybe>>;

fn string_byte_offset(s: &str, index: usize) -> usize {
    s.char_indices().nth(index).map_or(s.len(), |(offset, _)| offset)
}

pub fn Data_String_CodeUnits__charAt(just: StringJust, nothing: Rc<Maybe>, index: i64, s: String) -> Rc<Maybe> {
    if index < 0 { return nothing; }
    s.chars().nth(index as usize).map_or(nothing, |c| just(Value::Char(c)))
}

pub fn Data_String_CodeUnits__toChar(just: StringJust, nothing: Rc<Maybe>, s: String) -> Rc<Maybe> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => just(Value::Char(c)),
        _ => nothing,
    }
}

pub fn Data_String_CodeUnits__indexOf(just: StringJust, nothing: Rc<Maybe>, pattern: String, s: String) -> Rc<Maybe> {
    Data_String_CodeUnits__indexOfStartingAt(just, nothing, pattern, 0, s)
}

pub fn Data_String_CodeUnits__indexOfStartingAt(just: StringJust, nothing: Rc<Maybe>, pattern: String, start: i64, s: String) -> Rc<Maybe> {
    let length = s.chars().count();
    if start < 0 || start as usize > length { return nothing; }
    let offset = string_byte_offset(&s, start as usize);
    match s[offset..].find(pattern.as_str()) {
        Some(found) => just(Value::Int(s[..offset + found].chars().count() as i64)),
        None => nothing,
    }
}

pub fn Data_String_CodeUnits__lastIndexOf(just: StringJust, nothing: Rc<Maybe>, pattern: String, s: String) -> Rc<Maybe> {
    let length = s.chars().count() as i64;
    Data_String_CodeUnits__lastIndexOfStartingAt(just, nothing, pattern, length, s)
}

pub fn Data_String_CodeUnits__lastIndexOfStartingAt(just: StringJust, nothing: Rc<Maybe>, pattern: String, start: i64, s: String) -> Rc<Maybe> {
    let limit = start.clamp(0, s.chars().count() as i64) as usize;
    // The bound limits the match's start, not its end; matches may overlap.
    let found = s.char_indices().map(|(offset, _)| offset).chain(std::iter::once(s.len()))
        .enumerate().take(limit + 1).filter(|(_, offset)| s[*offset..].starts_with(pattern.as_str()))
        .map(|(index, _)| index).last();
    found.map_or(nothing, |index| just(Value::Int(index as i64)))
}

pub fn Data_String_CodeUnits_countPrefix(predicate: Func1<char, bool>, s: String) -> i64 {
    s.chars().take_while(|c| predicate(*c)).count() as i64
}

pub fn Data_String_CodeUnits_length(s: String) -> i64 {
    s.chars().count() as i64
}

pub fn Data_String_CodeUnits_singleton(c: char) -> String { c.to_string() }

pub fn Data_String_CodeUnits_fromCharArray(values: Value) -> String {
    values.unwrap_array().iter().map(|value| value.unwrap_char()).collect()
}

pub fn Data_String_CodeUnits_toCharArray(s: String) -> Value {
    crate::mk_array(s.chars().map(Value::Char).collect())
}

pub fn Data_String_CodeUnits_take(count: i64, s: String) -> String {
    s.chars().take(count.max(0) as usize).collect()
}

pub fn Data_String_CodeUnits_drop(count: i64, s: String) -> String {
    s.chars().skip(count.max(0) as usize).collect()
}

pub fn Data_String_CodeUnits_slice(begin: i64, end: i64, s: String) -> String {
    let length = s.chars().count() as i64;
    let normalize = |index: i64| if index < 0 { (length + index).max(0) } else { index.min(length) };
    let start = normalize(begin) as usize;
    let stop = normalize(end) as usize;
    s.chars().skip(start).take(stop.saturating_sub(start)).collect()
}

pub fn Data_String_CodeUnits_splitAt(index: i64, s: String) -> Value {
    let offset = string_byte_offset(&s, index.max(0) as usize);
    Value::Record_after_before(perceus_ptr::PerceusPtr::new(crate::Record_after_before {
        before: Some(Value::String(s[..offset].to_owned())),
        after: Some(Value::String(s[offset..].to_owned())),
    }))
}
