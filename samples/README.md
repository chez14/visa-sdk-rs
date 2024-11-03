# Visa SDK Samples

This folder contains sample projects demonstrating how to use the Visa SDK for
various API endpoints. Each sample project is designed to showcase different
functionalities provided by the SDK.

## Getting Started

To run the samples, follow these steps:

1. **Clone this repository**, then navigate to the `samples` folder.

2. **Set up environment variables**: Each sample project may require specific
    environment variables to be set. Refer to the `.env.sample` file in each
    project directory for the required variables. Copy the `.env.sample` file to
    `.env` and fill in the necessary values.

    ```sh
    # samples/hello-world/

    cp .env.sample .env
    # Edit the .env file and fill in the required values
    ```
    You need to register an account on [Visa Development
    Platform](https://developer.visa.com/) website, generate the required
    certificate, obtain the User ID and Password (different from your account),
    and fill it into the `.env` file.

3. **Build and run the sample**: Navigate to the sample project directory and
    run the project using Cargo.

    ```sh
    cargo run
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE)
file for details.
