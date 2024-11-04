use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::models::schema::users)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub password: String,
    pub name: String,
    pub user_role: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = online_test)]
pub struct OnlineTest {
    pub id: i32,
    pub name: String,
    pub mix_ans: bool,
    pub group_id: i32,
    pub questions_id: i32,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = offline_test)]
pub struct OfflineTest {
    pub id: i32,
    pub questions_id: i32,
    pub name: String,
    pub mix_ans: bool,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = group)]
pub struct Group {
    pub id: String,
    pub owner_id: i32,
    pub users_id: i32,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = achievement)]
pub struct Achievement {
    pub id: String,
    pub group_id: i32,
    pub contest_id: i32,
    pub num_right_ans: i32,
    pub point: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = question)]
pub struct Question {
    pub id: String,
    pub content: String,
    pub grade: i32,
    pub subject: String,
    pub difficulty: String,
    pub typing: String,
    pub ans: String
}





