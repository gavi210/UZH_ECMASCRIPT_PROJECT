# Tips for Building
- Go to the website https://github.com/denoland/rusty_v8/releases and get the appropriate version of the static rusty_v8 library (for normal macOS x86_64-apple-darwin)
- Copy this file to ./target/debug/gn_out/obj (if there is no such file even if it fails try to build the project first)
- If necessary delete the file librusty_v8.a in there
- Now rename the file you just copied to librusty_v8.a

# Incremental Development
