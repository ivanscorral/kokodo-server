// @generated automatically by Diesel CLI.

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    todo_tags (todo_id, tag_id) {
        todo_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    todos (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(todo_tags -> tags (tag_id));
diesel::joinable!(todo_tags -> todos (todo_id));
diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tags,
    todo_tags,
    todos,
    users,
);
