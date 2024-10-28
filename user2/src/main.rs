use common2::error_types::description::pack;
use common2::error_types::description::IErrorDescription;
use common2::error_types::identifier::CompilerSubdomain;
use common2::error_types::identifier::DomainPath;
use common2::error_types::identifier::ErrorIdentifier;
use common2::error_types::packed::ErrorCode;
use common2::error_types::packed::PackedError;
use strum_macros::{EnumDiscriminants, FromRepr};

#[repr(u32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(Domain))]
#[strum_discriminants(derive(FromRepr))]
pub enum ZksyncError {
    Compiler(CompilerError) = 1,
    Tooling(ToolingError) = 2,
}

#[repr(u32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerSubdomainCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum CompilerError {
    Zksolc(ZksolcError) = 1,
    Solc(SolcError) = 2,
}

trait Subdomain {
    fn get_domain() -> Domain;
}

impl Subdomain for CompilerError {
    fn get_domain() -> Domain {
        Domain::Compiler
    }
}

impl Subdomain for CompilerSubdomainCode {
    fn get_domain() -> Domain {
        Domain::Compiler
    }
}

#[repr(u32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingSubdomainCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum ToolingError {
    RustSDK(RustSDKError) = 1,
}

impl Subdomain for ToolingError {
    fn get_domain() -> Domain {
        Domain::Tooling
    }
}

impl Subdomain for ToolingSubdomainCode {
    fn get_domain() -> Domain {
        Domain::Tooling
    }
}

// --- Should be autogenerated from common JSON +AND+ a repository-local JSONs. Doubles are merged. ---
#[repr(u32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingSomeSubdomainErrorCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum RustSDKError {
    WrongTool { info: String } = 1,
}

#[non_exhaustive]
// non_exhaustive should be removed when the API stabilizes and the migration to the new system is complete.
#[repr(u32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ZksolcErrorCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum ZksolcError {
    GenericError {
        filename: String,
        line: u32,
        column: u32,
    } = 42,
}

impl ZksolcError {
    pub fn get_discriminant(&self) -> ZksolcErrorCode {
        self.clone().into()
    }
}
#[repr(u32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(SolcErrorCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum SolcError {
    SomeError(String),
}

impl SolcError {
    pub fn get_discriminant(&self) -> SolcErrorCode {
        self.clone().into()
    }
}
// -----
//
impl IErrorDescription for ZksolcError {
    fn get_identifier(&self) -> ErrorIdentifier {
        let code = self.get_discriminant() as ErrorCode;
        let path = DomainPath::Compiler(CompilerSubdomain::Zksolc);
        ErrorIdentifier { path, code }
    }

    fn get_message(&self) -> String {
        match self {
            ZksolcError::GenericError {
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

pub fn typed_error(err: &PackedError) -> Result<ZksyncError, serde_json::Error> {
    let PackedError { data, .. } = err;
    serde_json::from_value(data.clone())
}

pub fn thrower_known() -> Result<(), PackedError> {
    Err(pack(ZksolcError::GenericError {
        filename: "some_filename".to_string(),
        line: 10,
        column: 42,
    }))
}

pub fn catch_known() {
    let received_error = thrower_known().unwrap_err();
    if let Ok(typed_error) = typed_error(&received_error) {
        match typed_error {
            ZksyncError::Compiler(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => match zksolc_error {
                    ZksolcError::GenericError { .. } => (),
                    _ => todo!(),
                },
                CompilerError::Solc(_) => todo!(),
            },
            ZksyncError::Tooling(_) => todo!(),
        }
    } else {
        println!("Use json to work with this error: {} ", &received_error);
    }
}
pub fn main() {}
