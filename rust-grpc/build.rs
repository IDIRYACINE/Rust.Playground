fn main() {
    tonic_build::configure()
    .build_server(true)
    .compile(
        &["proto/userRegister.proto"],
        &["proto/"], 
    ).unwrap();

}