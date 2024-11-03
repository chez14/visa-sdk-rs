use derive_builder::UninitializedFieldError;
use derive_more::From;

pub type BuilderResult<T> = core::result::Result<T, BuilderError>;

#[derive(Debug, From)]
pub enum BuilderError {
    ValidationViolition(String),

    // -- Externals
    #[from]
    UninitializedField(UninitializedFieldError),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for BuilderError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for BuilderError {}

// endregion: --- Error Boilerplate
