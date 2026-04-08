use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysPermissionCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysPermissionCategory::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SysPermissionCategory::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysPermissionCategory::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SysPermissionCategory::Sort)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysPermissionCategory::Remark)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SysPermissionCategory::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysPermissionCategory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum SysPermissionCategory {
    Table,
    Id,
    Name,
    Code,
    Sort,
    Remark,
    CreatedAt,
}
