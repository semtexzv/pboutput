
fn main() {
    prost_build::Config::new()
        .out_dir("src")
        .compile_protos(&["pboutput.proto"], &["proto"]).unwrap();

    // protokit_codegen::Codegen::new()
    //     .out_dir("src/pkit")
    //     .include("proto")
    //     .compile("pboutput.proto").unwrap()
    //     .generate().unwrap();
}