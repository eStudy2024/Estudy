#![allow(non_snake_case)]
use crate::error::AppError;
use crate::models::onlinetest::models::OnlineTest;
use crate::models::schema::{
    group, users, users_group,
    users_group::{group_id, user_id},
};
use crate::models::user::models::User;
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
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = group)]
pub struct Group {
    pub id: i32,
    pub owner_id: Option<i32>,
    pub name: String,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = group)]
pub struct CreateGroup<'a> {
    pub name: &'a str,
    pub owner_id: i32,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = group)]
pub struct UpdateGroup {
    pub name: Option<String>,
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
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = users_group)]
#[diesel(primary_key(user_id, group_id))]
struct UsersGroup {
    user_id: i32,
    group_id: i32,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users_group)]
struct AddUserToGroup {
    user_id: i32,
    group_id: i32,
}

impl Group {
    pub fn Find(conn: &mut PgConnection, id: i32) -> Result<Group, AppError> {
        let t = group::table.find(id);
        let gr = t.first::<Group>(conn)?;
        Ok(gr)
    }
    pub fn CreateGroup(conn: &mut PgConnection, record: CreateGroup) -> Result<Group, AppError> {
        let group = diesel::insert_into(group::table)
            .values(record)
            .get_result::<Group>(conn)?;
        Ok(group)
    }

    pub fn UpdateGroup(
        conn: &mut PgConnection,
        gid: i32,
        change_set: UpdateGroup,
    ) -> Result<Group, AppError> {
        let target = group::table.find(gid);
        let group = diesel::update(target).set(change_set).get_result(conn)?;
        Ok(group)
    }

    pub fn RemoveGroup(conn: &mut PgConnection, gid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(group::table.find(gid)).execute(conn)?;
        Ok(res != 0)
    }

    pub fn GetAllGroupUserIn(conn: &mut PgConnection, uid: i32) -> Result<Vec<Group>, AppError> {
        UsersGroup::GetAllGroupUserIn(conn, uid)
    }
    pub fn GetAllUserInGroup(conn: &mut PgConnection, gid: i32) -> Result<Vec<User>, AppError> {
        UsersGroup::GetAllUserInGroup(conn, gid)
    }
    pub fn RemoveUserFromGroup(
        conn: &mut PgConnection,
        gid: i32,
        uid: i32,
    ) -> Result<bool, AppError> {
        UsersGroup::RemoveUserFromGroup(conn, uid, gid)
    }
    pub fn AddUserToGroup(conn: &mut PgConnection, gid: i32, uid: i32) -> Result<bool, AppError> {
        let record = AddUserToGroup {
            user_id: uid,
            group_id: gid,
        };
        let ug = diesel::insert_into(users_group::table)
            .values(record)
            .execute(conn)?;
        Ok(ug != 0)
    }

    pub fn GetAllTestGroupOwn(
        conn: &mut PgConnection,
        gid: i32,
    ) -> Result<Vec<OnlineTest>, AppError> {
        let grp = Group::Find(conn, gid)?;
        let tst = OnlineTest::belonging_to(&grp)
            .select(OnlineTest::as_select())
            .load(conn)?;
        Ok(tst)
    }
}

impl UsersGroup {
    fn _Find(conn: &mut PgConnection, gid: i32, uid: i32) -> Result<UsersGroup, AppError> {
        let usr_grp = users_group::table
            .filter(group_id.eq(gid))
            .filter(user_id.eq(uid))
            .first(conn)?;
        Ok(usr_grp)
    }
    fn GetAllUserInGroup(conn: &mut PgConnection, gid: i32) -> Result<Vec<User>, AppError> {
        let grp = Group::Find(conn, gid)?;
        let usrs = UsersGroup::belonging_to(&grp)
            .inner_join(users::table)
            .select(User::as_select())
            .load(conn)?;
        Ok(usrs)
    }
    fn GetAllGroupUserIn(conn: &mut PgConnection, uid: i32) -> Result<Vec<Group>, AppError> {
        let usr = User::Find(conn, uid)?;
        let grps = UsersGroup::belonging_to(&usr)
            .inner_join(group::table)
            .select(Group::as_select())
            .load(conn)?;
        Ok(grps)
    }
    fn RemoveUserFromGroup(conn: &mut PgConnection, uid: i32, gid: i32) -> Result<bool, AppError> {
        let res = diesel::delete(
            users_group::table
                .filter(group_id.eq(gid))
                .filter(user_id.eq(uid)),
        )
        .execute(conn)?;
        Ok(res != 0)
    }
}
