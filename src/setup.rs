use std::cell::LazyCell;

use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DB_NAME: LazyCell<&str> = LazyCell::new(|| DATABASE_URL.rsplit_once('/').unwrap().1);

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    // Create DB if it doesn't exist.
    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", *DB_NAME),
            ))
            .await?;

            Database::connect(DATABASE_URL).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", *DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", *DB_NAME),
            ))
            .await?;

            Database::connect(DATABASE_URL).await?
        }
        DbBackend::Sqlite => db,
    };

    // Bring the database up to speed.
    Migrator::up(&db, None).await?;

    Ok(db)
}
