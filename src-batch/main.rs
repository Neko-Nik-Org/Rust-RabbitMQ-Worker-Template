use lapin::options::BasicAckOptions;
use lapin::message::Delivery;
use std::time::Duration;
use futures::StreamExt;

mod rabbitmq;



/// Process a batch of RabbitMQ messages (Auto Acknowledge after processing)
async fn process_batch_message(deliveries: &Vec<Delivery>) {
    // You can implement batch processing logic here
    // DO THE ACTUAL WORK HERE
    for delivery in deliveries {
        // Get the message body as a string
        let payload: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&delivery.data);
        log::trace!("Received message: {}", payload);

        // Check for a specific header
        let headers = delivery.properties.headers();
        let header_value = rabbitmq::get_header_value_if_exists(headers, "my-header-key");
        log::info!("Header Value: {:?}", header_value);
    }
}


#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Load RabbitMQ configuration from environment variables
    let config = rabbitmq::RabbitMQConfig::from_env();

    // Create a new consumer
    let mut consumer = config.create_new_consumer().await.expect("Failed to create consumer");

    // Prefetch count for batch processing
    let prefetch_count: usize = config.prefetch_count;
    let mut messages_batch: Vec<Delivery> = Vec::with_capacity(prefetch_count);

    log::debug!("Starting batch message processing with prefetch count: {}", prefetch_count);
    // Process messages in batches
    loop {
        // Fill the batch
        messages_batch.clear();

        // Collect messages up to the prefetch count
        while messages_batch.len() < prefetch_count {
            match tokio::time::timeout(
                // Batch wait time (in milliseconds) - Wait till this time for more messages
                Duration::from_millis(config.prefetch_window),
                consumer.next(),
            )
            .await
            {
                Ok(Some(Ok(delivery))) => {
                    messages_batch.push(delivery);
                }
                _ => break, // timeout or no more messages
            }
        }

        // If batch is not empty, continue processing
        if messages_batch.is_empty() {
            continue;
        }

        // Process each message in the batch
        process_batch_message(&messages_batch).await;

        // Acknowledge each message in the batch
        for delivery in messages_batch.drain(..) {
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to acknowledge message");
        }
    }
}
