use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::de::{Deserialize, Deserializer};

/// Wrapper struct to handle encoding Cbor semantic tags.
#[derive(Deserialize)]
pub struct EncodeCborTag<T: Serialize> {
    __cbor_tag_ser_tag: u64,
    __cbor_tag_ser_data: T,
}

impl<T: Serialize> EncodeCborTag<T> {
    /// Constructs a new `EncodeCborTag`, to wrap your type in a tag.
    pub fn new(tag: u64, value: T) -> Self {
        EncodeCborTag {
            __cbor_tag_ser_tag: tag,
            __cbor_tag_ser_data: value,
        }
    }

    /// Returns the tag.
    pub fn tag(&self) -> u64 {
        self.__cbor_tag_ser_tag
    }

    /// Returns the inner value, consuming the wrapper.
    pub fn value(self) -> T {
        self.__cbor_tag_ser_data
    }
}

impl<T: Serialize> Serialize for EncodeCborTag<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EncodeCborTag", 2)?;
        state.serialize_field("__cbor_tag_ser_tag", &self.__cbor_tag_ser_tag)?;
        state.serialize_field("__cbor_tag_ser_data", &self.__cbor_tag_ser_data)?;
        state.end()
    }
}

/// TaggedString
#[derive(Clone, Debug, PartialEq)]
pub struct TaggedString {
    /// Tag
    pub tag: u64,
    /// Raw String to be tagged
    pub data: String,
}

impl TaggedString {
    /// Returns tag
    pub fn tag(&self) -> u64 {
        return self.tag;
    }
}

impl Serialize for TaggedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        EncodeCborTag::new(self.tag, &self.data).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TaggedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = EncodeCborTag::deserialize(deserializer)?;
        Ok(TaggedString {tag: wrapper.tag(), data: wrapper.value() })
    }
}
