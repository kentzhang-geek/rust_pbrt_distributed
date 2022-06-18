fn main() {
    println!("cargo:rustc-link-search=native=../../../fbx_plugin/cmake-build-relwithdebinfo/");
    println!("cargo:rustc-link-search=native=../../../fbx_plugin/SDK/2020.3.1/lib/vs2019/x64/release/");
    println!("cargo:rustc-link-lib=static=rpd_fbx_plugin");
    println!("cargo:rustc-link-lib=static=libfbxsdk-md");
    println!("cargo:rustc-link-lib=static=libxml2-md");
    println!("cargo:rustc-link-lib=static=zlib-md");
}