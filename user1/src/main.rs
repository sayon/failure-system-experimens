mod tests;

use common::error::CustomErrorMessage;
use common::error::ICustomError;
use common::error::IUnifiedError;
use common::identifier::Identifier;
use common::kind::CompilerSubdomain;
use common::kind::EraSubdomain;
use common::kind::Kind;
use common::kind::ToolingSubdomain;
use common::packed::pack;
use common::packed::serialized;
use common::packed::PackedError;
use common::serialized::unpack_typed;
use common::serialized::unpack_untyped;
use common::serialized::SerializedError;
use serde_json::json;
use strum_macros::EnumDiscriminants;

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {
    CompilerError(CompilerError),
    ToolingError(ToolingError),
}

impl ZksyncError {
    pub fn get_kind(&self) -> common::kind::Kind {
        match self {
            ZksyncError::CompilerError(compiler_error) => Kind::Compiler(match compiler_error {
                CompilerError::Zksolc(_) => CompilerSubdomain::Zksolc,
                CompilerError::Solc(_) => CompilerSubdomain::Solc,
            }),
            ZksyncError::ToolingError(tooling_error) => Kind::Tooling(match tooling_error {
                ToolingError::RustSDK(_) => ToolingSubdomain::RustSDK,
            }),
        }
    }
    pub fn get_code(&self) -> i32 {
        match self {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => {
                    Into::<ZksolcErrorCode>::into(zksolc_error) as i32
                }
                CompilerError::Solc(solc_error) => Into::<SolcErrorCode>::into(solc_error) as i32,
            },
            ZksyncError::ToolingError(tooling_error) => match tooling_error {
                ToolingError::RustSDK(rust_sdkerror) => {
                    Into::<RustSDKErrorCode>::into(rust_sdkerror) as i32
                }
            },
        }
    }
}

impl IUnifiedError for ZksyncError {
    fn get_identifier(&self) -> Identifier {
        Identifier {
            kind: self.get_kind(),
            code: self.get_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => zksolc_error.get_message(),
                CompilerError::Solc(solc_error) => solc_error.get_message(),
            },
            ZksyncError::ToolingError(tooling_error) => match tooling_error {
                ToolingError::RustSDK(rust_sdkerror) => rust_sdkerror.get_message(),
            },
        }
    }
}
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerSubdomainCode))]
#[strum_discriminants(vis(pub))]
pub enum CompilerError {
    Zksolc(ZksolcError),
    Solc(SolcError),
}

#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingSubdomainCode))]
#[strum_discriminants(vis(pub))]
pub enum ToolingError {
    RustSDK(RustSDKError),
}

impl ICustomError<ZksyncError> for ZksolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Zksolc(self.clone()))
    }
}
impl ICustomError<ZksyncError> for SolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Solc(self.clone()))
    }
}
impl ICustomError<ZksyncError> for RustSDKError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::ToolingError(ToolingError::RustSDK(self.clone()))
    }
}

// --- Should be autogenerated from common JSON +AND+ a repository-local JSONs. Doubles are merged. ---
#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(RustSDKErrorCode))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum RustSDKError {
    WrongTool { info: String } = 1,
}

#[non_exhaustive]
// non_exhaustive should be removed when the API stabilizes and the migration to the new system is complete.
#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ZksolcErrorCode))]
#[strum_discriminants(vis(pub))]
pub enum ZksolcError {
    Generic {
        filename: String,
        line: i32,
        column: i32,
    } = 42,
}

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(SolcErrorCode))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum SolcError {
    SomeError(String) = 1,
}

impl CustomErrorMessage for SolcError {
    fn get_message(&self) -> String {
        match self {
            SolcError::SomeError(e) => format!("This is a solc error {}", e),
        }
    }
}

impl CustomErrorMessage for ZksolcError {
    fn get_message(&self) -> String {
        match self {
            ZksolcError::Generic {
                filename,
                line,
                column,
            } => format!(
                "Some error in zksolc when processing  {} line {} col {}",
                filename, line, column
            ),
        }
    }
}

impl CustomErrorMessage for RustSDKError {
    fn get_message(&self) -> String {
        match self {
            RustSDKError::WrongTool { info } => format!("Wrong tool or smth: {}", info),
        }
    }
}
// -------------------------------------------------------------

pub fn thrower_known() -> Result<(), PackedError<ZksyncError>> {
    Err(pack(ZksolcError::Generic {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    }))
}
pub fn thrower_known_serialized() -> Result<(), SerializedError> {
    Err(serialized(&pack(ZksolcError::Generic {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    })))
}

pub fn handle_known() {
    let received_error: PackedError<ZksyncError> = thrower_known().unwrap_err();
    let typed_error = &received_error.data;
    match typed_error {
        ZksyncError::CompilerError(compiler_error) => match &compiler_error {
            CompilerError::Zksolc(zksolc_error) => match &zksolc_error {
                ZksolcError::Generic { .. } => {
                    println!("Caught known error: {:#?}", &typed_error);
                    println!(
                        "Don't have to use json to work with this error: {:} ",
                        &received_error
                    );
                }
                _ => todo!(),
            },
            CompilerError::Solc(_) => todo!(),
        },
        ZksyncError::ToolingError(_) => todo!(),
    }
}

pub fn handle_known_serialized(received_error: &SerializedError) {
    if let Ok(typed_error) = unpack_typed::<ZksyncError>(&received_error) {
        match &typed_error {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => match zksolc_error {
                    ZksolcError::Generic { .. } => {
                        println!("Caught known error: {:#?}", &typed_error);
                        println!(
                            "Don't have to use json to work with this error: {:} ",
                            &received_error
                        );
                    }
                    _ => todo!(),
                },
                CompilerError::Solc(_) => todo!(),
            },
            ZksyncError::ToolingError(_) => todo!(),
        }
    } else {
        println!("Use json to work with this error: {:} ", &received_error);
    }
}

pub fn thrower_unknown() -> Result<(), SerializedError> {
    Err(SerializedError::new_custom(
        Kind::Era(EraSubdomain::VM),
        242,
        "Message does not matter -- except for a possible prefix.",
        json!(
            { "EraError" : { "VM" : { "SomeVMError" : { "somefield" : "somevalue" } } } }
        ),
    ))
}

pub fn handle_unknown_serialized(received_error: &SerializedError) {
    if let Ok(_) = unpack_typed::<ZksyncError>(&received_error) {
        unreachable!()
    } else {
        let error_object = unpack_untyped(received_error).unwrap();

        println!("Error object: {:#?}", error_object);
        let o = error_object.get_object();
        dbg!(&o);
        println!("Field 'somefield' is equal to: {:?}", o.get("somefield"));
    }
}

pub fn main() {
    handle_known();

    let received_error: SerializedError = thrower_known_serialized().unwrap_err();
    handle_known_serialized(&received_error);

    let received_error: SerializedError = thrower_unknown().unwrap_err();
    handle_unknown_serialized(&received_error);
}
