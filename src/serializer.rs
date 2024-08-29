use crate::error::{Error, Result};
use serde::{ser, Serialize};

pub fn serialize<T: ser::Serialize>(request: &T) -> String {
    let mut serializer = KvRequestSerializer {
        output: String::new(),
    };

    request
        .serialize(&mut serializer)
        .expect("failed to serialize");
    serializer.output
}

struct KvRequestSerializer {
    output: String,
}

impl<'a> ser::Serializer for &'a mut KvRequestSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeStructVariant = Self;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;

    fn serialize_bool(self, _: bool) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i8(self, _: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, _: i32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, _: u8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u16(self, _: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, _: u32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u64(self, _: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += " ";
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T: ser::Serialize + ?Sized>(self, _: &T) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ser::Serialize + ?Sized>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ser::Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        let req_type = match variant {
            "Set" => Ok("set"),
            "Get" => Ok("get"),
            "Rm" => Ok("rm"),
            _ => Err(Error::InvalidData(String::from("invalid request provided"))),
        }?;

        self.output += "+:";
        self.output += req_type;

        Ok(self)
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut KvRequestSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += ":\n";
        Ok(())
    }
}

#[test]
fn test_serialization_request_struct() {
    use crate::request::Request;

    let get_request = Request::Get {
        key: "get_key_testing".to_owned(),
    };

    let expected_get = "+:get get_key_testing:\n";
    assert_eq!(serialize(&get_request), expected_get);

    let set_request = Request::Set {
        key: "set_key_testing".to_owned(),
        val: "set_val_testing".to_owned(),
    };

    let expected_set = "+:set set_key_testing set_val_testing:\n";
    assert_eq!(serialize(&set_request), expected_set);

    let rm_request = Request::Rm {
        key: "rm_key_testing".to_owned(),
    };

    let expected_rm = "+:rm rm_key_testing:\n";
    assert_eq!(serialize(&rm_request), expected_rm);
}
