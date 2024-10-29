use crate::{
    error::IUnifiedError, identifier::Identifier, kind::Kind, packed::PackedError, untyped::UntypedErrorObject
};

pub type ErrorCode = i32;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerializedError {
    pub code: ErrorCode,
    pub message: String,
    pub data: serde_json::Value,
}

impl SerializedError {
    pub fn new_custom(kind: Kind, code: ErrorCode, message: impl Into<String>, unified_error_json: serde_json::Value) -> Self {
        SerializedError {
            code: Identifier::new(kind, code).encode(),
            message: message.into(),
            data: unified_error_json
        }
    }
}
impl std::fmt::Display for SerializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ code: {}; message: \"{}\"; data: \"{}\"}}",
            self.code, self.message, self.data
        ))
    }
}

pub fn serialize<U>(error: PackedError<U>) -> Result<SerializedError, serde_json::Error>
where
    U: serde::Serialize,
{
    Ok(SerializedError {
        code: error.identifier.encode(),
        message: error.message,
        data: serde_json::value::to_value(&error.data)?,
    })
}
pub fn serialize_ref<U>(error: &PackedError<U>) -> Result<SerializedError, serde_json::Error>
where
    U: serde::Serialize,
{
    Ok(SerializedError {
        code: error.identifier.encode(),
        message: error.message.clone(),
        data: serde_json::value::to_value(&error.data)?,
    })
}

pub fn unpack_untyped(se: &SerializedError) -> Result<UntypedErrorObject, serde_json::Error> {
    //FIXME unhandled errors
    let identifier = Identifier::decode(se.code).unwrap();
    let skip_domain = se.data.as_object().unwrap().values().nth(0).unwrap();
    let skip_subdomain = skip_domain.as_object().unwrap().values().nth(0).unwrap();
    let (name,value) = skip_subdomain.as_object().unwrap().iter().nth(0).unwrap();
    let fields: serde_json::Map<String, serde_json::Value> = value.as_object().unwrap().clone();
    Ok(UntypedErrorObject { identifier, name: name.clone(), fields, raw: se.data.clone() } )
}

pub fn unpack_typed<T>(se: &SerializedError) -> Result<T, serde_json::Error>
where
    T: IUnifiedError + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    serde_json::value::from_value(se.data.clone())
}
