use crate::models::AuthorPost;
use crate::models::Post;
use crate::models::PostById;
use crate::schema::author_post;
use crate::schema::author_post::authorid;
use crate::schema::author_post::postid;
use crate::schema::post;
use crate::PostDAO;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Array;
use diesel::sql_types::Integer;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::RunQueryDsl;

pub struct PostDB {
    pub pool: bb8::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
}

#[async_trait]
#[async_trait(?Send)]
impl PostDAO for PostDB {
    async fn get_posts_by_ids(
        &self,
        post_ids: &Vec<i32>,
    ) -> Result<Vec<PostById>, Box<dyn std::error::Error>> {
        if post_ids.len() == 0 {
            Err("post IDs can not be empty")?;
        }

        let mut conn = self.pool.get().await?;

        let query = sql_query(
            "
                SELECT
                    post.postid,
                    post.title,
                    post.body,
                    post.updated,
                    post.created,
                    array_agg(author.authorid) as authorids,
                    array_agg(author.firstname) as firstnames,
                    array_agg(author.lastname) as lastnames,
                    array_agg(author.username) as usernames,
                    array_agg(author.email) as emails
                FROM
                    author_post
                    INNER JOIN post ON author_post.postid = post.postid
                    INNER JOIN author ON author_post.authorid = author.authorid
                WHERE
                    post.postid = ANY($1)
                GROUP BY
                    post.postid,
                    post.body,
                    post.created,
                    post.title,
                    post.updated
            ",
        )
        .bind::<Array<Integer>, _>(post_ids);
        Ok(query.load(&mut conn).await?)
    }

    async fn upsert_post(
        &self,
        upsert_post: Post,
        author_ids: Vec<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;

        // Upsert Post
        let inserted_post_id = diesel::insert_into(post::table)
            .values(&upsert_post)
            .on_conflict(post::postid)
            .do_update()
            .set(&upsert_post)
            .returning(post::postid)
            .get_result(&mut conn)
            .await?;

        if inserted_post_id == 0 {
            Err("no rows inserted")?;
        }

        let mut author_post: Vec<AuthorPost> = Vec::new();
        for author in author_ids.iter() {
            author_post.push(AuthorPost {
                id: None,
                authorid: *author,
                postid: inserted_post_id,
            });
        }

        // Delete old associations
        // Test ME
        match diesel::delete(author_post::table.filter(postid.eq(inserted_post_id)))
            .execute(&mut conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                print!("failed deleting old author_post references, err: {:?}", err);
                Err("failed deleting old author_post references}")?;
            }
        }

        // Update author_post table
        diesel::insert_into(author_post::table)
            .values(author_post)
            .on_conflict((authorid, postid))
            .do_nothing()
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
