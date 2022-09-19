use chrono::prelude::*;
use serde_json::Value;

// Assuming passed string is of format mm-dd-yyyy
pub fn transform_raw_date(raw_date: &str) -> Result<i64, ()> {
    let wrapped_date =
        NaiveDateTime::parse_from_str(&format!("{} 0:0:0", raw_date), "%Y-%m-%d %H:%M:%S");

    if wrapped_date.is_err() {
        return Err(());
    }

    let date = wrapped_date.unwrap().timestamp();

    Ok(date)
}

pub fn get_current_timestamp() -> i64 {
    Utc::now().naive_utc().timestamp()
}

fn inner_extractor<'a>(value: &'a Value, path: &'_ str) -> Option<&'a Value> {
    if let (Some(array), Ok(idx)) = (value.as_array(), path.parse::<usize>()) {
        return array.get(idx);
    }

    value.get(path)
}

pub fn extract_value_json(value: &Value, path: &str) -> Option<Value> {
    let mut split = path.split('.');

    let initial_part = split.next()?;
    let initial_value = inner_extractor(value, initial_part)?;

    let mut result = Some(initial_value.to_owned());

    for part in split {
        if let Some(next) = inner_extractor(&result.unwrap(), part) {
            result = Some(next.to_owned());
        } else {
            result = None;
            break;
        }
    }

    result
}
