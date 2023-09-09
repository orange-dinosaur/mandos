use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::debug;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::iterable::{Iterable, IterableType};

use super::{Db, DbRow};

// returns the created row
pub async fn create<T>(db: Db, table_name: &str, struct_to_create: T) -> Result<DbRow>
where
    T: Default + Iterable,
{
    let (fields_names, fields_values) = struct_to_create.get_fields();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
        // spaces as that might interfere with identifiers or quoted strings where exact
        // values may matter.
        format!(
            "insert into {} ({}) values (",
            table_name,
            fields_names.join(", ")
        ),
    );

    // TODO: extract the mathch into a function
    let mut separated = query_builder.separated(", ");
    fields_values.iter().for_each(|v| match v {
        IterableType::Uuid(ev) => {
            separated.push_bind(ev);
        }
        IterableType::DateTime(ev) => {
            separated.push_bind(ev);
        }
        IterableType::Bool(ev) => {
            separated.push_bind(ev);
        }
        IterableType::String(ev) => {
            separated.push_bind(ev);
        }
    });
    separated.push_unseparated(") returning *");

    let query = query_builder.build();

    debug!("FN: model::db::crud::create - Query: {}", query.sql());

    let row = query.fetch_one(&db).await.map_err(Error::Sqlx)?;

    Ok(row)
}

pub async fn _get_one_by_id(db: Db, table_name: &str, id: Uuid) -> Result<DbRow> {
    let query = format!("select * from {} where id = $1", table_name);

    let row = sqlx::query(&query)
        .bind(id)
        .fetch_one(&db)
        .await
        .map_err(Error::Sqlx)?;

    Ok(row)
}

pub async fn get_one_by_field(
    db: Db,
    table_name: &str,
    field_name: &str,
    field_value: IterableType,
) -> Result<DbRow> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
        // spaces as that might interfere with identifiers or quoted strings where exact
        // values may matter.
        format!("select * from {} where {} = ", table_name, field_name,),
    );

    // TODO: extract the mathch into a function
    match field_value {
        IterableType::Uuid(fv) => {
            query_builder.push_bind(fv);
        }
        IterableType::DateTime(fv) => {
            query_builder.push_bind(fv);
        }
        IterableType::Bool(fv) => {
            query_builder.push_bind(fv);
        }
        IterableType::String(fv) => {
            query_builder.push_bind(fv);
        }
    };

    let query = query_builder.build();

    debug!(
        "FN: model::db::crud::get_one_by_field - Query: {}",
        query.sql()
    );

    let row = query.fetch_one(&db).await.map_err(Error::Sqlx)?;

    Ok(row)
}

pub async fn get_all(db: Db, table_name: &str) -> Result<Vec<DbRow>> {
    let query = format!("select * from {}", table_name);

    let rows = sqlx::query(&query)
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?;

    Ok(rows)
}

// TODO: if row not found return dynamic entity
pub async fn update_by_id<T>(db: Db, table_name: &str, struct_for_update: T, id: Uuid) -> Result<()>
where
    T: Iterable,
{
    let (fields_names, fields_values) = struct_for_update.get_fields();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
        // spaces as that might interfere with identifiers or quoted strings where exact
        // values may matter.
        format!("update {} set ", table_name),
    );

    let mut separated = query_builder.separated("");
    for (i, field_name) in fields_names.iter().enumerate() {
        separated.push_unseparated(field_name);
        separated.push_unseparated(" = ");
        match &fields_values[i] {
            IterableType::Uuid(ev) => {
                separated.push_bind(ev);
            }
            IterableType::DateTime(ev) => {
                separated.push_bind(ev);
            }
            IterableType::Bool(ev) => {
                separated.push_bind(ev);
            }
            IterableType::String(ev) => {
                separated.push_bind(ev);
            }
        }

        if i < fields_names.len() - 1 {
            separated.push_unseparated(", ");
        }
    }

    separated.push_unseparated(" where id = ");
    separated.push_bind(id);

    let query = query_builder.build();

    debug!("FN: model::db::crud::update - Query: {}", query.sql());

    let rows_affected = query
        .execute(&db)
        .await
        .map_err(Error::Sqlx)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::SqlxEntityNotFound {
            entity: "table_name",
            id: id.to_string(),
        });
    }

    Ok(())
}

// TODO: if row not found return dynamic entity
pub async fn delete_by_id(db: Db, table_name: &str, id: Uuid) -> Result<()> {
    let query = format!("delete from {} where id = $1", table_name);

    let rows_affected = sqlx::query(&query)
        .bind(id)
        .execute(&db)
        .await
        .map_err(Error::Sqlx)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::SqlxEntityNotFound {
            entity: "table_name",
            id: id.to_string(),
        });
    }

    Ok(())
}
