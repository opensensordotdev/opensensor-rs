//! Generic OpenSensor Transducer for abstracting away hardware-specific sensor implementation details from Sensors

use crate::measurement::Measurement;
use async_trait::async_trait;
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

/// Transducer that handles hardware-specific communications (serial port, network socket, etc)
#[async_trait]
pub trait Transducer {
    /// Type for the measurement struct produced by the Transducer
    type SensorMeasurement: for<'a> Measurement<'a> + Send;

    /// Type for the error returned by the Transducer
    type Error: std::error::Error + Send;

    /// Identifier for the Transducer i.e. "AIS_NMEA_PILOTHOUSE"
    /// This has to be a function vs a constant because it'll be dynamically set by users
    ///
    /// TODO: Decide on standard format for these identifiers?
    fn source_id(&self) -> &str;

    /// Function for the Sensor that is connected to the Transducer to call to start reading measurements
    ///
    /// This is an Option because there can only be one copy of an mpsc::Receiver. So attempts to call this
    /// after the single instance of the Receiver has been returned will result in None
    fn rx(&mut self) -> Option<Receiver<Self::SensorMeasurement>>;

    /// Spawn the main loop of transducer, returning the join handle for the an error if it fails in a way that is unrecoverable
    ///
    /// Currently, the return type of tokio::runtime::task::JoinHandle indicates that we intend this method to call tokio::spawn()
    /// on an async inner loop that reads from the physical interface to the sensor/simulator. This might be suboptimal because
    /// if the loop involves any significant compute, we could end up blocking the tokio async executor. It might be worth
    /// reimplementing this to spawn a thread or fork a process?
    async fn listen(mut self) -> Result<JoinHandle<Result<(), Self::Error>>, Self::Error>;
}
