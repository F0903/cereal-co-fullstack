pub use sea_orm_migration::prelude::*;

mod m20241209_140447_create_product_table;
mod m20241210_075709_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241209_140447_create_product_table::Migration),
            Box::new(m20241210_075709_create_user_table::Migration),
        ]
    }
}
