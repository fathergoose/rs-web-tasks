use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = crate::schema::things)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Thing {
    pub id: i32,
    pub title: String,
    pub details: Option<String>,
    pub active: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Insertable)]
#[diesel(table_name = crate::schema::things)]
pub struct NewThing<'a> {
    pub title: &'a str,
    pub details: &'a str,
}
