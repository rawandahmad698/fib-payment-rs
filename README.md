# FIB Payments Rust SDK

Rust SDK for integrating with the FIB Payments API.

## Features

- **Type Safety**: Fully type-safe API with compile-time checks
- **Automatic Token Management**: Thread-safe token caching and automatic renewal
- **Resilient HTTP Client**: Automatic retries with exponential backoff
- **Async/Await Support**: Built on Tokio for async operations
- **Error Handling**: Rich error types with detailed API error information
- **Zero-Cost Abstractions**: Idiomatic Rust implementation with no runtime overhead

## Directory Structure

```
fib-payment-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs                 # Library entry point and exports
│   ├── client/                # Client implementation
│   │   └── fib_client.rs      # Main client interface
│   ├── config/                # Configuration handling
│   │   ├── mod.rs
│   │   └── settings.rs        # Configuration implementation
│   ├── error.rs               # Error types and handling
│   ├── http/                  # HTTP layer
│   │   ├── mod.rs
│   │   └── client.rs          # Resilient HTTP client
│   ├── models/                # Data models
│   │   ├── mod.rs
│   │   ├── payment.rs         # Payment-related structs
│   │   └── token.rs           # Authentication types
│   └── repository/            # API implementation
│       ├── mod.rs
│       └── payment.rs         # Payment operations
└── examples/                  # Usage examples
    └── basic_usage.rs         # Basic usage example
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
// Will be available on crates.io soon
```

## Quick Start

### Load Configuration First
```rust
use fib_payment_rs::{FibClient, FibConfig};

// Load environment variables from .env file
dotenv::dotenv().ok();
    
// Initialize configuration from environment variables
let config = FibConfig::from_env()?;
```

### and then
```rust
use fib_payment_rs::{FibClient, FibConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = FibConfig::from_env()?;
    
    // Create client instance
    let client = FibClient::new(config);

    // Create a payment
    let payment = client.create_payment(
        1000.0,                        // amount
        Some("IQD".to_string()),       // currency
        None,                          // callback_url
        Some("Test payment".into()),   // description
        None,                          // refundable_for
    ).await?;

    println!("Created payment: {:?}", payment);
    Ok(())
}
```

### Check Payment Status
```rust

let status = client.get_payment_status(&payment.payment_id).await?;
println!("Payment status: {:#?}", status);
```

### Refund a Payment

```rust
match client.refund_payment(&payment.payment_id).await {
    Ok(refund) => println!("Refund successful: {:#?}", refund),
    Err(e) => println!("Refund failed: {}", e),
}
```

### Thread-Safe Token Management

token management using Tokio's `Mutex`:

```rust
token: Mutex<Option<String>>
```

- Automatically caches authentication tokens
- Thread-safe access for concurrent operations
- Lazy token initialization
- Automatic token refresh on expiration

### HTTP Client

- Automatic retries with exponential backoff
- Configurable retry parameters:
    - Initial interval: 100ms
    - Max interval: 10s
    - Multiplier: 2.0
    - Max elapsed time: 30s
- Smart retry decisions based on error types
- Automatic handling of transient failures

### Error Handling

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("API error: {message} (Code: {code}, Status: {status_code}, Trace: {trace_id:?})")]
    ApiError {
        status_code: u16,
        code: String,
        message: String,
        trace_id: Option<String>,
    },
    // ... other error types
}
```

- Rich error information including trace IDs
- Compile-time error handling enforcement
- Zero-cost error propagation
- Detailed error context for debugging

### Configuration

```rust
#[derive(Debug, Clone)]
pub struct FibConfig {
    pub base_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: Option<String>,
    pub refundable_for: String,
    pub currency: String,
}
```

- Environment variable loading with validation
- URL parsing and validation at initialization
- Optional fields with sensible defaults
- Clone support for configuration reuse

## Roadmap
- [ ] Add tests
- [ ] Add more examples
- [ ] Crates.io release

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
