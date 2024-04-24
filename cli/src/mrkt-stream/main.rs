use database::{
    repositories::{self, collection::CollectionStatSelectOption},
    ConnectOptions, Database, Sort,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await.unwrap();

    let colllections = repositories::collection::find_collections_with_stats(
        &db,
        [
            CollectionStatSelectOption::Address,
            CollectionStatSelectOption::Name,
            CollectionStatSelectOption::Symbol,
            CollectionStatSelectOption::Volume,
            CollectionStatSelectOption::Socials,
            CollectionStatSelectOption::Image,
            CollectionStatSelectOption::VolumeOf24h,
            CollectionStatSelectOption::VolumeOf30d,
            CollectionStatSelectOption::Royalty,
            CollectionStatSelectOption::Supply,
        ],
        (Some(1), Some(10)),
        (CollectionStatSelectOption::Volume, Sort::Desc),
    )
    .await
    .unwrap();

    dbg!(colllections)
}
