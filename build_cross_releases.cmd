rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-msvc
cross build --release --target x86_64-unknown-linux-gnu