fn main() {
    println!("cargo:rustc-link-search=native=/root/tmp_project/morpheus/python_app/env/lib/");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/root/tmp_project/morpheus/python_app/env/lib/");
}
