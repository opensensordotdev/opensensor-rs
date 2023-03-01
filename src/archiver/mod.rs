#[allow(clippy::too_many_arguments)]
pub mod cli;
pub mod error;

#[cfg(test)]
mod tests;

use aws_sdk_s3::model::{
    BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier,
};
use aws_sdk_s3::output::ListObjectsV2Output;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error};
use std::str;
use tracing::{event, Level};

/// Delete a bucket, assuming all objects have already been removed from the bucket
pub async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    client.delete_bucket().bucket(bucket_name).send().await?;
    event!(Level::INFO, "Deleted bucket {}", bucket_name);
    Ok(())
}

/// Delete all objects within a bucket, allowing the bucket to be deleted without forcing
pub async fn delete_objects(client: &Client, bucket_name: &str) -> Result<(), Error> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;

    let mut delete_objects: Vec<ObjectIdentifier> = vec![];
    for obj in objects.contents().unwrap_or_default() {
        let obj_id = ObjectIdentifier::builder()
            .set_key(Some(obj.key().unwrap().to_string()))
            .build();
        delete_objects.push(obj_id);
    }
    client
        .delete_objects()
        .bucket(bucket_name)
        .delete(Delete::builder().set_objects(Some(delete_objects)).build())
        .send()
        .await?;

    let objects: ListObjectsV2Output = client.list_objects_v2().bucket(bucket_name).send().await?;
    match objects.key_count {
        0 => Ok(()),
        _ => Err(Error::Unhandled(Box::from(
            "There were still objects left in the bucket.",
        ))),
    }
}

/// Print a list of the objects within a bucket
pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<(), Error> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    println!("Objects in bucket:");
    for obj in objects.contents().unwrap_or_default() {
        println!("{:?}", obj.key().unwrap());
    }

    Ok(())
}

/// Copy an S3 object within a bucket
pub async fn copy_object(
    client: &Client,
    bucket_name: &str,
    object_key: &str,
    target_key: &str,
) -> Result<(), Error> {
    let mut source_bucket_and_object: String = "".to_owned();
    source_bucket_and_object.push_str(bucket_name);
    source_bucket_and_object.push('/');
    source_bucket_and_object.push_str(object_key);

    client
        .copy_object()
        .copy_source(source_bucket_and_object)
        .bucket(bucket_name)
        .key(target_key)
        .send()
        .await?;

    event!(
        Level::INFO,
        "Copied {}/{} to {}/{}",
        bucket_name,
        object_key,
        bucket_name,
        target_key
    );

    Ok(())
}

/// Download a specific object from a bucket, given the S3 client configuration
///
/// # Parameters
///
///
/// # Errors
///
/// - aws_sdk_s3::types::SdkError<aws_sdk_s3::error::GetObjectError>: If we fail to get the requested object
///
/// # Examples
///
/// ```no_run
/// let access_key = "USERNAME";
/// let secret_key = "SUPER_SECRET_PASSWORD";
/// let endpoint = "http://localhost:9000";
/// let region = "opensensor-region";
/// let bucket_name = "opensensor-archive";
/// let sensor_name = "radar-2d";
/// let chunk_size = 10000;
/// let kafka_addresses = "127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012";
///
/// let cli = Cli::new(
///     access_key,
///     secret_key,
///     endpoint,
///     region,
///     bucket_name,
///     sensor_name,
///     chunk_size,
///     kafka_addresses,
/// );
///
/// let client = cli.build_client();
///
/// let bucket = "models"
/// let key = "simple/config.pbtxt"
/// download_object(&client, bucket, key).await;
/// ```
pub async fn download_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    client.get_object().bucket(bucket).key(key).send().await?;

    Ok(())
}

/// Compresses and uploads an S3 object, given a client and bucket name
///
/// # Parameters
///
/// - data_uncompressed: reference to a byte array, the uncompressed data you want to upload
/// - client: the s3 client you want to use for uploading
/// - bucket_name: the bucket to upload to
/// - key: key within bucket bucket_name to upload to
///
/// # Errors
///
/// - aws_sdk_s3::Error: catch-all error for all the reasons the upload could fail (data fails to upload,
/// bucket name wrong, invalid key, etc)
///
/// # Examples
///
/// ```no_run
/// let access_key = "USERNAME";
/// let secret_key = "SUPER_SECRET_PASSWORD";
/// let endpoint = "http://localhost:9000";
/// let region = "opensensor-region";
/// let bucket_name = "opensensor-archive";
/// let sensor_name = "radar-2d";
/// let chunk_size = 10000;
/// let kafka_addresses = "127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012";
///
/// let cli = Cli::new(
///     access_key,
///     secret_key,
///     endpoint,
///     region,
///     bucket_name,
///     sensor_name,
///     chunk_size,
///     kafka_addresses,
/// );
///
/// let client = cli.build_client();
///
/// let data_uncompressed: [u8] = [1, 2, 3, 4, 5, 6];
/// let key = "test_key"
/// upload_object_zstd(&data_uncompressed, &client, bucket_name, key).await.unwrap()
/// ```
pub async fn upload_object_zstd(
    data_uncompressed: &[u8],
    client: &Client,
    bucket_name: &str,
    key: &str,
) -> Result<(), Error> {
    let body_compressed = ByteStream::from(zstd::bulk::compress(data_uncompressed, 0).unwrap());
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body_compressed)
        .content_type("application/octet-stream")
        .content_encoding("zstd")
        .send()
        .await?;

    event!(
        Level::INFO,
        "Uploaded zstd compressed object at key {} to bucket {}",
        key,
        bucket_name,
    );
    Ok(())
}

/// Create a s3 bucket given a region and s3 client configuration
///
/// # Parameters:
///
/// - client: s3 client configuration to create the bucket with
/// - bucket_name: name of the bucket to create, subject to the s3 bucket naming
///   [rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html)
/// - region: s3 region, within the client, to create the bucket within
///
/// # Errors:
///
/// - aws_sdk_s3::Error: catch-all error for all the reasons the bucket creation could fail
///
/// # Examples
/// ```
/// let access_key = "USERNAME";
/// let secret_key = "SUPER_SECRET_PASSWORD";
/// let endpoint = "http://localhost:9000";
/// let region = "opensensor-region";
/// let bucket_name = "opensensor-archive";
/// let sensor_name = "radar-2d";
/// let chunk_size = 10000;
/// let kafka_addresses = "127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012";
///
/// let cli = Cli::new(
///     access_key,
///     secret_key,
///     endpoint,
///     region,
///     bucket_name,
///     sensor_name,
///     chunk_size,
///     kafka_addresses,
/// );
///
/// let client = cli.build_client();
///
/// create_bucket(&client, bucket_name, region).await.unwrap()
/// ```
pub async fn create_bucket(client: &Client, bucket_name: &str, region: &str) -> Result<(), Error> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await?;
    event!(
        Level::INFO,
        "Created bucket {} in region {}",
        bucket_name,
        region,
    );
    Ok(())
}
