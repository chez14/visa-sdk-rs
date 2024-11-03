/// Represents the API level for the application, which determines the URL to be
/// used.
///
/// ## Documentations
/// - <https://developer.visa.com/pages/going-live>
/// - <https://developer.visa.com/pages/visa-developer-pki>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiLevel {
    /// Use the sandbox base URL for testing and development.
    #[default]
    Sandbox,
    /// Use the certification base URL for pre-production testing.
    Certification,
    /// Use the production base URL for live operations.
    Production,
}
