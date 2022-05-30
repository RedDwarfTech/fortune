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
    }
}
