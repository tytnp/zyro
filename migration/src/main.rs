use sea_orm_migration::prelude::*;
use migration::Migrator;
use migration::sea_orm::{Database, DatabaseConnection};

#[async_std::main]
async fn main() {
    //cli::run_cli(migration::Migrator).await;

    //sea-orm-cli generate entity -o entity/src/model   // generate entity 命令
    let db: DatabaseConnection = Database::connect("sqlite://zyro.sqlite?mode=rwc").await.unwrap();
    Migrator::fresh(&db).await.unwrap();
}
