use serde::{de, Serializer};

#[macro_export]
macro_rules! params {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            #[allow(unused_mut)]
            let mut map = Map::new();
            $( map.insert($k, $v); )*
            map
        }
    };
    ( $( $k:expr => $v:expr ),+ , ) => {
        params! { $( $k => $v ),* }
    };
}

pub(crate) fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["true", "false"])),
    }
}

pub(crate) fn serialize_bool_to_string<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *b {
        serializer.serialize_str("true")
    } else {
        serializer.serialize_str("false")
    }
}
