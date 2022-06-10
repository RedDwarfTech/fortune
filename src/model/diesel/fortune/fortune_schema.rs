table! {
    bill_book (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        creator -> Int8,
        bill_book_template_id -> Int8,
        name -> Varchar,
        contents -> Varchar,
    }
}

table! {
    bill_book_contents (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        creator -> Int8,
        bill_book_id -> Int8,
        name -> Varchar,
        contents -> Varchar,
        content_id -> Int8,
        parent_id -> Int8,
        contents_type -> Int4,
    }
}

table! {
    bill_book_template (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        remark -> Varchar,
        name -> Varchar,
        tags -> Varchar,
        slogan -> Varchar,
        icon_url -> Varchar,
        user_count -> Int8,
        template_type -> Int4,
        custom -> Int4,
        creator -> Int8,
        owner -> Int8,
        online -> Int4,
    }
}

table! {
    bill_book_template_contents (id) {
        id -> Int8,
        parent_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        name -> Varchar,
        contents_type -> Int4,
        deleted -> Int4,
        hidden -> Int4,
        sort -> Int4,
        contents_source -> Int4,
        bill_book_template_id -> Int8,
    }
}

table! {
    bill_record (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        user_id -> Int8,
        bill_book_id -> Int8,
        remark -> Nullable<Varchar>,
        amount -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    bill_book,
    bill_book_contents,
    bill_book_template,
    bill_book_template_contents,
    bill_record,
);
