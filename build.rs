fn main() {
    // Specify the directory containing the IPOPT and HSL libraries
    println!("cargo:rustc-link-search=/usr/local/lib");

    // Link the ipopt library
    println!("cargo:rustc-link-lib=ipopt");

    // Link the HSL library
    println!("cargo:rustc-link-lib=hsl");

    // Set rpath to IPOPT and HSL dynamic libraries for runtime linking
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");

    // Tell cargo to rerun this build script if the libraries change
    println!("cargo:rerun-if-changed=/usr/local/lib/libipopt.dylib");
    println!("cargo:rerun-if-changed=/usr/local/lib/libhsl.dylib");
}
