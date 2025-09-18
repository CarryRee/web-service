cargo new crate1 --lib  # 创建库 crate
cargo new crate2 --bin  # 创建二进制 crate

// 项目根目录
cargo run --bin auth-service


// sqlx 迁移功能
cargo install sqlx-cli --no-default-features --features native-tls,postgres
