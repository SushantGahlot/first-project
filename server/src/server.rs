use async_trait::async_trait;
use author_api::author_service_server::AuthorServiceServer;
use database::{author::AuthorDB, posts::PostDB};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use post_api::post_service_server::PostServiceServer;
use r2d2::Pool;
use tonic::transport::Server;

use crate::{author::AuthorAPI, post::PostAPI, GRPCService};

pub mod author_api {
    tonic::include_proto!("author_api");
}

pub mod post_api {
    tonic::include_proto!("post_api");
}

pub struct GRPCServer {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub struct MetadataMap<'a> {
    pub metadata: &'a tonic::metadata::MetadataMap,
}

#[async_trait]
impl GRPCService for GRPCServer {
    async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let author_service = AuthorAPI {
            author_db: AuthorDB {
                pool: self.pool.clone(),
            },
        };

        let post_service = PostAPI {
            author_db: AuthorDB {
                pool: self.pool.clone(),
            },
            post_db: PostDB {
                pool: self.pool.clone(),
            },
        };

        let address = "0.0.0.0:50052".parse().unwrap();
        Server::builder()
            .add_service(AuthorServiceServer::new(author_service))
            .add_service(PostServiceServer::new(post_service))
            .serve(address)
            .await?;
        Ok(())
    }
}
