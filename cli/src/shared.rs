use database::{
    prelude::Decimal,
    repositories::{
        collection::{self as CollectionRespository, CreateCollectionParams},
        nft::{self as NftRepository, AttributeParams, CreateNftParams},
    },
    DatabaseConnection,
};
use service::{get_collection_metadata, get_nft_metadata, CosmosClient};

pub async fn create_collection_if_not_exist(
    db: &DatabaseConnection,
    client: &CosmosClient,
    address: String,
    royalty: Option<f32>,
) -> anyhow::Result<()> {
    let collection = CollectionRespository::find_by_address(db, &address).await?;

    if collection.is_some() {
        return Ok(());
    }

    let metadata = get_collection_metadata(&address).await?;
    let supply = client.get_cw721_contract_supply(&address).await?;
    let info = client.get_cw721_contract_info(&address).await?;

    CollectionRespository::create(
        db,
        CreateCollectionParams {
            address,
            symbol: info.symbol,
            name: info.name,
            banner: metadata.banner,
            description: metadata.description,
            image: metadata.pfp,
            socials: metadata.socials,
            supply: supply.count as i32,
            royalty: royalty
                .map(Decimal::from_f32_retain)
                .map(Option::unwrap_or_default),
        },
    )
    .await?;

    Ok(())
}

pub async fn create_nft_or_update_owner(
    db: &DatabaseConnection,
    client: &CosmosClient,
    token_address: String,
    token_id: String,
    owner: Option<String>,
) -> anyhow::Result<i32> {
    let nft = NftRepository::find_by_address_and_token_id(db, &token_address, &token_id).await?;

    if let Some(nft) = nft {
        NftRepository::update_owner(db, &token_address, &token_id, owner).await?;

        return Ok(nft.id);
    }

    let info = client.get_nft_info(&token_address, &token_id).await?;

    let metadata = get_nft_metadata(&info.token_uri).await?;

    create_collection_if_not_exist(
        db,
        client,
        token_address.to_owned(),
        info.extension
            .map(|ex| ex.royalty_percentage.unwrap_or_default()),
    )
    .await?;

    let nft_id = NftRepository::create(
        db,
        CreateNftParams {
            token_address,
            token_id,
            token_uri: info.token_uri,
            description: metadata.description,
            image: metadata.image,
            name: metadata.name,
            owner_address: None,
            traits: metadata.attributes.map(|attributes| {
                attributes
                    .into_iter()
                    .map(|attribute| AttributeParams {
                        display_type: attribute.display_type.map(|v| v.to_string()),
                        trait_type: attribute.trait_type,
                        r#type: attribute.r#type,
                        value: attribute.value.map(|v| v.to_string()),
                    })
                    .collect()
            }),
        },
    )
    .await?;

    Ok(nft_id)
}
