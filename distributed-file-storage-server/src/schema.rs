// @generated automatically by Diesel CLI.

diesel::table! {
    chunks (id) {
        id -> Uuid,
        file_id -> Uuid,
        chunk_num -> Int4,
        data -> Bytea,
    }
}

diesel::table! {
    files (id) {
        id -> Uuid,
        name -> Text,
        chunk_count -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(chunks, files,);
