// @generated automatically by Diesel CLI.

diesel::table! {
    achivement (id) {
        id -> Int4,
        num_right_ans -> Int4,
        point -> Float4,
        user_id -> Int4,
        contest_id -> Int4,
    }
}

diesel::table! {
    group (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
        name -> Varchar,
    }
}

diesel::table! {
    offline_question (question_id, offline_test_id) {
        question_id -> Int4,
        offline_test_id -> Int4,
    }
}

diesel::table! {
    offline_test (id) {
        id -> Int4,
        name -> Varchar,
        mix_ans -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    online_achivement (achivement_id, online_test_id) {
        achivement_id -> Int4,
        online_test_id -> Int4,
    }
}

diesel::table! {
    online_question (question_id, online_test_id) {
        online_test_id -> Int4,
        question_id -> Int4,
    }
}

diesel::table! {
    online_test (id) {
        id -> Int4,
        name -> Varchar,
        mix_ans -> Bool,
        group_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

diesel::table! {
    question (id) {
        id -> Int4,
        content -> Text,
        grade -> Float4,
        subject -> Varchar,
        difficulty -> Varchar,
        ans -> Varchar,
        typing -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Varchar,
        password -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    users_group (group_id, user_id) {
        group_id -> Int4,
        user_id -> Int4,
    }
}

diesel::joinable!(achivement -> online_test (contest_id));
diesel::joinable!(achivement -> users (user_id));
diesel::joinable!(group -> users (owner_id));
diesel::joinable!(offline_question -> offline_test (offline_test_id));
diesel::joinable!(offline_question -> question (question_id));
diesel::joinable!(offline_test -> users (user_id));
diesel::joinable!(online_achivement -> achivement (achivement_id));
diesel::joinable!(online_achivement -> online_test (online_test_id));
diesel::joinable!(online_question -> online_test (online_test_id));
diesel::joinable!(online_question -> question (question_id));
diesel::joinable!(online_test -> group (group_id));
diesel::joinable!(online_test -> users (user_id));
diesel::joinable!(users_group -> group (group_id));
diesel::joinable!(users_group -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    achivement,
    group,
    offline_question,
    offline_test,
    online_achivement,
    online_question,
    online_test,
    question,
    users,
    users_group,
);
