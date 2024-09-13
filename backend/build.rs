fn main() {
    println!("cargo:rustc-env=SH_VERSION=main");
    println!("cargo:rustc-env=SH_GITHASH=current");
}
