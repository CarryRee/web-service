cargo new crate1 --lib  # 创建库 crate
cargo new crate2 --bin  # 创建二进制 crate

// 项目根目录
cargo run --bin auth-service


// sqlx 迁移功能
cargo install sqlx-cli --no-default-features --features native-tls,postgres

🚀 设置指定仓库的提交邮箱和名称

git config user.name "你的名字"
git config user.email "你的邮箱@example.com"
