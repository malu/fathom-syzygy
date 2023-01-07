fn main() {
    println!("cargo:rerun-if-changed=git/fathom");
    cc::Build::new()
        .include("git/fathom/src")
        .file("git/fathom/src/tbprobe.c")
        .define("TB_NO_HELPER_API", None)
        .compile("fathom")
}
