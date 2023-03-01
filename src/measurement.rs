//! Measurement trait for raw sensor measurements and derived data streams

use std::error::Error;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use flatbuffers::FlatBufferBuilder;
use futures_core::Stream;
use redpanda::{
    message::{BorrowedMessage, Message},
    producer::RedpandaRecord,
};

/// Convert nanoseconds since unix epoch (in UTC) to a UTC datetime
pub fn nanos_to_date_time(unix_ns: i64) -> LocalResult<DateTime<Utc>> {
    Utc.timestamp_opt(
        (unix_ns / 1_000_000_000) as i64,
        (unix_ns % 1_000_000_000) as u32,
    )
}

/// Measurement error
///
/// Enforce that this can only be implemented for errors with the std::error::Error trait bound
///
/// Since there is no way to enforce that an enum contains a variant, this trait requires the enum to return
/// its empty payload error variant
pub trait MeasurementError: Error {
    /// Return the empty payload variant here
    fn empty_payload_error() -> Self;
}

/// Raw measurement from a Sensor or derived data from a computation (i.e. tracking algorithm or ML model)
///
/// ## Implementing
///
/// Satisfy the trait boundary `Into<FlatBufferBuilder<'a>>` by implementing
/// `From<YourMeasurementStruct> for for FlatBufferBuilder<'_>`
///
/// ### Implementers are responsible for
///
/// - `Error` : Error type used in the Measurement's constructor and field validation methods
/// - `TOPIC_NAME` : Topic to store this measurement to in Redpanda
/// - `from_bytes` : How to deserialize from bytes to your Measurement
/// - `timestamp` : Return your Measurement's internal representation of the UTC time is was measured
/// - `Into<FlatBufferBuilder<'a>> : How to serialize your Measurement to a Flatbuffer
///
/// ### Default implementations are provided for
///
/// - `to_bytes`
/// - `to_message`
/// - `from_message`
/// - `timestamp_nanos`
pub trait Measurement<'a>: Into<FlatBufferBuilder<'a>> {
    /// Associated type for the measurement's specific error
    ///
    /// We need this to be separate from the MeasurementError defined above because there are measurement-specific
    /// field validations that need to be applied & can't be expressed adequately in a generic MeasurementError type
    type Error: MeasurementError;

    /// Topic to produce this measurement to
    ///
    /// OpenSensor follows the following Kafka topic naming convention:
    /// [raw | derived].domain.[subdomain-1]...[subdomain-N].data-name
    ///
    /// ** All text between `.` should be lowercase-kebab-case **
    ///
    /// ## Definitions:
    /// - 'raw' : data read directly off a sensor, parsed, and produced to Kafka (i.e. radar data off the sensor)
    /// - 'derived' : data produced by consuming one/more raw data streams and performing some value added
    ///   computation (i.e. calculating bearing/range to own-platform by combining latitude/longitude from an AIS
    ///   or ADS-B measurement with own-platform latitude/longitude) or as the result of a ML model
    /// - 'domain' : the domain the data is read from (i.e. surface, air, subsurface, rf)
    ///
    /// TODO: most kafka topic naming guidance says to not include changing fields (i.e. ML algorithm type or
    /// version number) in the topic name, but we probably want to avoid collisions between data from different
    /// algorithm versions & want that to be reflected in the topic name?
    ///
    /// ## Examples:
    /// AIS vessel static data - `raw.surface.ais.vessel-static`
    /// ADS-B Aircraft identification and category - `raw.air.ads-b.aircraft-identification-category`
    /// Semantic segmentation data from ML model trained on ship data - `derived.surface.ship-segmentation.mask-r-cnn`
    ///
    /// ## Naming convention based off a combination of:
    /// - https://www.kadeck.com/blog/kafka-topic-naming-conventions-5-recommendations-with-examples
    /// - https://cnr.sh/essays/how-paint-bike-shed-kafka-topic-naming-conventions
    /// - https://www.conduktor.io/kafka/kafka-topics-naming-convention
    const TOPIC_NAME: &'static str;

    /// Serialize a Measurement into a vec of bytes, suitable for network transfer, consuming the Measurement
    ///
    /// ## Default Implementation
    ///
    /// Notionally, this should be using FlatBuffers, but technically this isn't specific
    /// and it's probably better to avoid being overly proscriptive.
    fn to_bytes(self) -> Vec<u8> {
        let fbb: FlatBufferBuilder = self.into();

        fbb.finished_data().to_vec()
    }

    /// Serialize a Measurement to a Kafka message
    ///
    /// ## Default Implementation
    ///
    /// This default implementation can be overridden if a specific measurement needs different Kafka
    /// message serialization semantics. If you override Measurement::to_message, you MUST also override the
    /// Measurement::from_message method. Otherwise your custom message serialization won't be undone correctly.
    fn to_message(self) -> RedpandaRecord
    where
        Self: Sized,
    {
        let payload: Vec<u8> = self.to_bytes();
        RedpandaRecord::new(Self::TOPIC_NAME, None, payload, None)
    }

    /// Deserialize a Measurement from a vec of bytes off the network
    ///
    /// Notionally, this should be implemented using the FlatBuffers to read a struct
    /// from serialized data
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Deserialize a Measurement from a Kafka message
    ///
    /// ## Default Implementation
    ///
    /// This default implementation can be overridden if a specific measurement needs different Kafka
    /// message serialization semantics. If you override Measurement::to_message, you MUST also override
    /// this method. Otherwise your custom message serialization won't be undone correctly.
    ///
    /// ## Notes
    ///
    /// Working on the entire borrowed message instead of just the payload allows
    /// different message implementations to choose what they want to store in the
    /// message headers vs in the payload if they choose to implement a message-specific
    /// version of this method. We only care that you can deserialize a
    /// Measurement from a kafka message, not the specifics of how.
    fn from_message(message: BorrowedMessage) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let bytes = match message.payload() {
            Some(b) => b,
            None => return Err(Self::Error::empty_payload_error()),
        };

        Self::from_bytes(bytes)
    }

    /// Getter for the measurement's timestamp in UTC
    ///
    /// This is NOT to be confused with the time the Kafka Record wrapping the Measurement is created.
    ///
    /// There are two times associated with a Redpanda Record:
    /// 1. Timestamp that the Measurement is created. We store this in up to nanosecond precision as a DateTime<Utc>
    /// 2. Timestamp that the Redpanda Record is created. In Kafka, this represented as a long (64 bit) int storing
    ///    the milliseconds since UTC epoch.
    ///
    /// This method returns a DateTime<Utc> corresponding to (1)
    fn timestamp(&self) -> DateTime<Utc>;

    /// Getter for the measurement's timestamp in UTC nanoseconds since Unix epoch
    ///
    /// This is NOT to be confused with the time the Kafka Record wrapping the Measurement is created.
    ///
    /// There are two times associated with a Redpanda Record:
    /// 1. Timestamp that the Measurement is created. We store this in up to nanosecond precision as a DateTime<Utc>
    /// 2. Timestamp that the Redpanda Record is created. In Kafka, this represented as a long (64 bit) int storing
    ///    the milliseconds since UTC epoch.
    ///
    /// This method returns (1) in nanoseconds
    fn timestamp_nanos(&self) -> i64 {
        self.timestamp().timestamp_nanos()
    }

    /// Getter for the identify of the sensor or algorithm source that generated the measurement
    ///
    /// The source_id is an identifier that specifies what sensor the measurement was read from
    fn source_id(&self) -> &str;
}

/// Steam of sensor measurements, either from raw or derived data
pub trait MeasurementStream {
    /// Type of the individual sensor measurement in the stream
    type Item: for<'a> Measurement<'a>;

    /// A measurement stream from a sensor or derived data stream
    fn stream(&self) -> dyn Stream<Item = Self::Item>;
}
