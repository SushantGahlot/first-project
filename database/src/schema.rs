// @generated automatically by Diesel CLI.

diesel::table! {
    author (authorid) {
        authorid -> Int4,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    author_post (id) {
        id -> Int4,
        authorid -> Int4,
        postid -> Int4,
    }
}

diesel::table! {
    post (postid) {
        body -> Nullable<Varchar>,
        created -> Nullable<Timestamp>,
        postid -> Int4,
        title -> Varchar,
        updated -> Nullable<Timestamp>,
    }
}

diesel::joinable!(author_post -> author (authorid));
diesel::joinable!(author_post -> post (postid));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    author_post,
    post,
);