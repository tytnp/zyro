// async fn test() {
//     let conn = Database::connect("sqlite://zyro.sqlite?mode=rwc")
//         .await
//         .unwrap();
//     let sys_dictionary: Vec<entity::model::sys_dictionary::Model> =
//         SysDictionary::find().all(&conn).await.unwrap();
//     println!(" {}", &sys_dictionary.len());
//     ()
// }


mod db {
    mod test_role {
        use std::error::Error;
        use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
        use sea_orm::ActiveValue::Set;

        // use entity::model::prelude::*;
        use entity;


        async fn connect() -> Result<DatabaseConnection, Box<dyn Error>> {
            Ok(Database::connect("sqlite://zyro.sqlite?mode=rwc").await?)
        }

        #[tokio::test]
        async fn add_role() -> Result<(), Box<dyn Error>> {
            let conn = connect().await?;


            entity::model::sys_role::ActiveModel {
                id: Default::default(),
                name: Set(Some("123".into())),
                default_route: Default::default(),
                status: Default::default(),
                created_at: Default::default(),
                updated_at: Default::default(),
            }.save(&conn).await?;



            // let role = entity::model::sys_role::Model {
            //     id: 0,
            //     name: "None".into(),
            //     default_route: None,
            //     status: None,
            //     created_at: None,
            //     updated_at: None,
            // };
            // conn.ping().await?;
            // conn.close().await?;
            Ok(())
        }
    }
}