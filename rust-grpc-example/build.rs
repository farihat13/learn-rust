fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("Generated files will be in: {}", out_dir);
    // tonic_build::compile_protos("proto/calc.proto").unwrap();
    tonic_build::configure()
        .out_dir("src/proto") // Store generated code in src/proto/
        .compile(&["src/proto/calc.proto"], &["proto"])
        .unwrap();
}
