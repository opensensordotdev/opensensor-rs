//! Generic code for implementing new sensors within the OpenSensor ecosystem
//!
//! The design goals of the sensor crate are similar to those of the
//! [embedded-hal](https://www.github.com/rust-embedded/embedded-hal):
//! - Erase sensor specific details
//! - Be generic within a sensor and across sensors
//! - Serve as a foundation for building an ecosystem of sensor-agnostic data engineering code

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

pub mod archiver;
/// Trait for arrow serialization
pub mod arrow;
pub mod error;
pub mod measurement;
/// Trait that sensors should implement to produce parquet archives
pub mod parquet;
#[allow(dead_code, unused_imports, missing_docs)]
#[allow(clippy::all)]
pub mod reflection_generated;
pub mod sensor;

#[cfg(test)]
mod test_arrow;
#[cfg(test)]
mod tests;

pub mod transducer;

pub use sensor::Sensor;
/// Reexports
pub use transducer::Transducer;

/// A sink for sensor data stored in Redpanda into various downstream data systems
///
/// Use for implementing an S3 Parquet sink (also the Archiver trait), MyCelial (SQLite), and OLTP (Scylladb)
///
/// To make it possible to track how much of a given topic has been written to the particular sink, do manual
/// offset commits to the consumer group (and use dedicated consumer group ids for each type of sink per measurement)
/// See the archiver crate and trait for an example of how to do manual offset commits once a batch of measurements
/// have been confirmed to be written to a downstream sink.
///
/// It might make more sense to separate these out by the type of sink (have a separate Archiver, SQLite, and ScyllaDB trait)
/// that can also be implemented on AlgorithmResult/InferenceResults vs a single SensorSink trait (and have to also write a
/// ModelSink + other types of traits)
#[async_trait::async_trait]
pub trait SensorSink {}
