use database::{author::AuthorDB, AuthorDAO};
use tonic::{Request, Response, Status};

use crate::server::author_api::{
    author_service_server::AuthorService, Author, GetAuthorIdsByEmailResponse,
    GetAuthorIdsByEmailsRequest, GetAuthorsByIdsRequest, GetAuthorsByIdsResponse,
};

pub struct AuthorAPI {
    pub author_db: AuthorDB,
}

#[tonic::async_trait]
impl AuthorService for AuthorAPI {
    async fn get_author_ids_by_emails(
        &self,
        req: Request<GetAuthorIdsByEmailsRequest>,
    ) -> Result<Response<GetAuthorIdsByEmailResponse>, Status> {
        println!(
            "Got a request from {:?} to get author ids by emails",
            req.remote_addr()
        );

        if req.get_ref().email.len() == 0 {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "email IDs can not be empty",
            ))?;
        }

        match self.author_db.get_author_ids_by_email(&req.get_ref().email).await {
            Ok(author_ids) => {
                if author_ids.len() == 0 {
                    Err(Status::new(tonic::Code::NotFound, "authors not found"))?;
                }
                Ok(Response::new(GetAuthorIdsByEmailResponse { author_ids }))
            }
            Err(err) => {
                println!(
                    "Error getting author ids for email ids: {:?}, address: {:?}, error: {:?}",
                    req.get_ref().email,
                    req.remote_addr(),
                    err
                );
                Err(Status::new(tonic::Code::NotFound, "authors not found"))
            }
        }
    }

    async fn get_authors_by_ids(
        &self,
        req: Request<GetAuthorsByIdsRequest>,
    ) -> Result<Response<GetAuthorsByIdsResponse>, Status> {
        print!(
            "Got a request from {:?} to get authors by ids",
            req.remote_addr()
        );

        let author_ids = &req.get_ref().author_ids;

        if author_ids.len() == 0 {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "author ids can not be empty",
            ))?;
        }

        match self.author_db.get_authors_by_ids(&author_ids).await {
            Ok(authors) => {
                if authors.len() == 0 {
                    return Err(Status::new(tonic::Code::NotFound, "authors not found"))?;
                }

                let mut authors_resp: Vec<Author> = Vec::new();

                for author in authors {
                    authors_resp.push(Author {
                        user_name: author.username,
                        first_name: author.firstname,
                        last_name: author.lastname,
                        email: author.email,
                        author_id: author.authorid,
                    })
                }

                Ok(Response::new(GetAuthorsByIdsResponse {
                    authors: authors_resp,
                }))
            }
            Err(err) => {
                println!(
                    "Error getting authors for ids: {:?}, address: {:?}, error: {:?}",
                    author_ids,
                    req.remote_addr(),
                    err
                );
                Err(Status::new(tonic::Code::NotFound, "authors not found"))
            }
        }
    }
}
