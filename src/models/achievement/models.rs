#![allow(non_snake_case)]
use crate::{
    error::AppError,
    models::{
        onlinetest::models::OnlineTest,
        schema::{
            achivement,
            online_achivement::{self, achivement_id, online_test_id},
            online_test,
        },
        user::models::User,
    },
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(OnlineTest, foreign_key = contest_id))]
#[diesel(table_name = achivement)]
pub struct Achievement {
    pub id: i32,
    pub num_right_ans: i32,
    pub point: f32,
    pub user_id: i32,
    pub contest_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = achivement)]
pub struct CreateAchivement {
    pub num_right_ans: i32,
    pub point: f32,
    pub user_id: i32,
    pub contest_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = achivement)]
pub struct UpdateAchivement {
    pub num_right_ans: Option<i32>,
    pub point: Option<f32>,
}

impl Achievement {
    pub fn Find(conn: &mut PgConnection, aid: i32) -> Result<Achievement, AppError> {
        let t = achivement::table.find(aid);
        let achv = t.first(conn)?;
        Ok(achv)
    }
    pub fn CreateAchivement(
        conn: &mut PgConnection,
        record: CreateAchivement,
    ) -> Result<Achievement, AppError> {
        let achv = diesel::insert_into(achivement::table)
            .values(record)
            .get_result(conn)?;
        Ok(achv)
    }
    pub fn UpdateAchivement(
        conn: &mut PgConnection,
        aid: i32,
        change_set: UpdateAchivement,
    ) -> Result<Achievement, AppError> {
        let target = achivement::table.find(aid);
        let achv = diesel::update(target).set(change_set).get_result(conn)?;
        Ok(achv)
    }
    pub fn RemoveAchivement(conn: &mut PgConnection, aid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(achivement::table.find(aid)).execute(conn)?;
        Ok(res != 0)
    }
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
#[diesel(belongs_to(Achievement, foreign_key = achivement_id))]
#[diesel(belongs_to(OnlineTest, foreign_key = online_test_id))]
#[diesel(primary_key(achivement_id, online_test_id))]
#[diesel(table_name = online_achivement)]
pub struct OnlineAchivement {
    achivement_id: i32,
    online_test_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = online_achivement)]
pub struct AddAchivementToTest {
    achivement_id: i32,
    online_test_id: i32,
}

impl OnlineAchivement {
    pub fn AddAchivementToTest(conn: &mut PgConnection, aid: i32, tid: i32) -> Result<OnlineAchivement, AppError> {
        let record = AddAchivementToTest {
            achivement_id: aid,
            online_test_id: tid,
        };
        let res = diesel::insert_into(online_achivement::table)
            .values(record)
            .get_result(conn)?;
        Ok(res)
    }
    pub fn RemoveAchivementFromTest(
        conn: &mut PgConnection,
        aid: i32,
        tid: i32,
    ) -> Result<bool, AppError> {
        let res = diesel::delete(
            online_achivement::table
                .filter(achivement_id.eq(aid))
                .filter(online_test_id.eq(tid)),
        )
        .execute(conn)?;
        Ok(res != 0)
    }
    pub fn GetAllAchivementFromTest(
        conn: &mut PgConnection,
        tid: i32,
    ) -> Result<Vec<Achievement>, AppError> {
        let ot = OnlineTest::Find(conn, tid)?;
        let achv = OnlineAchivement::belonging_to(&ot)
            .inner_join(achivement::table)
            .select(Achievement::as_select())
            .load(conn)?;
        Ok(achv)
    }
    pub fn GetAllTestFromAchivement(
        conn: &mut PgConnection,
        aid: i32,
    ) -> Result<Vec<OnlineTest>, AppError> {
        let achv = Achievement::Find(conn, aid)?;
        let ot = OnlineAchivement::belonging_to(&achv)
            .inner_join(online_test::table)
            .select(OnlineTest::as_select())
            .load(conn)?;
        Ok(ot)
    }
}
