//! Archive a Kafka topic to S3-compatible object storage
//!
//! Archiver is configured via command line arguments that allow users to specify the following:
//! - access-key: MINIO_ROOT_USER or AWS_ACCESS_KEY_ID as plaintext
//! - secret-key: MINIO_ROOT_PASSWORD or AWS_SECRET_ACCESS_KEY as plaintext
//! - endpoint: Address to connect to the s3-compatible storage at. This will be different depending on whether you're connecting
//!             from outside the k8s cluster or docker compose environment or inside. This doesn't seem to require specifying that
//!             the connection protocol should be s3...http://localhost:9000 seems to work just fine for MinIO running inside
//!             docker compose with port 9000 exposed.
//! - region: MINIO_REGION_NAME or AWS_DEFAULT_REGION. The s3 region to connect to.
//! - bucket-name: Bucket to save sensor archive data to.
//! - sensor-name: Name of the sensor to archive data from. This name is used to generate the Kafka topic name to subscribe to
//!                ("{sensor-name}-measurements"), the Kafka group_id associated with the consumer ("{sensor-name}-archiver") and the tag to prepend all object names with ()
//! - chunk-size: How many sensor measurements to include in a single archive file. As long as the resulting file is <2gb
//!               (the max size of a flatbuffer) and you have adequate system memory to store the flatbuffers before
//!               they're constructed and written to s3. In practice, this should probably be in the low hundreds of mb, but depends
//!               on the data production rate of the sensor.
//! - kafka-addresses: Hostname and ports, in Kafka form, of the brokers to connect to.
//!
//! Data is archived as a vector of chunk-size flatbuffer records, zstd compressed per archival file. To parse, un-compress
//! and use the readers provided in the messages crate. Readers can be generated for any of the programming languages
//! supported by flatbuffers. Last archived offsets are saved automatically in the consumer group topic offsets.
//!
//! # Example
//!
//! ```
//! cargo run --bin archiver -- --access-key user \
//! --secret-key user123456 \
//! --endpoint http://localhost:9000 \
//! --region opensensor-region \
//! --bucket-name  \
//! --sensor-name radar-2d \
//! --chunk-size 10000 \
//! --kafka-addresses 127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012
//! ```

// use archiver::cli::Cli;
// use archiver::error::ArchiveError;
// use chrono::Utc;
// use clap::Parser;
// use flatbuffers::FlatBufferBuilder;
// use futures_util::StreamExt;
// use messages::radar_2d::{
//     root_as_radar_measurement_2d, RadarMeasurement2d, RadarMeasurement2dArgs,
//     RadarMeasurement2dFlatBufferBuilder, RadarVector2DBuilder,
// };
// use redpanda::{consumer::CommitMode, consumer::Consumer, message::Message, RedpandaBuilder};
// use tracing::{event, Level};

// #[tokio::main]
// async fn main() -> Result<(), ArchiveError> {
//     utility::configure_tracing();
//     let cli = Cli::parse();

//     run_archiver(cli).await?;

//     Ok(())
// }

// /// Run a kafka archiver, given a parsed command line configuration
// ///
// /// # Parameters
// ///
// /// - cli (archiver.cli.Cli): CLI configuration to run the archiver from
// ///
// /// # Examples
// ///
// /// ```no_run
// /// let access_key = "USERNAME";
// /// let secret_key = "SUPER_SECRET_PASSWORD";
// /// let endpoint = "http://localhost:9000";
// /// let region = "opensensor-region";
// /// let bucket_name = "opensensor-archive";
// /// let sensor_name = "radar-2d";
// /// let chunk_size = 10000;
// /// let kafka_addresses = "127.0.0.1:9010,127.0.0.1:9011,127.0.0.1:9012";
// ///
// /// let cli = Cli::new(
// ///     access_key,
// ///     secret_key,
// ///     endpoint,
// ///     region,
// ///     bucket_name,
// ///     sensor_name,
// ///     chunk_size,
// ///     kafka_addresses,
// /// );
// ///
// /// run_archiver(cli).await?;
// /// ```
// ///
// /// TODO: Verify the ordering of these archived chunks is correct (does the archived data end up revered because of the push
// /// and then pop?)
// /// TODO: implement individual archiver for each message type because that's required to do the serialization and deserialization
// async fn run_archiver(cli: Cli) -> Result<(), ArchiveError> {
//     let client = cli.build_client();

//     // Configure Redpanda, disabling auto-commit to ensure we only commit topics consumption offsets
//     // for the "sensor_name-archiver" topics once the consumed records have been successfully
//     // written to S3
//     let mut builder = RedpandaBuilder::default();
//     let group_id = format!("{}-archiver", cli.sensor_name());
//     builder.set_group_id(&group_id);
//     builder.set("enable.auto.commit", "false");
//     builder.set_bootstrap_servers(cli.kafka_addresses());
//     let topic = format!("{}-measurements", cli.sensor_name());
//     let consumer = builder.build_consumer().unwrap();
//     consumer.subscribe(&[&topic]).unwrap();
//     let mut stream = consumer.stream();
//     event!(Level::INFO, "{:?}", consumer.consumer.position().unwrap());

//     // locals for archive chunk tracking
//     let chunk_size = cli.chunk_size();
//     let mut chunk_counter = 0;

//     // Vector to save archive chunks to...hard limit of 2GB per buffer due to 32 bit flatbuffer address space.
//     // The practical limit is somewhat less than this...current implementation relies on there being enough
//     // RAM to store all in-progress archive chunks in memory.
//     // The allocation for this seems unavoidable...there is no other way to save the intermediate references
//     // to flatbuffer buffer data
//     let mut archival_buffer: Vec<RadarMeasurement2dFlatBufferBuilder> = Vec::new();

//     // Stream the topic, writing archives to S3 every chunk_size messages
//     while let Some(m) = stream.next().await {
//         chunk_counter += 1;
//         let bytes = m.as_ref().unwrap().payload();
//         // If there's no payload, continue to the next message
//         if bytes.is_none() {
//             event!(
//                 Level::WARN,
//                 "Got empty Redpanda message payload, continuing to next message"
//             );
//             continue;
//         }
//         // We've already handled the Option::None case above, so this won't panic
//         let bytes = bytes.unwrap();
//         // Try to map the bytes to a radar_measurement_2d
//         let measurement = root_as_radar_measurement_2d(bytes).unwrap();
//         let radar_builder = RadarMeasurement2dFlatBufferBuilder::new(
//             measurement.measurement_strengths().unwrap().bytes(),
//             measurement.theta_radians(),
//         )
//         .unwrap();
//         archival_buffer.push(radar_builder);

//         if chunk_counter == chunk_size {
//             // Create a FlatBufferBuilder to construct the RadarVector2d from
//             let mut fbb = FlatBufferBuilder::new();
//             let mut offsets = Vec::new();

//             while let Some(m) = archival_buffer.pop() {
//                 let measurement_strengths_offset = fbb.create_vector(m.measurement_strengths());
//                 let offset = RadarMeasurement2d::create(
//                     &mut fbb,
//                     &RadarMeasurement2dArgs {
//                         theta_radians: *m.theta_radians(),
//                         measurement_strengths: Some(measurement_strengths_offset),
//                     },
//                 );
//                 offsets.push(offset);
//             }

//             let data_offsets = fbb.create_vector(&offsets);

//             let mut radar_vector_builder = RadarVector2DBuilder::new(&mut fbb);
//             radar_vector_builder.add_data(data_offsets);
//             let radar_vector_buffer = radar_vector_builder.finish();
//             fbb.finish_minimal(radar_vector_buffer);

//             let now = Utc::now();
//             let key = format!("{}/{}", cli.sensor_name(), now.to_rfc3339());
//             let data_uncompressed = fbb.finished_data();

//             // Try to upload (and compress) the data to s3. Return errors on upload failure or on offset commit failure
//             match archiver::upload_object_zstd(data_uncompressed, &client, cli.bucket_name(), &key)
//                 .await
//             {
//                 Ok(_) => event!(
//                     Level::DEBUG,
//                     "Uploaded key {} to bucket {}",
//                     key,
//                     cli.bucket_name()
//                 ),
//                 Err(e) => return Err(ArchiveError::S3Error(e)),
//             };
//             if let Err(e) = consumer.consumer.commit_consumer_state(CommitMode::Sync) {
//                 event!(Level::ERROR, "Failed to commit consumer offset. This may result in duplicate archives in archival storage. {}", e);
//                 return Err(ArchiveError::KafkaError(e));
//             };
//             // let timestamp = deserialize_key(m.unwrap().key().unwrap()).unwrap();
//             event!(Level::INFO, count = chunk_counter, timestamp = ?now, position = ?consumer.consumer.position().unwrap());

//             chunk_counter = 0;
//         }
//     }

//     Ok(())
// }

fn main() {}
