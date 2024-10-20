/// A macro to configure an API invoker with the mock server's address and scheme.
///
/// This macro sets the IP host, port, and scheme of the given API invoker to match
/// the provided mock server's socket address.
///
/// # Arguments
///
/// * `$api_invoker` - The API whose URL will be configured.
/// * `$server_mocks` - The mock server providing the socket address.
///
/// # Example
///
/// ```
/// apply_mock!(api_invoker, server_mocks);
/// ```
#[macro_export]
macro_rules! apply_mock {
    ($api_invoker:expr, $server_mocks:expr) => {
        let _ = $api_invoker
            .url
            .set_ip_host($server_mocks.socket_address().ip());
        let _ = $api_invoker
            .url
            .set_port(Some($server_mocks.socket_address().port()));
        let _ = $api_invoker.url.set_scheme("http");
    };
}
