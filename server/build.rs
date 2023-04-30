extern crate glob;
extern crate tonic_build;
use glob::glob;

fn generate_proto() {
    let proto_paths: Vec<_> = glob("./proto/**/*.proto")
    .map(|entry| entry).expect("failed creating proto paths")
    .filter_map(|result| result.ok()) // keep only Ok values
    .collect();

    tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .compile(&proto_paths, &["./proto"])
    .expect("failed generating protos") 
} 

fn main() { 
    println!("cargo:rerun-if-changed=/proto");
    generate_proto();
}