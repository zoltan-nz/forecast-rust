pub use sea_orm_migration::prelude::*;

mod m20241104_023919_create_cities_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241104_023919_create_cities_table::Migration),
        ]
    }
}
