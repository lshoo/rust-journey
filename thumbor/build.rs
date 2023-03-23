fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["protos/abi.proto"], &["."])
        .unwrap();
}
