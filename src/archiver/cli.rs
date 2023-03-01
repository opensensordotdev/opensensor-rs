//! Command Line Interface for an archiver

use aws_sdk_s3::{Client, Config, Credentials, Endpoint, Region};
use clap::Parser;

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    /// Sets a s3 access key (MinIO username)
    #[arg(short, long, value_name = "S3_ACCESS_KEY")]
    access_key: String,

    /// Sets the s3 secret key (MinIO password)
    #[arg(short, long, value_name = "S3_SECRET_KEY")]
    secret_key: String,

    /// Sets the s3 endpoint to connect to
    /// The protocol in the URL doesn't have to be s3://
    /// To connect from outside docker-compose to the local s3 endpoint, use http://localhost:9000
    /// TODO: how to do this when archiver is deployed inside docker-compose or k8s
    #[arg(short, long, value_name = "S3_ENDPOINT")]
    endpoint: String,

    /// Sets the s3 region to connect to
    #[arg(short, long, value_name = "S3_REGION")]
    region: String,

    /// Sets the s3 bucket name to archive to
    /// Note: This should just be of the form "opensensor-archive" or any other valid s3 bucket name
    #[arg(short, long, value_name = "S3_BUCKET_NAME")]
    bucket_name: String,

    /// Sensor name to archive data from
    /// Several pieces of information are derived from this:
    /// Redpanda topic name = sensor_name + "-measurements"
    /// Consumer group name = sensor_name + "-archiver"
    #[arg(long, value_name = "SENSOR_NAME")]
    sensor_name: String,

    /// How many messages to include per archive chunk
    #[arg(short, long, value_name = "MESSAGES_PER_CHUNK")]
    chunk_size: u64,

    /// Addresses of the brokers to connect to, in kafka form
    /// ex. 127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012
    #[arg(short, long, value_name = "KAFKA_ADDRESSES")]
    kafka_addresses: String,
}

impl Cli {
    /// Construct a new Cli for mocking + testing
    pub fn new(
        access_key: &str,
        secret_key: &str,
        endpoint: &str,
        region: &str,
        bucket_name: &str,
        sensor_name: &str,
        chunk_side: u64,
        kafka_addresses: &str,
    ) -> Self {
        Cli {
            access_key: access_key.to_owned(),
            secret_key: secret_key.to_owned(),
            endpoint: endpoint.to_owned(),
            region: region.to_owned(),
            bucket_name: bucket_name.to_owned(),
            sensor_name: sensor_name.to_owned(),
            chunk_size: chunk_side,
            kafka_addresses: kafka_addresses.to_owned(),
        }
    }

    /// S3 access key accessor
    pub fn access_key(&self) -> &str {
        &self.access_key
    }

    /// S3 secret key accessor
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }

    /// S3 endpoint accessor
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// S3 region accessor
    pub fn region(&self) -> &str {
        &self.region
    }

    /// S3 bucket name accessor
    pub fn bucket_name(&self) -> &str {
        &self.bucket_name
    }

    /// sensor name accessor
    pub fn sensor_name(&self) -> &str {
        &self.sensor_name
    }

    /// Max number of records to put in a single archival chunk
    pub fn chunk_size(&self) -> u64 {
        self.chunk_size
    }

    /// Kafka addresses the archiver consumes from
    pub fn kafka_addresses(&self) -> &str {
        &self.kafka_addresses
    }

    pub fn build_client(&self) -> Client {
        // credential provider name is required, but the value doesn't seem to matter
        let provider_name = "opensensor-credentials";
        let creds = Credentials::new(
            &self.access_key,
            &self.secret_key,
            None,
            None,
            provider_name,
        );

        let s3_endpoint = Endpoint::immutable(self.endpoint.parse().unwrap());

        let config = Config::builder()
            .region(Region::new(self.region.clone()))
            .endpoint_resolver(s3_endpoint)
            .credentials_provider(creds)
            .build();

        Client::from_conf(config)
    }
}
