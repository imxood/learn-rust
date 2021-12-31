fn add_lib(_name: impl AsRef<str>, _static: bool) {
    println!(
        "cargo:rustc-link-lib={}{}",
        if _static { "static=" } else { "" },
        _name.as_ref()
    );
}

fn main() {
    add_lib("c++_shared", false);
}
