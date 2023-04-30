use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use database::{author::AuthorDB, models::PostById, posts::PostDB, PostDAO};
use prost_types::Timestamp;
use tonic::{Request, Response, Status};

use crate::server::{
    author_api::Author,
    post_api::{
        post_service_server::PostService, GetPostsByIdsRequest, GetPostsByIdsResponse, Post,
        UpsertPostRequest, UpsertPostResponse,
    },
};

pub struct PostAPI {
    pub post_db: PostDB,
    pub author_db: AuthorDB,
}

#[tonic::async_trait]
impl PostService for PostAPI {
    async fn get_posts_by_ids(
        &self,
        req: Request<GetPostsByIdsRequest>,
    ) -> Result<Response<GetPostsByIdsResponse>, Status> {
        match req.get_ref().post_ids.len() {
            post_count if post_count == 0 => Err(Status::new(
                tonic::Code::InvalidArgument,
                "post ids can not be empty",
            ))?,
            post_count if post_count > 50 => Err(Status::new(
                tonic::Code::InvalidArgument,
                "post ids can not be more than 50",
            ))?,
            _ => {}
        }

        if req.get_ref().search_term.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "search term is a mandatory field",
            ))?;
        }

        let posts_result = self.post_db.get_posts_by_ids(&req.get_ref().post_ids);
        let mut posts: Vec<PostById> = Vec::new();

        match posts_result {
            Err(err) => {
                print!("error getting posts: {:?}", err);
                return Err(Status::new(
                    tonic::Code::Internal,
                    "could not get posts by ids",
                ))?;
            }
            Ok(db_posts) => {
                if db_posts.len() == 0 {
                    Err(Status::new(tonic::Code::NotFound, "posts not found"))?;
                }
                posts = db_posts;
            }
        }

        let mut proto_posts: Vec<Post> = Vec::new();
        let mut post_count_with_term = 0.0;
        let mut tfs: Vec<i32> = Vec::new();

        for post in &posts {
            let tf = calculate_tf(&post.body, &req.get_ref().search_term);
            tfs.push(tf);

            if tf > 0 {
                post_count_with_term += 1.0;
            }

            let mut proto_post_authors: Vec<Author> = Vec::new();

            for (i, author_id) in post.authorids.iter().enumerate() {
                proto_post_authors.push(Author {
                    user_name: (*post.usernames[i]).to_string(),
                    first_name: (*post.firstnames[i]).to_string(),
                    last_name: (*post.lastnames[i]).to_string(),
                    email: (*post.emails[i]).to_string(),
                    author_id: *author_id,
                })
            }

            let created_proto = convert_naive_time_to_proto_timestamp(post.created)?;
            let updated_proto = convert_naive_time_to_proto_timestamp(post.updated)?;

            let proto_post = Post {
                authors: proto_post_authors,
                body: (*post.body).to_string(),
                created: Some(created_proto),
                post_id: post.postid,
                title: (*post.title).to_string(),
                updated: Some(updated_proto),
                tfidf: 0.0,
            };

            proto_posts.push(proto_post);
        }

        let mut idf: f64 = 0.0;

        if post_count_with_term > 0.0 {
            idf = f64::log10((posts.len() as f64) / (post_count_with_term as f64));
        }

        for (i, proto_post) in proto_posts.iter_mut().enumerate() {
            if tfs[i] == 0 || idf == 0.0 {
                continue;
            }

            proto_post.tfidf = (tfs[i] as f64 * idf) as f32;
        }
        Ok(Response::new(GetPostsByIdsResponse { posts: proto_posts }))
    }

    async fn upsert_post(
        &self,
        req: Request<UpsertPostRequest>,
    ) -> Result<Response<UpsertPostResponse>, Status> {
        if req.get_ref().title.is_empty() {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                "post title can not be empty",
            ))?;
        }

        if req.get_ref().author_id.len() == 0 {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                "authors can not be empty",
            ))?;
        }

        if req.get_ref().post_id < 0 || req.get_ref().author_id.iter().any(|id| id.is_negative()) {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                "ids can not be negative",
            ))?;
        }

        let mut updated: Option<NaiveDateTime> = None;
        if req.get_ref().post_id > 0 {
            updated = Some(Utc::now().naive_utc());
        }

        let upsert_post_request = req.into_inner();

        let db_post = database::models::Post {
            title: upsert_post_request.title,
            body: if upsert_post_request.body.len() > 0 {
                Some(upsert_post_request.body)
            } else {
                None
            },
            postid: if upsert_post_request.post_id > 0 {
                Some(upsert_post_request.post_id)
            } else {
                None
            },
            created: None,
            updated: updated,
        };

        Ok(self
            .post_db
            .upsert_post(db_post, upsert_post_request.author_id)
            .map_err(|err| {
                println!("failed upserting post. err: {:?}", err);
                Status::new(tonic::Code::Internal, "failed upserting post")
            })
            .map(|_| Response::new(UpsertPostResponse {}))?)
    }
}

fn calculate_tf(post_body: &str, term: &str) -> i32 {
    if post_body.len() == 0 {
        return 0;
    }

    let mut term_count = 0;

    for word in post_body.split(" ") {
        if word == term {
            term_count += 1;
        }
    }

    term_count
}

fn convert_naive_time_to_proto_timestamp(ndt: NaiveDateTime) -> Result<Timestamp, Status> {
    let proto_timestamp_result = Timestamp::from_str(&ndt.to_string());
    match proto_timestamp_result {
        Err(err) => Err(Status::new(
            tonic::Code::Internal,
            format!("failed converting timestamp, err: {}", err),
        )),
        Ok(ts) => Ok(ts),
    }
}
