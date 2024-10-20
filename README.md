# Visa API SDK for Rust

## Installation

*Tbd*.

## Security

### Mutual TLS / Two Way SSL (mTls)

This repo does not provide the CA Root Bundle. You can download the CA in the
following links:
- https://developer.visa.com/pages/working-with-visa-apis/two-way-ssl#section2
- https://developer.visa.com/pages/visa-developer-pki

Provide the Client Secret, CA Bundle when initializing the client object:

```rs
// tbd
```


## Message Level Encryption (MLE)

Guide:
- https://developer.visa.com/pages/encryption_guide#gettingstartedwithmle

Provide the Client Secret, and Server Encryption Certificate when initializing
the client object:

```rs
// tbd
```


## License
[MIT](./LICENSE).

## Disclaimer
1. I (`chez14`) am not affiliated with Visa Inc. or any of its subsidiaries.
2. This crate has not been tested in a production environment. Use at your own risk.
