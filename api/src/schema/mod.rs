use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

mod account_emails;
mod accounts;
mod route_policies;
mod route_policy_scopes;

pub async fn apply(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let manager = SchemaManager::new(conn);

    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        "CREATE EXTENSION IF NOT EXISTS pgcrypto".to_string(),
    ))
    .await?;

    accounts::apply(&manager, conn).await?;
    account_emails::apply(&manager, conn).await?;
    route_policies::apply(&manager, conn).await?;
    route_policy_scopes::apply(&manager, conn).await?;
    apply_audit_invariants(conn).await?;

    Ok(())
}

async fn apply_audit_invariants(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
"#
        .to_string(),
    ))
    .await?;

    for table in ["accounts", "route_policies"] {
        let trigger_name = format!("trg_{}_set_updated_at", table);
        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            format!(
                r#"
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_trigger
    WHERE tgname = '{trigger_name}'
      AND tgrelid = '{table}'::regclass
  ) THEN
    EXECUTE 'CREATE TRIGGER {trigger_name}
             BEFORE UPDATE ON {table}
             FOR EACH ROW
             EXECUTE FUNCTION set_updated_at()';
  END IF;
END $$;
"#
            ),
        ))
        .await?;

    }

    for table in ["accounts"] {
        let constraint_name = format!("{}_deleted_pair_check", table);
        conn.execute(Statement::from_string(
            DbBackend::Postgres,
            format!(
                r#"
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = '{constraint_name}'
      AND conrelid = '{table}'::regclass
  ) THEN
    EXECUTE 'ALTER TABLE {table}
             ADD CONSTRAINT {constraint_name}
             CHECK ((deleted_at IS NULL) = (deleted_by IS NULL))';
  END IF;
END $$;
"#
            ),
        ))
        .await?;
    }

    Ok(())
}
