use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("sqlite://zyro.sqlite?mode=rwc").await.unwrap();
    db.close().await.unwrap();
}