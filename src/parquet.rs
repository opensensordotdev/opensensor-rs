use parquet;

use std::sync::Arc;
use parquet::schema::types::Type;

///  This purpose of this trait is to facilitate code reuse for sensor data serialization and archiving.  Sensors should implement this trait.
pub trait ParquetArchivable {

    /// Should be the same as the sensor error
    type Error;

    /// Writes out the contents of self into get_file() and returns Ok() or the sensor error
    fn to_bytes_parquet(self) -> Result<Vec<u8>, Self::Error>;

    /// Reads the file in get_file() into either Ok(ParquetArchivableType) or the sensor error
    fn from_bytes_parquet(bytes: &[u8]) -> Result<Self, Self::Error> where Self: Sized;

    /// The output of this is a parquet schema.
    fn schema(&self) -> Arc<Type>;
}