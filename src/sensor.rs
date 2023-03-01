//! Generic OpenSensor Sensor for producing sensor measurements from a Transducer to the OpenSensor stack

use crate::error::SensorError;
use crate::measurement::Measurement;
use redpanda::{error::KafkaError, producer::DeliveryFuture};

/// Sensor that produces a stream of measurements
#[async_trait::async_trait]
pub trait Sensor {
    /// Measurement type used within the Sensor
    ///
    /// If you have a sensor that produces multiple measurements (i.e. AIS produces several different
    /// message types), then write a struct per message type and implement measurement::Measurement for
    /// each struct. Then the SensorMeasurement associated type here is just an enum of those structs,
    /// and the enum also implements measurement::Measurement. The top-level enum Measurement implementation
    /// just matches self & calls the correct variant's implementation of the respective Measurement
    /// trait method.
    ///
    /// Send bound is required for this type to be used for async functions
    type SensorMeasurement: for<'a> Measurement<'a> + Send;

    /// Start collecting measurements, return an error if we hit something unrecoverable
    /// It's fine that this function is async because we're only calling it one (so one heap allocation)
    /// The function should call produce_measurement
    async fn run(mut self) -> Result<(), SensorError>;

    /// Produce a measurement to Redpanda
    /// Don't use async_trait here because each function call results in a heap allocation...we expect this
    /// function to be called in a hot loop and we don't want a separate heap allocation every time we call it...
    ///
    /// TODO: We should register the failures to queue or deliver measurements somewhere...probably in traces that go to Loki
    fn produce_measurement(
        &self,
        measurement: Self::SensorMeasurement,
    ) -> Result<DeliveryFuture, KafkaError>;
}
