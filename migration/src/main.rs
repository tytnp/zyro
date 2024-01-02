use sea_orm_migration::prelude::*;
// use migration::Migrator;
// use migration::sea_orm::{Database, DatabaseConnection};

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
    // let db: DatabaseConnection = Database::connect("sqlite://zyro.sqlite?mode=rwc").await.unwrap();
    // Migrator::fresh(&db).await.unwrap();
}
//cargo install sea-orm-cli                                                 //  安装 sea-orm-cli
//sea-orm-cli migrate generate name                                         //  创建迁移文件
//sea-orm-cli migrate fresh                                                 //  迁移
//sea-orm-cli generate entity --with-serde both -o entity/src/model         //  generate entity 命令
