use database::{dao::DB, DAO};
use server::{server::GRPCServer, GRPCService};

#[tokio::main]
async fn main() {
    let db = DB::new();

    let grpc_server = GRPCServer {
        pool: db.pool.clone(),
    };
    grpc_server.start().await.expect("Failed to start server");
}