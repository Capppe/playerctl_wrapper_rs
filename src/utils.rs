use dbus::arg::{PropMap, RefArg};

pub fn get_string(value: &dyn RefArg) -> Option<String> {
    value.as_str().map(|s| s.to_owned())
}

pub fn get_i64(value: &dyn RefArg) -> Option<i64> {
    if let Some(val) = value.as_u64() {
        Some(val.try_into().unwrap_or(0))
    } else if let Some(val) = value.as_f64() {
        Some(val.as_i64()?)
    } else if let Some(val) = value.as_i64() {
        Some(val)
    } else {
        Some(0)
    }
}

pub fn get_f64(value: &dyn RefArg) -> Option<f64> {
    value.as_f64()
}

pub fn get_bool(value: &dyn RefArg) -> Option<bool> {
    value.as_u64().map(|b| b == 1)
}

pub fn get_string_vec(value: &dyn RefArg) -> Option<Vec<String>> {
    if let Some(iter) = value.as_iter() {
        let mut vec = Vec::new();
        for v in iter {
            if let Some(s) = v.as_str() {
                vec.push(s.to_owned());
            } else if let Some(inner_iter) = v.as_iter() {
                for inner_v in inner_iter {
                    if let Some(inner_s) = inner_v.as_str() {
                        vec.push(inner_s.to_owned());
                    }
                }
            }
        }
        Some(vec)
    } else {
        None
    }
}

pub fn find_in_propmap<T: 'static>(map: PropMap, key: &str) -> Result<T, String>
where
    T: std::any::Any + Clone,
{
    match map.get(key) {
        Some(variant) => {
            if let Some(v) = variant.0.as_ref().as_any().downcast_ref::<T>() {
                return Ok(v.clone());
            } else {
                Err(format!("Failed to cast to the wanted type"))
            }
        }
        None => Err(format!("Failed to find key '{}' in propmap", key)),
    }
}
