//! This file is used to compile the resources for the Windows executable.

fn main() {
    // Enable the following line to compile the resources only in release mode if it slows down the build.
    //#[cfg(not(debug_assertions))]
    {
        extern crate embed_resource;
        use std::env;
        let target = env::var("TARGET").unwrap();
        if target.contains("windows") {
            // Set an icon for the executable
            let _ = embed_resource::compile("../../assets/windows/icon.rc", embed_resource::NONE);
        }
    }
}
