# HelloWorld

This sample demonstrates how to use the HelloWorld endpoint of the Visa API to
test the connection to the Visa API in certification and production
environments.

## Environment Variables

- `VISA_USER_ID`: Your Visa API user ID.
- `VISA_PASSWORD`: Your Visa API password.
- `VISA_CERT`: Path to your mutual TLS certificate.
- `VISA_CERT_PASSWORD`: (Optional) Password for your mutual TLS certificate.

## Running the Sample

1. Set up the environment variables as described above.
2. Build and run the sample:

    ```sh
    cargo run
    ```

## References
- <https://developer.visa.com/pages/working-with-visa-apis/visa-developer-quick-start-guide>
- <https://developer.visa.com/pages/visa-developer-center-playground>
