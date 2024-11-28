#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    models::{
        question::models::Question,
        schema::{
            offline_question::{self, offline_test_id, question_id},
            offline_test, question,
        },
        user::models::User,
    },
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Identifiable,
    Associations,
    Selectable,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = offline_test)]
pub struct OfflineTest {
    pub id: i32,
    pub name: String,
    pub mix_ans: bool,
    pub user_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = offline_test)]
pub struct CreateOfflineTest<'a> {
    pub id: i32,
    pub name: &'a str,
    pub mix_ans: bool,
    pub user_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = offline_test)]
pub struct UpdateOfflineTest {
    pub name: Option<String>,
    pub mix_ans: Option<bool>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Selectable,
    PartialEq,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(Question, foreign_key=question_id))]
#[diesel(belongs_to(OfflineTest, foreign_key=offline_test_id))]
#[diesel(primary_key(offline_test_id, question_id))]
#[diesel(table_name = offline_question)]
pub struct OfflineQuestion {
    pub offline_test_id: i32,
    pub question_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = offline_question)]
pub struct AddQuestionToOfflineTest {
    pub offline_test_id: i32,
    pub question_id: i32,
}

impl OfflineTest {
    pub fn Find(conn: &mut PgConnection, id: i32) -> Result<OfflineTest, AppError> {
        let t = offline_test::table.find(id);
        let ot = t.first(conn)?;
        Ok(ot)
    }
    pub fn CreateTest(
        conn: &mut PgConnection,
        record: CreateOfflineTest,
    ) -> Result<OfflineTest, AppError> {
        let ot = diesel::insert_into(offline_test::table)
            .values(record)
            .get_result(conn)?;
        Ok(ot)
    }
    pub fn UpdateOfflineTest(
        conn: &mut PgConnection,
        tid: i32,
        change_set: UpdateOfflineTest,
    ) -> Result<OfflineTest, AppError> {
        let target = offline_test::table.find(tid);
        let ot = diesel::update(target).set(change_set).get_result(conn)?;
        Ok(ot)
    }
    pub fn RemoveOfflineTest(conn: &mut PgConnection, tid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(offline_test::table.find(tid)).execute(conn)?;
        Ok(res != 0)
    }
    pub fn AddQuestionToTest(
        conn: &mut PgConnection,
        qid: i32,
        tid: i32,
    ) -> Result<bool, AppError> {
        let record = AddQuestionToOfflineTest {
            offline_test_id: tid,
            question_id: qid,
        };
        let qt = diesel::insert_into(offline_question::table)
            .values(record)
            .execute(conn)?;
        Ok(qt != 0)
    }
}

impl OfflineQuestion {
    fn _Find(conn: &mut PgConnection, tid: i32, qid: i32) -> Result<OfflineQuestion, AppError> {
        let t = offline_question::table.find((tid, qid));
        let oq = t.first(conn)?;
        Ok(oq)
    }
    pub fn GetAllQuestionFromTest(
        conn: &mut PgConnection,
        tid: i32,
    ) -> Result<Vec<Question>, AppError> {
        let test = OfflineTest::Find(conn, tid)?;
        let qs = OfflineQuestion::belonging_to(&test)
            .inner_join(question::table)
            .select(Question::as_select())
            .load::<Question>(conn)?;
        Ok(qs)
    }
    pub fn RemoveQuestionFromTest(
        conn: &mut PgConnection,
        tid: i32,
        qid: i32,
    ) -> Result<bool, AppError> {
        let res = diesel::delete(offline_question::table.filter(question_id.eq(qid)))
            .filter(offline_test_id.eq(tid))
            .execute(conn)?;
        Ok(res != 0)
    }
    pub fn GetAllTestHaveQuestion(
        conn: &mut PgConnection,
        qid: i32,
    ) -> Result<Vec<OfflineTest>, AppError> {
        let qs = Question::Find(conn, qid)?;
        let t = OfflineQuestion::belonging_to(&qs)
            .inner_join(offline_test::table)
            .select(OfflineTest::as_select())
            .load::<OfflineTest>(conn)?;
        Ok(t)
    }
}
