use std::path::Path;

fn build_flatbuffers() {
    let flatbuffer_paths = [Path::new("flatbuffers/reflection.fbs")];
    flatc_rust::run(flatc_rust::Args {
        inputs: &flatbuffer_paths,
        out_dir: Path::new("src"),
        ..Default::default()
    })
    .expect("flatc");
}

fn main() {
    build_flatbuffers();
}