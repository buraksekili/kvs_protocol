use serde::de;

use crate::error::Result;

/// Deserializes a key from a binary KeyCode representation.
// pub fn deserialize<'a, T: de::Deserialize<'a>>(input: &'a [u8]) -> Result<T> {
//     let mut deserializer = Deserializer::from_bytes(input);
//     let t = T::deserialize(&mut deserializer)?;
//     if !deserializer.input.is_empty() {
//         return errdata!(
//             "unexpected trailing bytes {:x?} at end of key {input:x?}",
//             deserializer.input,
//         );
//     }
//     Ok(t)
// }

/// Deserializes keys from byte slices into a given type. The format is not
/// self-describing, so the caller must provide a concrete type to deserialize
/// into.
pub struct Deserializer<'de> {
    input: &'de [u8],
}
