use sea_orm_migration::prelude::*;
use sea_query::Expr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cities::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cities::Name).string().not_null())
                    .col(ColumnDef::new(Cities::Lat).float().not_null())
                    .col(ColumnDef::new(Cities::Long).float().not_null())
                    .col(
                        ColumnDef::new(Cities::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cities_name")
                    .table(Cities::Table)
                    .col(Cities::Name)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cities::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Cities {
    Table,
    Id,
    Name,
    Lat,
    Long,
    CreatedAt,
}
