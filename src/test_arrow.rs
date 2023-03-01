use std::borrow::Borrow;
use std::fs::File;
use std::sync::Arc;

use arrow2::array::*;
use arrow2::chunk::Chunk;
use arrow2::datatypes::{Schema, Field};
use arrow2::io::parquet::read;
use arrow2::io::parquet::write::{FileWriter, Encoding, RowGroupIterator, Version, ZstdLevel, CompressionOptions, WriteOptions};
use arrow2_convert::deserialize::{arrow_array_deserialize_iterator, TryIntoCollection};

use arrow2_convert::{
    serialize::TryIntoArrow, ArrowDeserialize, ArrowField,
    ArrowSerialize,
};

/// Complex example that uses the following features:
///
/// - Deeply Nested structs and lists
/// - Custom types
#[derive(Debug, Clone, PartialEq, ArrowField, ArrowSerialize, ArrowDeserialize)]
pub struct Root {
    name: Option<String>,
    is_deleted: bool,
    a1: Option<f64>,
    a2: i64,
    // binary
    a3: Option<Vec<u8>>,
    // date32
    a4: chrono::NaiveDate,
    // timestamp(ns, None)
    a5: chrono::NaiveDateTime,
    // timestamp(ns, None)
    a6: Option<chrono::NaiveDateTime>,
    // array of date times
    date_time_list: Vec<chrono::NaiveDateTime>,
    // optional list array of optional strings
    nullable_list: Option<Vec<Option<String>>>,
    // optional list array of required strings
    required_list: Vec<Option<String>>,
    // custom type
    custom: CustomType,
    // custom optional type
    nullable_custom: Option<CustomType>,
    // vec custom type
    custom_list: Vec<CustomType>,
    // nested struct
    child: Child,
    // int 32 array
    int32_array: Vec<i32>,
    // large binary
    #[arrow_field(type = "arrow2_convert::field::LargeBinary")]
    large_binary: Vec<u8>,
    // fixed size binary
    // #[arrow_field(type = "arrow2_convert::field::FixedSizeBinary<3>")]
    fixed_size_binary: Vec<u8>,
    // large string
    #[arrow_field(type = "arrow2_convert::field::LargeString")]
    large_string: String,
    // large vec
    #[arrow_field(type = "arrow2_convert::field::LargeVec<i64>")]
    large_vec: Vec<i64>,
    // fixed size vec
    // #[arrow_field(type = "arrow2_convert::field::FixedSizeVec<i64, 3>")]
    fixed_size_vec: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, ArrowField, ArrowSerialize, ArrowDeserialize)]
pub struct Child {
    a1: i64,
    a2: String,
    // nested struct array
    child_array: Vec<ChildChild>,
}

#[derive(Debug, Clone, PartialEq, Eq, ArrowField, ArrowSerialize, ArrowDeserialize)]
pub struct ChildChild {
    a1: i32,
    bool_array: Vec<bool>,
    int64_array: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A newtype around a u64
pub struct CustomType(u64);

/// To use with Arrow three traits need to be implemented:
/// - ArrowField
/// - ArrowSerialize
/// - ArrowDeserialize
impl arrow2_convert::field::ArrowField for CustomType {
    type Type = Self;

    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Extension(
            "custom".to_string(),
            Box::new(arrow2::datatypes::DataType::UInt64),
            None,
        )
    }
}

impl arrow2_convert::serialize::ArrowSerialize for CustomType {
    type MutableArrayType = arrow2::array::MutablePrimitiveArray<u64>;

    #[inline]
    fn new_array() -> Self::MutableArrayType {
        Self::MutableArrayType::from(<Self as arrow2_convert::field::ArrowField>::data_type())
    }

    #[inline]
    fn arrow_serialize(v: &Self, array: &mut Self::MutableArrayType) -> arrow2::error::Result<()> {
        array.try_push(Some(v.0))
    }
}

impl arrow2_convert::deserialize::ArrowDeserialize for CustomType {
    type ArrayType = arrow2::array::PrimitiveArray<u64>;

    #[inline]
    fn arrow_deserialize(v: Option<&u64>) -> Option<Self> {
        v.map(|t| CustomType(*t))
    }
}

// enable Vec<CustomType>
arrow2_convert::arrow_enable_vec_for_type!(CustomType);

fn item() -> Root {
    use chrono::{NaiveDate, NaiveDateTime};

    Root {
        name: Some("a".to_string()),
        is_deleted: false,
        a1: Some(0.1),
        a2: 1,
        a3: Some(b"aa".to_vec()),
        a4: NaiveDate::from_ymd_opt(1970, 1, 2).unwrap(),
        a5: NaiveDateTime::from_timestamp_opt(10000, 0).unwrap(),
        a6: Some(NaiveDateTime::from_timestamp_opt(10001, 0)).unwrap(),
        date_time_list: vec![
            NaiveDateTime::from_timestamp_opt(10000, 10).unwrap(),
            NaiveDateTime::from_timestamp_opt(10000, 11).unwrap(),
        ],
        nullable_list: Some(vec![Some("cc".to_string()), Some("dd".to_string())]),
        required_list: vec![Some("aa".to_string()), Some("bb".to_string())],
        custom: CustomType(10),
        nullable_custom: Some(CustomType(11)),
        custom_list: vec![CustomType(12), CustomType(13)],
        child: Child {
            a1: 10,
            a2: "hello".to_string(),
            child_array: vec![
                ChildChild {
                    a1: 100,
                    bool_array: vec![false],
                    int64_array: vec![45555, 2124214, 224, 24214, 2424],
                },
                ChildChild {
                    a1: 101,
                    bool_array: vec![true, true, true],
                    int64_array: vec![4533, 22222, 2323, 333, 33322],
                },
            ],
        },
        int32_array: vec![0, 1, 3],
        large_binary: b"aa".to_vec(),
        fixed_size_binary: b"aaa".to_vec(),
        large_string: "abcdefg".to_string(),
        large_vec: vec![1, 2, 3, 4],
        fixed_size_vec: vec![10, 20, 30],
    }
}

fn item2() -> Root {
    use chrono::{NaiveDate, NaiveDateTime};

    Root {
        name: Some("b".to_string()),
        is_deleted: true,
        a1: Some(0.1),
        a2: 1,
        a3: Some(b"aa".to_vec()),
        a4: NaiveDate::from_ymd_opt(1970, 1, 2).unwrap(),
        a5: NaiveDateTime::from_timestamp_opt(10000, 0).unwrap(),
        a6: None,
        date_time_list: vec![
            NaiveDateTime::from_timestamp_opt(10000, 10).unwrap(),
            NaiveDateTime::from_timestamp_opt(10000, 11).unwrap(),
        ],
        nullable_list: None,
        required_list: vec![Some("ee".to_string()), Some("ff".to_string())],
        custom: CustomType(11),
        nullable_custom: None,
        custom_list: vec![CustomType(14), CustomType(13)],
        child: Child {
            a1: 11,
            a2: "hello again".to_string(),
            child_array: vec![
                ChildChild {
                    a1: 100,
                    bool_array: vec![true, false, false, true],
                    int64_array: vec![111111, 2222, 33],
                },
                ChildChild {
                    a1: 102,
                    bool_array: vec![false],
                    int64_array: vec![45555, 2124214, 224, 24214, 2424],
                },
            ],
        },
        int32_array: vec![111, 1],
        large_binary: b"bb".to_vec(),
        fixed_size_binary: b"bbb".to_vec(),
        large_string: "abdefag".to_string(),
        large_vec: vec![5, 4, 3, 2],
        fixed_size_vec: vec![11, 21, 32],
    }
}

#[test]
fn test_round_trip() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [item(), item2()];

    let array: Box<dyn Array> = original_array.try_into_arrow()?;
    let struct_array = array
        .as_any()
        .downcast_ref::<arrow2::array::StructArray>()
        .unwrap();
    assert_eq!(struct_array.len(), 2);

    let values = struct_array.values();
    assert_eq!(values.len(), 21);
    assert_eq!(struct_array.len(), 2);

    // can iterate one struct at a time without collecting
    for _i in arrow_array_deserialize_iterator::<Root>(array.borrow())? {
        // do something
    }

    // or can back to our original vector
    let foo_array: Vec<Root> = array.try_into_collection()?;
    assert_eq!(foo_array, original_array);
    Ok(())
}

/// Write arrow2 array to parquet bytes in a buffer
#[test]
fn round_trip_parquet() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [item(), item(), item()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("root_custom_struct", <Root as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 25]],
    )?;

    // anything implementing `std::io::Write` works
    // let mut buffer = vec![];
    let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    // // Wrap buffer in a Cursor...this makes the buffer impl Read & Seek (needed for read::read_metadata)
    // let mut reader = std::io::Cursor::new(buffer);

    // // we can read its metadata:
    // let metadata = read::read_metadata(&mut reader)?;

    // // and infer a [`Schema`] from the `metadata`.
    // let schema = read::infer_schema(&metadata)?;

    // println!("Schema: {:?}", &schema);

    // // we can filter the columns we need (here we select all)
    // let schema = schema.filter(|_index, _field| true);

    // // we can read the statistics of all parquet's row groups (here for each field)
    // for field in &schema.fields {
    //     let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
    //     println!("{statistics:#?}");
    // }

    // // Get all the row groups
    // let row_groups = metadata
    //     .row_groups;

    // // We can then read the row groups into chunks
    // let chunks = read::FileReader::new(reader, row_groups, schema, None, None, None);

    // // iterate over chunks and validate each is not empty
    // for maybe_chunk in chunks {
    //     println!("{:?}", maybe_chunk);
    //     let chunk = maybe_chunk?;
    //     assert!(!chunk.is_empty());
    // }

    Ok(())
}

/// Test struct with no nested arrays or structs
#[derive(Clone, PartialEq, Debug, ArrowField, ArrowSerialize, ArrowDeserialize)]
struct FlatStruct {
    a: u32,
    b: String,
    c: i32,
}

impl Default for FlatStruct {
    fn default() -> Self {
        Self { a: 12, b: String::from("test_string"), c: 13 }
    }
}

/// Take flat struct round trip to parquet file bytes in a buffer and back
#[test]
fn test_flat_roundtrip() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [FlatStruct::default(), FlatStruct::default()];

    let array: Box<dyn Array> = original_array.try_into_arrow()?;
    let struct_array = array
        .as_any()
        .downcast_ref::<arrow2::array::StructArray>()
        .unwrap();
    assert_eq!(struct_array.len(), 2);

    let values = struct_array.values();
    assert_eq!(values.len(), 3);
    assert_eq!(struct_array.len(), 2);

    // iterate one struct at a time without collecting
    for s in arrow_array_deserialize_iterator::<FlatStruct>(array.borrow())? {
        println!("{:?}", s);
    }

    // or can back to our original vector
    let foo_array: Vec<FlatStruct> = array.try_into_collection()?;
    assert_eq!(foo_array, original_array);
    Ok(())
}

/// Write flat struct to parquet bytes
/// 
/// This resulting parquet file can actually be opened by pyarrow in the parquet.ipynb notebook in the root of this crate
/// This is the only parquet file generated by arrow2 that doesn't result in an OSError for nesting issues
#[test]
fn flat_struct_parquet_file() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [FlatStruct::default(), FlatStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("flat_struct", <FlatStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    // let mut buffer = vec![];
    let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    Ok(())
}

/// Round trip flat struct (no nested structs/arrays) to parquet file bytes and back
#[test]
fn flat_struct_round_trip_parquet() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [FlatStruct::default(), FlatStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("flat_struct", <FlatStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    let mut buffer = vec![];
    // let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    // Wrap buffer in a Cursor...this makes the buffer impl Read & Seek (needed for read::read_metadata)
    let mut reader = std::io::Cursor::new(buffer);

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    println!("Schema: {:?}", &schema);

    // we can filter the columns we need (here we select all)
    let schema = schema.filter(|_index, _field| true);

    // we can read the statistics of all parquet's row groups (here for each field)
    for field in &schema.fields {
        let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
        println!("{statistics:#?}");
    }

    // Get all the row groups
    let row_groups = metadata
        .row_groups;

    // We can then read the row groups into chunks
    let chunks = read::FileReader::new(reader, row_groups, schema, None, None, None);

    // iterate over chunks and validate each is not empty
    for maybe_chunk in chunks {
        println!("{:?}", maybe_chunk);
        let chunk = maybe_chunk?;
        assert!(!chunk.is_empty());
    }

    Ok(())
}

/// Sample struct with a nested array
#[derive(Clone, PartialEq, Debug, ArrowField, ArrowSerialize, ArrowDeserialize)]
struct ArrayStruct {
    a: u32,
    b: Vec<Vec<u32>>,
    c: i32,
}

impl Default for ArrayStruct {
    fn default() -> Self {
        Self { a: 12, b: vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9]], c: 13 }
    }
}

#[test]
fn test_array_roundtrip() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [ArrayStruct::default(), ArrayStruct::default()];

    let array: Box<dyn Array> = original_array.try_into_arrow()?;
    let struct_array = array
        .as_any()
        .downcast_ref::<arrow2::array::StructArray>()
        .unwrap();
    assert_eq!(struct_array.len(), 2);

    let values = struct_array.values();
    assert_eq!(values.len(), 3);
    assert_eq!(struct_array.len(), 2);

    // iterate one struct at a time without collecting
    for s in arrow_array_deserialize_iterator::<ArrayStruct>(array.borrow())? {
        println!("{:?}", s);
    }

    // or can back to our original vector
    let foo_array: Vec<ArrayStruct> = array.try_into_collection()?;
    assert_eq!(foo_array, original_array);
    Ok(())
}

/// Round trip serialization of a struct with a nested array to parquet and back
/// 
/// ERROR: even though this test will pass, if you try to read the resulting parquet file with pyarrow
/// using the parquet.ipynb notebook in the root of this crate, you get an
/// "OSError: Malformed levels. min: 0 max: 3 out of range.  Max Level: 2" 
#[test]
fn array_struct_parquet_file() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [ArrayStruct::default(), ArrayStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("array_struct", <ArrayStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    // let mut buffer = vec![];
    let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    Ok(())
}

/// Round trip serialization to parquet bytes in a buffer and back
#[test]
fn array_struct_round_trip_parquet() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [ArrayStruct::default(), ArrayStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("flat_struct", <ArrayStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    let mut buffer = vec![];
    // let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    // Wrap buffer in a Cursor...this makes the buffer impl Read & Seek (needed for read::read_metadata)
    let mut reader = std::io::Cursor::new(buffer);

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    println!("Schema: {:?}", &schema);

    // we can filter the columns we need (here we select all)
    let schema = schema.filter(|_index, _field| true);

    // we can read the statistics of all parquet's row groups (here for each field)
    for field in &schema.fields {
        let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
        println!("{statistics:#?}");
    }

    // Get all the row groups
    let row_groups = metadata
        .row_groups;

    // We can then read the row groups into chunks
    let chunks = read::FileReader::new(reader, row_groups, schema, None, None, None);

    // iterate over chunks and validate each is not empty
    for maybe_chunk in chunks {
        println!("{:?}", maybe_chunk);
        let chunk = maybe_chunk?;
        assert!(!chunk.is_empty());
    }

    Ok(())
}

/// Nested array structure for round-trip + parquet file serialization
#[derive(Clone, PartialEq, Debug, ArrowField, ArrowSerialize, ArrowDeserialize)]
struct NestedArrayStruct {
    a: u32,
    b: Vec<Vec<u32>>,
    c: i32,
}

impl Default for NestedArrayStruct {
    fn default() -> Self {
        Self { a: 12, b: vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9]], c: 13 }
    }
}

/// Nested parquet array roundtrip in memory
#[test]
fn test_nested_array_roundtrip() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [NestedArrayStruct::default(), NestedArrayStruct::default()];

    let array: Box<dyn Array> = original_array.try_into_arrow()?;
    let struct_array = array
        .as_any()
        .downcast_ref::<arrow2::array::StructArray>()
        .unwrap();
    assert_eq!(struct_array.len(), 2);

    let values = struct_array.values();
    assert_eq!(values.len(), 3);
    assert_eq!(struct_array.len(), 2);

    // iterate one struct at a time without collecting
    for s in arrow_array_deserialize_iterator::<NestedArrayStruct>(array.borrow())? {
        println!("{:?}", s);
    }

    // or can back to our original vector
    let foo_array: Vec<NestedArrayStruct> = array.try_into_collection()?;
    assert_eq!(foo_array, original_array);
    Ok(())
}


/// Test that you can write a nested array to parquet
/// 
/// ERROR: even though this test will pass, if you try to read the resulting parquet file with pyarrow
/// using the parquet.ipynb notebook in the root of this crate, you get an
/// "OSError: Malformed levels. min: 0 max: 3 out of range.  Max Level: 2"
#[test]
fn nested_array_struct_parquet_file() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [NestedArrayStruct::default(), NestedArrayStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("array_struct", <NestedArrayStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    // let mut buffer = vec![];
    let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    Ok(())
}

#[test]
fn nested_array_struct_round_trip_parquet() -> arrow2::error::Result<()> {
    // serialize to an arrow array
    let original_array = [NestedArrayStruct::default(), NestedArrayStruct::default()];

    // declare a schema with fields
    let schema = Schema::from(vec![
        Field::new("flat_struct", <NestedArrayStruct as arrow2_convert::field::ArrowField>::data_type(), true),
    ]);

    let chunk: Chunk<Arc<dyn Array>> = original_array.try_into_arrow()?;

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::default())),
        version: Version::V1,
        data_pagesize_limit: None,
    };
    
    // encodings has to be the length of the number of elements in the struct
    // Maybe dynamically do this the same way that io/parquet/write/pages.rs is checking?
    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        vec![vec![Encoding::Plain; 3]],
    )?;

    // anything implementing `std::io::Write` works
    let mut buffer = vec![];
    // let mut buffer = File::create("test.parquet").unwrap();
    let mut writer = FileWriter::try_new(&mut buffer, schema, options)?;

    // Write to buffer
    for group in row_groups {
        writer.write(group?)?;
    }
    let _file_size = writer.end(None)?;

    // Wrap buffer in a Cursor...this makes the buffer impl Read & Seek (needed for read::read_metadata)
    let mut reader = std::io::Cursor::new(buffer);

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    println!("Schema: {:?}", &schema);

    // we can filter the columns we need (here we select all)
    let schema = schema.filter(|_index, _field| true);

    // we can read the statistics of all parquet's row groups (here for each field)
    for field in &schema.fields {
        let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
        println!("{statistics:#?}");
    }

    // Get all the row groups
    let row_groups = metadata
        .row_groups;

    // We can then read the row groups into chunks
    let chunks = read::FileReader::new(reader, row_groups, schema, None, None, None);

    // iterate over chunks and validate each is not empty
    for maybe_chunk in chunks {
        println!("{:?}", maybe_chunk);
        let chunk = maybe_chunk?;
        assert!(!chunk.is_empty());
    }

    Ok(())
}