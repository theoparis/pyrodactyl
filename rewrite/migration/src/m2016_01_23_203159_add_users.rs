use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table("users")
					.if_not_exists()
					.col(pk_auto("id"))
					.col(char("uuid").char_len(36).not_null().unique_key())
					.col(string("email").not_null().unique_key())
					.col(text("password").not_null())
					.col(string("remember_token").null())
					.col(char("language").char_len(5).default("en"))
					.col(tiny_integer("root_admin").unsigned().default(0))
					.col(tiny_integer("use_totp").unsigned())
					.col(char("totp_secret").char_len(16).null())
					.col(
						timestamp("created_at")
							.default(Expr::current_timestamp()),
					)
					.col(
						timestamp("updated_at")
							.default(Expr::current_timestamp()),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table("users").to_owned())
			.await
	}
}
