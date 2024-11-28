#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::{error::AppError, models::schema::question};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = question)]
pub struct Question {
    pub id: i32,
    pub content: String,
    pub grade: f32,
    pub subject: String,
    pub difficulty: String,
    pub typing: String,
    pub ans: String
}

#[derive(Insertable,Debug, Deserialize, AsChangeset)]
#[diesel(table_name = question)]
pub struct UpdateQuestion {
    pub content: Option<String>,
    pub grade: Option<f32>,
    pub subject: Option<String>,
    pub difficulty: Option<String>,
    pub typing: Option<String>,
    pub ans: Option<String>
}

#[derive(Insertable,Debug, Deserialize, AsChangeset)]
#[diesel(table_name = question)]
pub struct CreateQuestion<'a> {
    pub content: &'a str,
    pub grade: f32,
    pub subject: &'a str,
    pub difficulty: &'a str,
    pub typing: &'a str,
    pub ans: &'a str
}

impl Question {
    pub fn Find(conn: &mut PgConnection, id: i32) -> Result<Question,AppError> {
        let t = question::table.find(id);
        let qt = t.first(conn)?;
        Ok(qt)
    }
    pub fn CreateQuestion(conn: &mut PgConnection, record: CreateQuestion) -> Result<Question,AppError> {
        let qt = diesel::insert_into(question::table)
            .values(record)
            .get_result(conn)?;
        Ok(qt)
    }
    pub fn UpdateQuestion(conn: &mut PgConnection, qid: i32, change_set: UpdateQuestion) -> Result<Question,AppError> {
        let target = question::table.find(qid);
        let qt = diesel::update(target)
            .set(change_set)
            .get_result(conn)?;
        Ok(qt)
    }
    pub fn RemoveQuestion(conn: &mut PgConnection, qid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(question::table.find(qid))
            .execute(conn)?;
        Ok(res != 0)
    }
}