use serde::de::{self};

use crate::{
    error::{Error, Result},
    request::Request,
};

pub struct RequestVisitor;

impl<'de> de::Visitor<'de> for RequestVisitor {
    type Value = Request;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a command in the format ':<cmd> <required_key> <optional_value>'")
    }

    fn visit_str<E>(self, cmd_str: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        let inputs = cmd_str.split_whitespace().collect::<Vec<&str>>();
        let len = inputs.len();
        if len != 2 && len != 3 {
            return Err(de::Error::custom(
                "invalid command request provided, valid commands are 'get', 'set' and 'rm'",
            ));
        }

        let get_key = |key: &str| -> String {
            let trimmed = key.trim();

            if trimmed.ends_with(":") {
                return trimmed
                    .get(0..trimmed.len() - 1)
                    .unwrap_or(trimmed)
                    .to_owned();
            }

            return trimmed.to_owned();
        };

        let mut cmd_name = inputs[0];
        if cmd_name.starts_with("+:") {
            cmd_name = cmd_name.trim().get(2..cmd_name.len()).unwrap_or(cmd_name);
        }

        let key = inputs[1];

        match cmd_name {
            "get" => Ok(Request::Get { key: get_key(key) }),
            "rm" => Ok(Request::Rm { key: get_key(key) }),
            "set" => {
                let val = inputs[2].trim();

                Ok(Request::Set {
                    key: key.to_owned(),
                    val: get_key(val),
                })
            }
            _ => Err(de::Error::custom(format!(
                "invalid command is provided, valid commands are 'get', 'set' and 'rm', but got {}",
                cmd_name
            ))),
        }
    }
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        seq map struct enum identifier ignored_any bytes
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str::<Self::Error>(&self.input)
    }

    fn deserialize_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::custom(
            "unsupported type provided to deserializer, only str is supported",
        ))
    }
}

pub fn deserialize<'a, T: de::Deserialize<'a>>(input: &'a str) -> Result<T> {
    let mut deserializer = Deserializer::from_str(input);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;

    #[test]
    fn test_deserialize_get() {
        let data = r"get abc:";
        let expected = Request::Get {
            key: "abc".to_string(),
        };

        let result: Request = deserialize(data).expect("failed to deserialize");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_deserialize_set() {
        let data = r"set burak 123:";
        let expected = Request::Set {
            key: "burak".to_string(),
            val: "123".to_string(),
        };

        let result: Request = deserialize(data).expect("failed to deserialize");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_deserialize_rm() {
        let data = r"rm burak:";
        let expected = Request::Rm {
            key: "burak".to_string(),
        };

        let result: Request = deserialize(data).expect("failed to deserialize");

        assert_eq!(expected, result);
    }
}
