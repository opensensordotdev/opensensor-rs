use crate::archiver::cli::Cli;
use crate::archiver::{create_bucket, delete_bucket};

/// Create a test CLI that can be used for testing against the OpenSensor docker-compose
pub fn create_test_cli() -> Cli {
    let access_key = "user";
    let secret_key = "user123456";
    let endpoint = "http://localhost:9000";
    let region = "opensensor-region";
    let bucket_name = "opensensor-archive";
    let sensor_name = "radar-2d";
    let chunk_size = 10000;
    let kafka_addresses = "127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012";

    Cli::new(
        access_key,
        secret_key,
        endpoint,
        region,
        bucket_name,
        sensor_name,
        chunk_size,
        kafka_addresses,
    )
}

#[tokio::test]
pub async fn test_create_delete_bucket() {
    let cli = create_test_cli();
    let client = cli.build_client();

    // Valid inputs
    let bucket_name = "test-bucket";
    create_bucket(&client, bucket_name, cli.region())
        .await
        .unwrap();
    delete_bucket(&client, bucket_name).await.unwrap();

    // Bad bucket name
    let invalid_bucket_name = "test_bucket";
    let create_result = create_bucket(&client, invalid_bucket_name, cli.region()).await;
    assert!(create_result.is_err());

    // Double create raises an Error
    create_bucket(&client, bucket_name, cli.region())
        .await
        .unwrap();
    let create_result = create_bucket(&client, bucket_name, cli.region()).await;
    assert!(create_result.is_err());
    delete_bucket(&client, bucket_name).await.unwrap();
}

#[tokio::test]
pub async fn test_upload() {}

use arrow2::array::*;
use arrow2::chunk::Chunk;
use arrow2::compute::arithmetics;
use arrow2::datatypes::{DataType, Field, Schema};
use arrow2::error::Result;
use arrow2::io::parquet::write::*;

#[tokio::test]
async fn test_write_parquet() -> Result<()> {
    // declare arrays
    let a = Int32Array::from(&[Some(1), None, Some(3)]);
    let b = Int32Array::from(&[Some(2), None, Some(6)]);

    // compute (probably the fastest implementation of a nullable op you can find out there)
    let c = arithmetics::basic::mul_scalar(&a, &2);
    assert_eq!(c, b);

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("c1", DataType::Int32, true),
        Field::new("c2", DataType::Int32, true),
    ]);

    // declare chunk
    let chunk = Chunk::new(vec![a.arced(), b.arced()]);

    // write to parquet (probably the fastest implementation of writing to parquet out there)
    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };

    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain], vec![Encoding::Plain]],
    )?;

    // anything implementing `std::io::Write` works
    let mut buffer = vec![];

    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write the file.
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    println!("{:?}", buffer);
    Ok(())
}

use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

#[derive(Clone, Debug, PartialEq, ArrowField, ArrowSerialize, ArrowDeserialize)]
struct ArrowTest {
    a: i32,
    b: u32,
    c: String,
}

// #[test]
// fn test_parquet_rust_struct() -> Result<()> {
//     let original_array = [
//         ArrowTest { c: "hello".to_string(), a: 8, b: 10 },
//         ArrowTest { c: "one more".to_string(), a: 12, b: 14 },
//         ArrowTest { c: "good bye".to_string(), a: 16, b: 18 },
//     ];
//     let arrow_array: Box<dyn Array> = original_array.try_into_arrow().unwrap();

//     // declare arrays
//     // let a = Int32Array::from(&[Some(1), None, Some(3)]);
//     // let b = Int32Array::from(&[Some(2), None, Some(6)]);

//     // compute (probably the fastest implementation of a nullable op you can find out there)
//     // let c = arithmetics::basic::mul_scalar(&a, &2);
//     // assert_eq!(c, b);

//     // declare a schema with fields
//     let schema = Schema::from(vec![
//         Field::new("c1", DataType::Extension(ArrowTest), true),
//     ]);

//     // declare chunk
//     let chunk = Chunk::new(vec![arrow_array]);

//     // write to parquet (probably the fastest implementation of writing to parquet out there)

//     let options = WriteOptions {
//         write_statistics: true,
//         compression: CompressionOptions::Snappy,
//         version: Version::V1,
//     };

//     let row_groups = RowGroupIterator::try_new(
//         vec![Ok(chunk)].into_iter(),
//         &schema,
//         options,
//         vec![vec![Encoding::Plain], vec![Encoding::Plain]],
//     )?;

//     // anything implementing `std::io::Write` works
//     let mut buffer = vec![];

//     let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

//     // Write the file.
//     for group in row_groups {
//         writer.write(group?)?;
//     }
//     let _file_size = writer.end(None)?;

//     println!("{:?}", buffer);
//     Ok(())
// }
