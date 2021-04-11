use crate::value::Value;
use crate::Direction;
use crate::TraceError;

pub(crate) fn format_buf(buf: &[u8], mut len: i64, max_print_len: usize) -> String {
    if len < 0 {
        return String::from("");
    }

    let mut truncated = false;

    if let Ok(utf8str) = std::str::from_utf8(&buf[..len as usize]) {
        let buf = utf8str;

        if len > max_print_len as i64 {
            len = max_print_len as i64;
            truncated = true;
        }

        format!(
            "{:x?}{}",
            &buf[..len as usize],
            if truncated { "..." } else { "" },
        )
    } else {
        if len > max_print_len as i64 / 3 {
            len = max_print_len as i64 / 3;
            truncated = true;
        }

        format!(
            "{:x?}{}",
            &buf[..len as usize],
            if truncated { "..." } else { "" },
        )
    }
}

pub(crate) fn skip_if_2<T>(d1: &Direction, d2: &Direction, f: T) -> Value
where
    T: Fn() -> Value,
{
    if *d1 == *d2 {
        f()
    } else {
        Value::Skip
    }
}

pub(crate) fn skip_if_3<T>(d1: &Direction, d2: &Direction, mut f: T) -> Result<Value, TraceError>
where
    T: FnMut() -> Result<Value, TraceError>,
{
    if *d1 == *d2 {
        f()
    } else {
        Ok(Value::Skip)
    }
}
