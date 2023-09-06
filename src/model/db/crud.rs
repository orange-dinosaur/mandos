/* pub async fn create(_ctx: &Ctx, mm: &ModelManager, ua_fc: UserAuthForCreate) -> Result<Uuid> {
    let db = mm.db();

    // Create UserAuth
    let user_auth = UserAuth::new(ua_fc)?;

    let query = "insert into users_auth (id, created_at, updated_at, last_login, needs_verify, is_blocked, username, email, password) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id";
    let (id,) = sqlx::query_as::<_, (Uuid,)>(query)
        .bind(user_auth.id)
        .bind(user_auth.created_at)
        .bind(user_auth.updated_at)
        .bind(user_auth.last_login)
        .bind(user_auth.needs_verify)
        .bind(user_auth.is_blocked)
        .bind(user_auth.username.to_string())
        .bind(user_auth.email.to_string())
        .bind(user_auth.password.to_string())
        .fetch_one(db)
        .await?;

    Ok(id)
} */

/* pub async fn _get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    let query = format!("select * from {} where id = $1", MC::TABLE);
    let entity = sqlx::query_as(&query)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id: id.to_string(),
        })?;

    Ok(entity)
} */

use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::debug;

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
    /* for field_value in fields_values.iter() {
        match field_value {
            IterableType::Uuid(ev) => {
                query_builder.push_bind(ev);
            }
            IterableType::DateTime(ev) => {
                query_builder.push_bind(ev);
            }
            IterableType::Bool(ev) => {
                query_builder.push_bind(ev);
            }
            IterableType::String(ev) => {
                query_builder.push_bind(ev);
            }
        }
    } */
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

    let row = query.fetch_one(&db).await;
    match row {
        Ok(row) => Ok(row),
        Err(e) => {
            debug!("FN: model::db::crud::create - Error: {}", e);
            Err(Error::Sqlx(e))
        }
    }
}
