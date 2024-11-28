#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::group::models::Group;
use crate::models::question::models::Question;
use crate::models::schema::{
    online_question,
    online_test, question,
    online_question::{
        question_id,
        online_test_id
    }
};
use crate::error::AppError;
use crate::models::user::models::User;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = online_test)]
pub struct OnlineTest {
    pub id: i32,
    pub name: String,
    pub mix_ans: bool,
    pub group_id: Option<i32>,
    pub user_id: i32,
}

#[derive(Insertable,Debug, Deserialize, AsChangeset)]
#[diesel(table_name = online_test)]
pub struct CreateOnlineTest<'a> {
    pub id: i32,
    pub name: &'a str,
    pub mix_ans: bool,
    pub group_id: Option<i32>,
    pub user_id: i32,
}

#[derive(Insertable,Debug, Deserialize, AsChangeset)]
#[diesel(table_name = online_test)]
pub struct UpdateOnlineTest {
    pub name: Option<String>,
    pub mix_ans: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, PartialEq, Identifiable, Associations)]
#[diesel(belongs_to(Question, foreign_key=question_id))]
#[diesel(belongs_to(OnlineTest, foreign_key=online_test_id))]
#[diesel(table_name = online_question)]
#[primary_key(online_test_id,question_id)]
pub struct OnlineQuestion {
    pub online_test_id: i32,
    pub question_id: i32
}

#[derive(Insertable,Debug, Deserialize, AsChangeset)]
#[diesel(table_name = online_question)]
pub struct AddQuestionToOnlineTest{
    pub online_test_id: i32,
    pub question_id: i32
}

impl OnlineTest {
    pub fn Find(conn: &mut PgConnection, id:i32) -> Result<OnlineTest,AppError> {
        let t = online_test::table.find(id);
        let ot = t.first(conn)?;
        Ok(ot)
    }
    pub fn CreateTest(conn: &mut PgConnection, record: CreateOnlineTest) -> Result<OnlineTest, AppError> {
        let ot = diesel::insert_into(online_test::table)
            .values(record)
            .get_result(conn)?;
        Ok(ot)
    }
    pub fn UpdateOnlineTest(conn: &mut PgConnection, tid: i32, change_set: UpdateOnlineTest) -> Result<OnlineTest,AppError> {
        let target = online_test::table.find(tid);
        let ot = diesel::update(target)
            .set(change_set)
            .get_result(conn)?;
        Ok(ot)
    }
    pub fn RemoveOnlineTest(conn: &mut PgConnection, tid: i32) -> Result<bool,AppError> {
        let res = diesel::delete(online_test::table.find(tid))
            .execute(conn)?;
        Ok(res != 0)
    }
    pub fn AddQuestionToTest(conn: &mut PgConnection, qid: i32, tid: i32) -> Result<bool, AppError> {
        let record = AddQuestionToOnlineTest{
            question_id: qid,
            online_test_id: tid
        };
        let qt = diesel::insert_into(online_question::table)
            .values(record)
            .execute(conn)?;
        Ok(qt != 0)
    }
}

impl OnlineQuestion {
    fn _Find(conn: &mut PgConnection, tid: i32, qid: i32) -> Result<OnlineQuestion,AppError> {
        let t = online_question::table.find((tid,qid));
        let oq = t.first(conn)?;
        Ok(oq)
    }
    pub fn GetAllQuestionFromTest(conn: &mut PgConnection, tid: i32) -> Result<Vec<Question>, AppError> {
        let test = OnlineTest::Find(conn, tid)?;
        let qs = OnlineQuestion::belonging_to(&test)
            .inner_join(question::table)
            .select(Question::as_select())
            .load::<Question>(conn)?;
        Ok(qs)
    }
    pub fn RemoveQuestionFromTest(conn: &mut PgConnection, tid: i32, qid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(online_question::table.filter(question_id.eq(qid)))
            .filter(online_test_id.eq(tid))
            .execute(conn)?;
        Ok(res != 0)
    }
    pub fn GetAllTestHaveQuestion(conn: &mut PgConnection, qid: i32) -> Result<Vec<OnlineTest>, AppError> {
        let qs = Question::Find(conn, qid)?;
        let t = OnlineQuestion::belonging_to(&qs)
            .inner_join(online_test::table)
            .select(OnlineTest::as_select())
            .load::<OnlineTest>(conn)?;
        Ok(t)
    }
}