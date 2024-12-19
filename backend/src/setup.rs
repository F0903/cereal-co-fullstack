use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::cell::LazyCell;

const FULL_DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DATABASE_URL: LazyCell<String> =
    LazyCell::new(|| FULL_DATABASE_URL.rsplit_once('/').unwrap().0.to_owned());
const DB_NAME: LazyCell<String> =
    LazyCell::new(|| FULL_DATABASE_URL.rsplit_once('/').unwrap().1.to_owned());

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    println!("Conneting to '{}'...", &*DATABASE_URL);
    // First connect without the database name at the end.
    let db = Database::connect(&*DATABASE_URL).await?;

    // Create DB if it doesn't exist.
    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", *DB_NAME),
            ))
            .await?;

            // Then connect with the database string at the end after.
            Database::connect(FULL_DATABASE_URL).await?
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

            Database::connect(FULL_DATABASE_URL).await?
        }
        DbBackend::Sqlite => db,
    };

    // Bring the database up to speed.
    Migrator::up(&db, None).await?;

    Ok(db)
}
