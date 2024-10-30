pub mod domains;
pub mod error;
mod tests;

use common::error::CustomErrorMessage;

use strum_macros::EnumDiscriminants;

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

pub fn main() {}
