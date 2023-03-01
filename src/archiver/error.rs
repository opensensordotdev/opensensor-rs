use aws_sdk_s3::Error;
use redpanda::error::KafkaError;

/// Error for all archiving-related issues
#[derive(thiserror::Error, Debug)]
pub enum ArchiveError {
    /// Wrap archiving-related Kafka errors
    #[error("A Kafka error occurred")]
    KafkaError(KafkaError),
    /// Wrap archiving-related s3 errors
    #[error("A S3 error occurred")]
    S3Error(Error),
}
