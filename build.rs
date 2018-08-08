extern crate cc;

fn main() {

let mut cc = cc::Build::new();
cc.warnings(true);

cc.file("src/rust1c.c");
cc.include("d:/devel/harbour/include");
cc.compile("librust1c.a");

// libhello.dll
println!("cargo:rustc-link-lib=dylib=hello");
println!("cargo:rustc-link-search=native=d:/devel/rust1");



println!("cargo:rustc-link-lib=dylib=kernel32");
println!("cargo:rustc-link-lib=dylib=user32");
println!("cargo:rustc-link-lib=dylib=gdi32");
println!("cargo:rustc-link-lib=dylib=advapi32");
println!("cargo:rustc-link-lib=dylib=ws2_32");
println!("cargo:rustc-link-lib=dylib=winspool");
println!("cargo:rustc-link-lib=dylib=comctl32");
println!("cargo:rustc-link-lib=dylib=comdlg32");
println!("cargo:rustc-link-lib=dylib=shell32");
println!("cargo:rustc-link-lib=dylib=uuid");
println!("cargo:rustc-link-lib=dylib=ole32");
println!("cargo:rustc-link-lib=dylib=oleaut32");
println!("cargo:rustc-link-lib=dylib=winmm");
println!("cargo:rustc-link-lib=dylib=iphlpapi");

println!("cargo:rustc-link-lib=dylib=hbcplr");
println!("cargo:rustc-link-lib=dylib=harbour_dll");
println!("cargo:rustc-link-search=native=d:/devel/harbour/lib/win/mingw");


}