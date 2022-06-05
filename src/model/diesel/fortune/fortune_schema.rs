table! {
    bill_record (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        user_id -> Int8,
        contents_id -> Int4,
        remark -> Nullable<Varchar>,
    }
}

table! {
    fortune_contents (id) {
        id -> Int4,
        parent_id -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        name -> Varchar,
        contents_type -> Int4,
        deleted -> Int4,
        hidden -> Int4,
        sort -> Int4,
        contents_source -> Nullable<Int4>,
    }
}

table! {
    fortune_user_contents (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        sort -> Int4,
        user_id -> Int8,
        contents_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    bill_record,
    fortune_contents,
    fortune_user_contents,
);
