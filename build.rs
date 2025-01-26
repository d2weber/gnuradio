fn main() {
    let _build = cxx_build::bridge("gnuradio-runtime/apps/gnuradio-config-info.rs");

    println!("cargo:rerun-if-changed=gnuradio-runtime/apps/gnuradio-config-info.rs");
}
