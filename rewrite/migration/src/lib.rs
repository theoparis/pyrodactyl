pub use sea_orm_migration::prelude::*;

mod m2016_01_23_203159_add_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![Box::new(m2016_01_23_203159_add_users::Migration)]
	}
}
