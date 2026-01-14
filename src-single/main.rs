use lapin::options::BasicAckOptions;
use lapin::message::Delivery;
use futures::StreamExt;

mod rabbitmq;



/// Process a single RabbitMQ message
async fn process_message(delivery: Delivery) {
    // Get the message body as a string
    let payload: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&delivery.data);

    // Check for a specific header
    let headers = delivery.properties.headers();
    let header_value = rabbitmq::get_header_value_if_exists(headers, "my-header-key");
    log::info!("Header Value: {:?}", header_value);

    // Log the message payload
    // DO THE ACTUAL WORK HERE
    log::trace!("Received message: {}", payload);

    // Acknowledge the message
    delivery
        .ack(BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message");
}


#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Load RabbitMQ configuration from environment variables
    let config = rabbitmq::RabbitMQConfig::from_env();

    // Create a new consumer
    let mut consumer = config.create_new_consumer().await.expect("Failed to create consumer");

    // Process messages (One by one)
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                process_message(delivery).await;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
            }
        }
    }
}
