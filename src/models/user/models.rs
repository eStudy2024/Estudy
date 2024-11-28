#![allow(non_snake_case)]
use crate::models::group::models::Group;
use crate::models::onlinetest::models::OnlineTest;
use crate::models::schema::users;
use crate::{error::AppError, models::achievement::models::Achievement};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::utils::{hash_password, verify_password};

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, PartialEq, Identifiable,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub password: String,
    pub name: String,
    pub user_role: String,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct SignupUser<'a> {
    pub display_name: &'a str,
    pub name: &'a str,
    pub password: String,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn Find(conn: &mut PgConnection, id: i32) -> Result<User, AppError> {
        let t = users::table.find(id);
        let user = t.first::<User>(conn)?;
        Ok(user)
    }

    pub fn FindByUsername(conn: &mut PgConnection, username: &str) -> Result<User, AppError> {
        let t = users::table.filter(users::name.eq(username));
        let user = t.first::<User>(conn)?;
        Ok(user)
    }

    pub fn SignUp<'a>(conn: &mut PgConnection, mut record: SignupUser) -> Result<User, AppError> {
        record.password = hash_password(&record.password)?;
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn SignIn(
        conn: &mut PgConnection,
        username: &str,
        naive_pw: &str,
    ) -> Result<User, AppError> {
        let user = Self::FindByUsername(conn, username)?;
        if !verify_password(naive_pw, &user.password)? {
            return Err(AppError::Forbidden(json!({"error": "User not found"})));
        }
        Ok(user)
    }
    pub fn UpdateUser(
        conn: &mut PgConnection,
        user_id: i32,
        change_set: UpdateUser,
    ) -> Result<Self, AppError> {
        let target = users::table.find(user_id);
        let user = diesel::update(target).set(change_set).get_result(conn)?;
        Ok(user)
    }
    pub fn DeleteUser(conn: &mut PgConnection, uid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(users::table.find(uid)).execute(conn)?;
        Ok(res != 0)
    }
    pub fn GetAllAchivement(
        conn: &mut PgConnection,
        uid: i32,
    ) -> Result<Vec<Achievement>, AppError> {
        let usr = User::Find(conn, uid)?;
        let achv = Achievement::belonging_to(&usr)
            .select(Achievement::as_select())
            .load(conn)?;
        Ok(achv)
    }
    pub fn GetAllGroupUserOwn(conn: &mut PgConnection, uid: i32) -> Result<Vec<Group>, AppError> {
        let usr = User::Find(conn, uid)?;
        let grps = Group::belonging_to(&usr)
            .select(Group::as_select())
            .load(conn)?;
        Ok(grps)
    }
    pub fn GetAllTestUserOwn(
        conn: &mut PgConnection,
        uid: i32,
    ) -> Result<Vec<OnlineTest>, AppError> {
        let usr = User::Find(conn, uid)?;
        let tst = OnlineTest::belonging_to(&usr)
            .select(OnlineTest::as_select())
            .load(conn)?;
        Ok(tst)
    }
}
