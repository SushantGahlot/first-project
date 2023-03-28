use database::{db::DBRepository, DatabaseRepository};

fn main() {
    let db_repository = DBRepository{};
    db_repository.run_migrations();

    println!("{:?}", db_repository.get_author_ids_by_email());
}