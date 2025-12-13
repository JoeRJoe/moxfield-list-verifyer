use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Report::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Report::IsValid).boolean().not_null())
                    .col(ColumnDef::new(Report::Name).string().not_null())
                    .col(ColumnDef::new(Report::Author).string().not_null())
                    .col(ColumnDef::new(Report::NonLandTutors).json().not_null())
                    .col(
                        ColumnDef::new(Report::MassLandDenialCards)
                            .json()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Report::CommanderTutors).json().not_null())
                    .col(ColumnDef::new(Report::TwoCardCombos).json().not_null())
                    .col(ColumnDef::new(Report::Gamechangers).json().not_null())
                    .col(
                        ColumnDef::new(Report::InfiniteTurnsCombos)
                            .json()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Report::Combos).json().not_null())
                    .col(ColumnDef::new(Report::DeckList).json().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Report {
    Table,
    Id,
    IsValid,
    Name,
    Author,
    NonLandTutors,
    MassLandDenialCards,
    CommanderTutors,
    TwoCardCombos,
    Gamechangers,
    InfiniteTurnsCombos,
    Combos,
    DeckList,
}
