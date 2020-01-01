table! {
    expenses (id) {
        id -> Int4,
        amount -> Float4,
        expense_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    expenses,
    users,
);
