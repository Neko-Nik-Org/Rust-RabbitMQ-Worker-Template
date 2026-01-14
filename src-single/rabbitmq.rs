use lapin::{options::*, types::{FieldTable, AMQPValue}, Connection, ConnectionProperties};
use std::env::var as env_var;
use std::error::Error;


/// Configuration for RabbitMQ connection and consumer
pub struct RabbitMQConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    virtual_host: String,
    consumer_tag: String,
    queue_name: String,
    is_queue_durable: bool,
}


/// Retrieve a header value by key from the FieldTable, if it exists
pub fn get_header_value_if_exists(headers: &Option<FieldTable>, key: &str) -> Option<String> {
    // Check if headers exist
    let headers = match headers {
        Some(h) => h,
        None => return None,
    };

    // Iterate through headers to find the key
    for (k, v) in headers {
        if k.as_str() == key {
            match v {
                AMQPValue::LongString(s) => {
                    return Some(String::from_utf8_lossy(s.as_bytes()).to_string());
                }
                AMQPValue::ShortString(s) => {
                    return Some(s.as_str().to_string());
                }
                AMQPValue::Boolean(b) => {
                    return Some(b.to_string());
                }
                AMQPValue::LongInt(i) => {
                    return Some(i.to_string());
                }
                AMQPValue::LongLongInt(i) => {
                    return Some(i.to_string());
                }
                AMQPValue::Float(f) => {
                    return Some(f.to_string());
                }
                AMQPValue::Double(d) => {
                    return Some(d.to_string());
                }
                AMQPValue::Timestamp(ts) => {
                    return Some(ts.to_string());
                }
                _ => {
                    return Some(format!("{:?}", v));
                }
            }
        }
    }

    // Key not found
    None
}



// ------- Implementations ------- //



impl RabbitMQConfig {
    /// Create a new RabbitMQConfig from environment variables
    pub fn from_env() -> Self {
        let host = env_var("RABBITMQ_HOST").expect("RABBITMQ_HOST must be set");
        let port = env_var("RABBITMQ_PORT")
            .unwrap_or_else(|_| "5672".to_string())
            .parse()
            .expect("RABBITMQ_PORT must be a valid u16");
        let username = env_var("RABBITMQ_USERNAME").expect("RABBITMQ_USERNAME must be set");
        let password = env_var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set");
        let virtual_host = env_var("RABBITMQ_VHOST").unwrap_or_else(|_| "/".to_string());
        let consumer_tag = env_var("RABBITMQ_CONSUMER_TAG").unwrap_or_else(|_| "my_consumer".to_string());
        let queue_name = env_var("RABBITMQ_QUEUE_NAME").expect("RABBITMQ_QUEUE_NAME must be set");
        let is_queue_durable = env_var("RABBITMQ_QUEUE_DURABLE")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("RABBITMQ_QUEUE_DURABLE must be a valid bool");

        RabbitMQConfig {
            host,
            port,
            username,
            password,
            virtual_host,
            consumer_tag,
            queue_name,
            is_queue_durable,
        }
    }

    /// Construct the AMQP URL from the configuration
    fn amqp_url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.virtual_host
        )
    }

    /// Create a new consumer for the specified queue
    pub async fn create_new_consumer(&self) -> Result<lapin::Consumer, Box<dyn Error>> {
        // Establish connection
        let connection = Connection::connect(&self.amqp_url(), ConnectionProperties::default()).await?;

        // Create a channel
        let channel = connection.create_channel().await?;

        // Declare the queue
        channel.queue_declare(
            &self.queue_name,
            QueueDeclareOptions {
                durable: self.is_queue_durable,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

        // Create the consumer
        let consumer = channel
            .basic_consume(
                &self.queue_name,
                &self.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(consumer)
    }
}
