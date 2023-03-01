# S3 archiver

Use AWS sdk to archive Redpanda topics to local MinIO object storage

Configuring AWS sdk to use something besides environment variable-based configurations:
[here](https://nickb.dev/blog/access-public-and-private-b2-s3-buckets-in-rust/)

## Reformulating row storage as column storage

Inspired from [here](https://towardsdatascience.com/the-beauty-of-column-oriented-data-2945c0c9f560)

Typically, we think of data as row entries like this:

```json
[
  {
    "message": "Hi Bob. How are you?",
    "timestamp": 1508423069,
    "senderId": 238476,
    "seen": true
  },{
    "message": "This is Alex.",
    "timestamp": 1508423226,
    "senderId": 238476,
    "seen": true
  },{
    "message": "Hi Alex. I am fine. How are you?",
    "timestamp": 1508423238,
    "senderId": 9837498,
    "seen": false
  }
]
```

...but you can rotate that data and look at it like this:

```json
{
  "messages": ["Hi Bob. How are you?", "This is Alex.", "Hi Alex. I am fine. How are you?"],
  "timestamps": [1508423069, 1508423226, 1508423238],
  "senderId": [238476, 238476, 9837498],
  "seen": [true, true, false]
}
```

Scan less to find what you want, better binary alignment

Concretely, for radar_2d, this looks like moving from:

```json
[
  {
    "measurement": [0,0,3,0,5],
    "theta_radians": 3.4,
    "timestamp": 1508423069,
  },{
    "measurement": [0,2,3,0,5],
    "theta_radians": 3.5,
    "timestamp": 1508423069,
  },{
    "measurement": [0,0,4,1,3],
    "theta_radians": 3.6,
    "timestamp": 1508423069,
  }
]
```

to

```json
{
  "measurements": [[0,0,3,0,5], [0,2,3,0,5], [0,0,4,1,3]],
  "timestamps": [1508423069, 1508423226, 1508423238],
  "theta_radians": [3.4, 3.5, 3.6],
}
```

In practice, since the `measurements` and `theta_radians` are contained in flatbuffer-serialized binary data, those
will be stored in a single parquet column with type `BYTE_ARRAY` (arbitrarily long byte arrays).

## Arrow and Parquet Relationship

3 Part series from Arrow project on relationship:

1. [Part 1](https://arrow.apache.org/blog/2022/10/05/arrow-parquet-encoding-part-1/)
2. [Part 2](https://arrow.apache.org/blog/2022/10/08/arrow-parquet-encoding-part-2/)
3. [Part 3](https://arrow.apache.org/blog/2022/10/17/arrow-parquet-encoding-part-3/)

## Arrow <-> Rust Struct Conversion

[Derive macro that supports arrow2](https://github.com/DataEngineeringLabs/arrow2-convert)

## Flatbuffer Reflection

We can use the flatbuffer intermediate representation to programatically access the structure of a flatbuffer. This feature is documented [here](https://google.github.io/flatbuffers/intermediate_representation.html); [this](https://jorenjoestar.github.io/post/flatbuffers_reflection_data_driven_rendering/) is one of the few articles explaining how to use this feature.

A method for building a binary flatbuffer (`.bfbs`) from a schema and reading it in using the [reflection.fbs](https://github.com/google/flatbuffers/blob/master/reflection/reflection.fbs) schema. This allows us to extract the actual structure of the flatbuffer and should make it possible to generate arrow + parquet serialization code just given a Flatbuffer idl.

In practice, this requires the following steps:

1. Generate the binary representation of the flatbuffer schema (encoded according to `reflection.fbs`):

- Change to an example sensor flatbuffer directory from the [opensensor](https://github.com/opensensordotdev/opensensor) repository: `cd opensensor/crates/sensor_simple/flatbuffers`
- `flatc --schema --binary --bfbs-comments simple.fbs`. `--bfbs-comments` includes comments in the binary flatbuffer, which can be used to auto-comment generated Arrow + Parquet code
- This will generate `simple.bfbs`

2. Generate Flatbuffer binding code to read a flatbuffer that's been encoded according to `reflection.fbs`

- The OpenSensor library will automatically do this for Rust through the build script (see `opensensor-rs/src/reflection_generated.rs`)
- You can manually do this by navigating to the directory that contains `reflection.fbs` and execute the `flatc` command to generate code for your language of choice: `flatc --rust reflection.fbs`, `flatc --python reflection.fbs`, etc.

3. Read the `.bfbs` data from the binary representation generated in (1) and load it as a Schema Flatbuffer (Schema is the root type in `reflection.fbs`). In rust it looks like this:

```rust
  use crate::reflection_generated::reflection;
  use std::io::Read;
    
    let mut file = File::open("flatbuffers/simple.bfbs").expect("Filed to open flatbuffer schema file");

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    let schema = reflection::root_as_schema(&buf).expect("Failed to deserialize schema flatbuffer");

    for object in schema.objects() {
        println!("{}", object.name());
        for field in object.fields() {
            println!("{:?}", field.name());
            println!("{:?}", field.type_().base_type())

            // Per the flatbuffer spec, only comments in the original .fbs file with triple slashes will be included here!
            println!("{:?}", field.documentation());
        }
    }
```

### Comment Extraction

Only comments prefaced with triple slash (`///`) will be included in the generated `.bfbs`.
See the [flatbuffer spec](https://google.github.io/flatbuffers/flatbuffers_guide_writing_schema.html), subheading "Comments and Documentation" for more information.
