fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("failed to locate protoc");
    std::env::set_var("PROTOC", protoc);
    println!("cargo:rerun-if-changed=../proto/mail/v1/mail.proto");
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["../proto/mail/v1/mail.proto"], &["../proto"])
        .expect("failed to compile mail proto");
}
