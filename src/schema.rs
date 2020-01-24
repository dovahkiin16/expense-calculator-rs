table! {
    expenses (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Float4,
        expense_type -> Varchar,
        need -> Bool,
        created_at -> Timestamp,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(expenses -> users (user_id));

allow_tables_to_appear_in_same_query!(expenses, users,);
