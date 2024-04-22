use crate::{find_attribute, shared::create_nft_or_update_owner, Attribute, Event, Transaction};
use anyhow::Ok;
use chrono::Utc;
use database::{
    repositories::tracing::{self as TracingRepository, CreateStreamTxParams},
    sea_orm_active_enums::StreamContext,
    DatabaseConnection,
};
use service::CosmosClient;

static MINT_ACTION: &'static str = "mint";
static TRANSFER_ACTION: &'static str = "transfer_nft";
static SEND_ACTION: &'static str = "send_nft";

fn is_cw721_action_attribute(attribue: &Attribute) -> bool {
    let Attribute { key, value } = attribue;

    if key != "action" {
        false
    } else {
        value == MINT_ACTION || value == TRANSFER_ACTION || value == SEND_ACTION
    }
}

fn is_cw721_event(event: &Event) -> bool {
    event.r#type == "wasm"
        && event
            .attributes
            .iter()
            .find(|attribute| is_cw721_action_attribute(attribute))
            .is_some()
}

async fn hanlde_transfer(
    db: &DatabaseConnection,
    client: &CosmosClient,
    event: &Event,
) -> anyhow::Result<()> {
    let token_address = find_attribute(event, "_contract_address")?;
    let token_id = find_attribute(event, "token_id")?;
    let recipient = find_attribute(event, "recipient")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(recipient)).await?;
    println!("done handle transfer");
    Ok(())
}

async fn hanlde_send(
    db: &DatabaseConnection,
    client: &CosmosClient,
    event: &Event,
) -> anyhow::Result<()> {
    let token_address = find_attribute(event, "_contract_address")?;
    let token_id = find_attribute(event, "token_id")?;
    let recipient = find_attribute(event, "recipient")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(recipient)).await?;
    println!("done handle send");
    Ok(())
}

async fn hanlde_mint(
    db: &DatabaseConnection,
    client: &CosmosClient,
    event: &Event,
) -> anyhow::Result<()> {
    let token_address = find_attribute(event, "_contract_address")?;
    let token_id = find_attribute(event, "token_id")?;
    let owner = find_attribute(event, "owner")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(owner)).await?;
    println!("done handle mint");

    Ok(())
}

pub async fn tx_handler(db: &DatabaseConnection, client: &CosmosClient, tx: Transaction) {
    let Transaction { tx_hash, events } = tx;

    let events = events
        .into_iter()
        .filter(is_cw721_event)
        .collect::<Vec<Event>>();

    for event in events {
        let action = event
            .attributes
            .iter()
            .find(|Attribute { key, .. }| key == "action")
            .map(|attribute| attribute.value.to_owned())
            .unwrap_or_default();

        let result = if action == MINT_ACTION {
            hanlde_mint(db, client, &event).await
        } else if action == TRANSFER_ACTION {
            hanlde_transfer(db, client, &event).await
        } else if action == SEND_ACTION {
            hanlde_send(db, client, &event).await
        } else {
            println!("unexpected action {} event {:#?}", action, event);
            Ok(())
        };

        if let Err(error) = result {
            TracingRepository::create_stream_tx(
                db,
                CreateStreamTxParams {
                    action,
                    context: StreamContext::Cwr721,
                    date: Utc::now().into(),
                    event: serde_json::json!(event),
                    is_failure: true,
                    tx_hash: tx_hash.to_owned(),
                    message: Some(error.to_string()),
                },
            )
            .await
            .unwrap_or_else(|e| eprintln!("unexpected error when create tracing tx {}", e));
        } else {
            TracingRepository::create_stream_tx(
                db,
                CreateStreamTxParams {
                    action,
                    context: StreamContext::Cwr721,
                    date: Utc::now().into(),
                    event: serde_json::json!(event),
                    is_failure: false,
                    tx_hash: tx_hash.to_owned(),
                    message: None,
                },
            )
            .await
            .unwrap_or_else(|e| eprintln!("unexpected error when create tracing tx {}", e));
        }
    }
}
