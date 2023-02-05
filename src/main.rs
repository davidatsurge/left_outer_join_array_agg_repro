use diesel_migrations::MigrationHarness;
use std::error::Error;

use diesel::{insert_into, prelude::*, sql_types::SingleValue};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

mod schema {
    // @generated automatically by Diesel CLI.
    diesel::table! {
        descendants (id) {
            id -> Text,
            parent_id -> Nullable<Text>,
        }
    }

    diesel::table! {
        parents (id) {
            id -> Text,
        }
    }

    diesel::joinable!(descendants -> parents (parent_id));

    diesel::allow_tables_to_appear_in_same_query!(descendants, parents,);
}

// I _think_ this is correct.
sql_function! {
    #[aggregate]
    #[sql_name = "ARRAY_AGG"]
    fn array_agg<ST :SingleValue >(x: ST) -> Array<ST>;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    use schema::*;

    // Setup migrations + mock data
    let database_url = dotenv::var("DATABASE_URL")?;
    let mut conn = PgConnection::establish(&database_url)?;
    conn.run_pending_migrations(MIGRATIONS)?;
    insert_into(parents::table)
        .values([parents::id.eq("first")])
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .unwrap();

    // Reproduce bug
    parents::table
        .left_join(descendants::table)
        // HERE: This is the cause of the UnexpectedNullError. I believe diesel should reject this
        // code. The correct way to perform an ARRAY_AGG over a nullable column should instead be
        // `.select(array_agg(descendants::id.nullable()))`, which diesel already accepts.
        .select(array_agg(descendants::id).nullable())
        .load::<Option<Vec<String>>>(&mut conn)?;

    Ok(())
}
