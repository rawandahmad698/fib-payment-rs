use fib_payment_rs::{FibClient, FibConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize configuration from environment variables
    let config = FibConfig::from_env()?;
    
    // Create a new client instance
    let client = FibClient::new(config);

    // Create a new payment
    let payment = client.create_payment(
        1000.0,                   // amount
        Some("IQD".to_string()),           // currency
        None, // callback_url (will use from config)
        Some("Test payment".to_string()),   // description
        None,                              // refundable_for
    ).await?;
    
    println!("Created payment: {:#?}", payment);
    
    // // Get payment status
    let status = client.get_payment_status(&payment.payment_id).await?;
    println!("Payment status: {:#?}", status);
    //
    // Try to refund the payment
    match client.refund_payment(&payment.payment_id).await {
        Ok(refund) => println!("Refund successful: {:#?}", refund),
        Err(e) => println!("Refund failed: {}", e),
    }
    
    Ok(())
} 