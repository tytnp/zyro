use entity::model::prelude::*;
#[cfg(test)]
use sea_orm::{Database, EntityTrait};

#[tokio::test]
async fn test() {
    let conn = Database::connect("sqlite://zyro.sqlite?mode=rwc")
        .await
        .unwrap();
    let sys_dictionary: Vec<entity::model::sys_dictionary::Model> =
        SysDictionary::find().all(&conn).await.unwrap();
    println!(" {}", &sys_dictionary.len());
    ()
}
