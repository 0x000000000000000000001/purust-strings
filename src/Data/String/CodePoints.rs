use std::rc::Rc;
use purust_core::{Func1, Func2, Value, purust_char_from_code_unit, purust_char_to_code_unit};
use Purs_Data_Maybe::Maybe;

// Pair only a leading surrogate followed immediately by a trailing surrogate.
// Isolated units remain observable code points, exactly as in PureScript.
fn code_point_segments(s: &str) -> impl std::iter::Iterator<Item = &str> {
    let mut positions = s.char_indices().peekable();
    std::iter::from_fn(move || {
        let (start, first) = positions.next()?;
        let first = purust_char_to_code_unit(first);
        if (0xd800..=0xdbff).contains(&first) {
            if let Some((_, next)) = positions.peek() {
                if (0xdc00..=0xdfff).contains(&purust_char_to_code_unit(*next)) { positions.next(); }
            }
        }
        let end = positions.peek().map_or(s.len(), |(offset, _)| *offset);
        Some(&s[start..end])
    })
}

pub fn Data_String_CodePoints__unsafeCodePointAt0(_fallback: Func1<String, i64>, s: String) -> i64 {
    let mut units = s.chars().map(purust_char_to_code_unit);
    let first = units.next().expect("unsafeCodePointAt0 on empty string");
    if (0xd800..=0xdbff).contains(&first) {
        if let Some(second) = units.next() {
            if (0xdc00..=0xdfff).contains(&second) {
                return 0x10000 + ((first as i64 - 0xd800) << 10) + second as i64 - 0xdc00;
            }
        }
    }
    first as i64
}

pub fn Data_String_CodePoints__codePointAt(_fallback: Func2<i64, String, Rc<Maybe>>, just: Func1<Value, Rc<Maybe>>, nothing: Rc<Maybe>, read: Func1<String, i64>, index: i64, s: String) -> Rc<Maybe> {
    if index < 0 { return nothing; }
    code_point_segments(&s).nth(index as usize).map_or(nothing, |segment| just(Value::Int(read(segment.to_owned()))))
}

pub fn Data_String_CodePoints__countPrefix(_fallback: Func2<Func1<i64, bool>, String, i64>, read: Func1<String, i64>, predicate: Func1<i64, bool>, s: String) -> i64 {
    code_point_segments(&s).take_while(|segment| predicate(read((*segment).to_owned()))).count() as i64
}

fn append_code_point(result: &mut String, point: i64) {
    assert!((0..=0x10ffff).contains(&point), "Code point out of range");
    if point <= 0xffff {
        result.push(purust_char_from_code_unit(point as u16));
    } else {
        let value = point as u32 - 0x10000;
        result.push(purust_char_from_code_unit((0xd800 + (value >> 10)) as u16));
        result.push(purust_char_from_code_unit((0xdc00 + (value & 0x3ff)) as u16));
    }
}

pub fn Data_String_CodePoints__fromCodePointArray(_singleton: Func1<i64, String>, points: Value) -> String {
    let mut result = String::new();
    for point in points.unwrap_array().iter() { append_code_point(&mut result, point.unwrap_int()); }
    result
}

pub fn Data_String_CodePoints__singleton(_fallback: Func1<i64, String>, point: i64) -> String {
    let mut result = String::new();
    append_code_point(&mut result, point);
    result
}

pub fn Data_String_CodePoints__take(_fallback: Func2<i64, String, String>, count: i64, s: String) -> String {
    code_point_segments(&s).take(count.max(0) as usize).collect()
}

pub fn Data_String_CodePoints__toCodePointArray(_fallback: Func1<String, Value>, read: Func1<String, i64>, s: String) -> Value {
    crate::mk_array(code_point_segments(&s).map(|segment| Value::Int(read(segment.to_owned()))).collect())
}
