use std::fs::File;

use async_stream::stream;

use chrono::Utc;
use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use crate::error::SensorError;
use crate::measurement;
use crate::reflection_generated::reflection;

#[test]
fn test_timestamp_nanos() {
    let now = Utc::now();
    let now_ns = measurement::nanos_to_date_time(now.timestamp_nanos()).unwrap();
    println!("{} {}", now, now_ns);

    assert_eq!(now, now_ns)
}

/// field.id: flatbuffer field ID number
/// field.optional: bool, whether field is optional or not
#[test]
fn test_reflection() {
    use std::io::Read;
    
    let mut file = File::open("flatbuffers/simple.bfbs").expect("Filed to open flatbuffer schema file");

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    let schema = reflection::root_as_schema(&buf).expect("Failed to deserialize schema flatbuffer");

    for object in schema.objects() {
        println!("{}", object.name());
        for field in object.fields() {
            println!("{:?}", field.name());
            println!("{:?}", field.type_().base_type());
            println!("{:?}", field.documentation());
        }
    }
}

/// Sensor that produces a stream of measurements
#[async_trait::async_trait]
trait TestSensor<'a> {
    /// Measurement type used within the Sensor
    ///
    /// Send bound is required for this type to be used for async functions
    type SensorMeasurement: crate::measurement::Measurement<'a> + Send;

    /// Start collecting measurements, return an error if we hit something unrecoverable
    async fn run(self) -> Result<(), SensorError>;

    /// The measurement stream from running the sensor
    async fn measurement_stream(&self) -> dyn Stream<Item = Self::SensorMeasurement>;
}

fn zero_to_three() -> impl Stream<Item = u32> {
    stream! {
        for i in 0..3 {
            yield i;
        }
    }
}

#[tokio::test]
async fn test_stream() {
    let s = zero_to_three();
    pin_mut!(s); // needed for iteration

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}

trait TestConst {
    const SENSOR_NAME: &'static str;
}

struct ConstTesting {}

impl TestConst for ConstTesting {
    const SENSOR_NAME: &'static str = "test-sensor";
}

// impl ConstTesting {
//     const SENSOR_TOPIC: &'static str = const_str::format!("kafka-{}", ConstTesting::SENSOR_NAME);

//     /// This is how you access the constant you define
//     pub fn produce(&self) {
//         println!("Wrote to kafka topic {}", ConstTesting::SENSOR_TOPIC);
//     }
// }