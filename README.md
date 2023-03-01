# OpenSensor - Traits & generics for implementing new sensors

Generics for implementing new sensors that interact with the OpenSensor Infrastructure as Code.

## Requirements

- [rust](https://www.rust-lang.org/): Minimum Supported Rust Version 1.65
- [lld linker](https://lld.llvm.org/): for faster Rust builds.
  - See [.cargo/config.toml](.cargo/config.toml) for platform-specific installation instructions and the [Rust Performance Book](https://nnethercote.github.io/perf-book/compile-times.html) for the reasons for using lld.
- [docker](https://www.docker.com/): Container engine
- [docker compose](https://docs.docker.com/compose/): Multi-container orchestration. NOTE: `docker-compose` is now deprecated and the compose functionality is integrated into the `docker compose` command. To install alongside an existing docker installation, run `sudo apt-get install docker-compose-plugin`. [ref](https://docs.docker.com/compose/#compose-v2-and-the-new-docker-compose-command).

For Debian-based Linux distros, you can install `opensensor-rs`'s dependencies (except Docker, that require special repository configuration documented above) with the following command:

`apt-get install clang build-essential lld clang zstd libzstd-dev make cmake pkg-config libssl-dev`

`opensensor-rs` is tested on Ubuntu 22.04 LTS, but welcomes pull requests to fix Windows or MacOS issues.

## Quick Start

1. Clone repo: ```git clone https://github.com/opensensordotdev/opensensor-rs.git```
  - Ensure all requirements have been installed, especially the lld linker!  Otherwise `opensensor-rs` won't build!
2. `./bootstrap_cluster.sh`: Start the testing Redpanda, MinIO, and monitoring stack.
3. `cargo test`: Verify all cargo tests pass

## Provided Generics

![OpenSensor architecture](transducer_sensor_background.png)

- `Measurement`: The physical data structure representing the discrete unit of data produced by the sensor. Each `Sensor` can produce multiple kinds of `Measurements`.
- `Transducer`: The interface with sensor hardware, producing a stream of `Measurements`.
- `Sensor`: The abstraction between multiple `Transducers` that produce the same kind of `Measurements`, grouping common validation logic together. Consumes from the [mpsc](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html) channel that the `Transducer` sends `Measurements` on, validates them, and produces to Redpanda.

## Adding New Measurements

`Measurements` are notionally serialized using Google [flatbuffers](https://google.github.io/flatbuffers/). For a sample `Measurement`, `Transducer`, and `Sensor` implementation, see the `sensor-simple` crate in the [opensensor](https://github.com/opensensordotdev/opensensor) repository.

## Arrow + Parquet Archiving

In order to make implementing new sensors as straightforward as possible, `opensensor-rs` seeks to provide automatic archiving of `Measurement` implementers to Parquet through Rust's Arrow bindings. Experiments for archiving arbitrary Rust structs to arrow and then parquet are documented in the `archiver` directory and in `arrow.rs`. Ideally, this functionality would be derivable or implementable through traits to allow arbitrary measurements to be serialized to/from parquet.

Filed this [issue](https://github.com/jorgecarleitao/arrow2/issues/1376) on arrow2, but even though their resulting PR fixed the Rust code (the tests in `test_arrow.rs` now pass), the resulting parquet for any nested arrays or structs still can't be deserialized in pyarrow.

The test cases in `test_arrow.rs` are based on the following examples:

- [arrow2 parquet writer](https://github.com/jorgecarleitao/arrow2/blob/main/examples/parquet_write.rs)
- [arrow2_convert simple example](https://github.com/DataEngineeringLabs/arrow2-convert/blob/main/examples/simple/src/main.rs)
- [arrow2_convert complex example](https://github.com/DataEngineeringLabs/arrow2-convert/blob/main/arrow2_convert/tests/complex_example.rs)
