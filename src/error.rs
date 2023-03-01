//! Error types for use by all Sensors
use redpanda::error::KafkaError;

/// Error type used by simple sensor
/// TODO: This will probably get deleted and turned into a Trait similar to the MeasurementError that
/// includes specific trait methods for returning specific variants of a Sensor's specific Error type
#[derive(thiserror::Error, Debug, Clone)]
pub enum SensorError {
    /// If the message returned by the consumer has an empty payload
    #[error("Unable to read any data from BorrowedMessage. Kafka payload was empty.")]
    EmptyPayloadError,
    /// If a Kafka error occurred
    #[error("Kafka error occurred {0}")]
    KafkaError(KafkaError),
    /// If the message never queued
    #[error("Failed to queue message locally, queue is full")]
    QueueError,
    /// If there is an error in the message's timestamp
    #[error("Invalid timestamp value {0}")]
    TimestampError(i64),
}
