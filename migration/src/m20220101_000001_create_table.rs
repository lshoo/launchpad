use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Wallet).string())
                    .col(ColumnDef::new(User::CreatedAt).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Assets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Assets::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Assets::AssetId).string().not_null())
                    .col(ColumnDef::new(Assets::Tick).string().not_null())
                    .col(ColumnDef::new(Assets::Name).string().not_null())
                    .col(ColumnDef::new(Assets::TotalSupply).string().not_null())
                    .col(ColumnDef::new(Assets::Data).string())
                    .col(ColumnDef::new(Assets::CreatedAt).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Assets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    CreatedAt,
    Wallet,
}

#[derive(DeriveIden)]
enum Assets {
    Table,
    Id,
    AssetId,
    Tick,
    Name,
    TotalSupply,
    Data,
    CreatedAt,
}
