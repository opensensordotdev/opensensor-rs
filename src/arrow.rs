/// Sensors should implement this trait for Apache Arrow in-memory serialization and deserialization
pub trait ArrowSerializable {

    /// This should be the error type of the implementing sensor
    type Error;

    /// Serialize this implementing sensor to bytes with Arrow IPC writer
    fn arrow_serialize(self) -> Vec<u8>;

    /// Static method to construct sensor type from bytes
    fn arrow_deserialize(bytes: &[u8]) -> Result<Self, Self::Error> where Self: Sized;
}