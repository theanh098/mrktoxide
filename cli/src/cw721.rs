use anyhow::Ok;
use base64::{prelude::BASE64_STANDARD, Engine};
use database::DatabaseConnection;
use service::CosmosClient;
use tendermint_rpc::{dialect::v0_37::Event, event::TxInfo};

use crate::{find_attribute, shared::create_nft_or_update_owner, to_wasm_payload, WasmPayload};

static MINT_ACTION: &'static str = "mint";
static TRANSFER_ACTION: &'static str = "transfer_nft";
static SEND_ACTION: &'static str = "send_nft";

fn is_cw721_action(action: &str) -> bool {
    action == &BASE64_STANDARD.encode(MINT_ACTION)
        || action == &BASE64_STANDARD.encode(TRANSFER_ACTION)
        || action == &BASE64_STANDARD.encode(SEND_ACTION)
}

fn is_cw721_event(event: &Event) -> bool {
    event.kind == "wasm"
        && event
            .attributes
            .iter()
            .find(|attribute| {
                let is_action_key = BASE64_STANDARD.encode("action") == attribute.key;

                if !is_action_key {
                    false
                } else {
                    is_cw721_action(&attribute.value)
                }
            })
            .is_some()
}

async fn hanlde_transfer(
    db: &DatabaseConnection,
    client: &CosmosClient,
    payload: &[WasmPayload],
) -> anyhow::Result<()> {
    let token_address = find_attribute(payload, "_contract_address")?;
    let token_id = find_attribute(payload, "token_id")?;
    let recipient = find_attribute(payload, "recipient")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(recipient)).await?;
    println!("done handle transfer");
    Ok(())
}

async fn hanlde_send(
    db: &DatabaseConnection,
    client: &CosmosClient,
    payload: &[WasmPayload],
) -> anyhow::Result<()> {
    let token_address = find_attribute(payload, "_contract_address")?;
    let token_id = find_attribute(payload, "token_id")?;
    let recipient = find_attribute(payload, "recipient")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(recipient)).await?;
    println!("done handle send");
    Ok(())
}

async fn hanlde_mint(
    db: &DatabaseConnection,
    client: &CosmosClient,
    payload: &[WasmPayload],
) -> anyhow::Result<()> {
    let token_address = find_attribute(payload, "_contract_address")?;
    let token_id = find_attribute(payload, "token_id")?;
    let owner = find_attribute(payload, "owner")?;

    create_nft_or_update_owner(db, client, token_address, token_id, Some(owner)).await?;
    println!("done handle mint");

    Ok(())
}

pub async fn tx_handler(
    db: &DatabaseConnection,
    client: &CosmosClient,
    tx: TxInfo,
) -> anyhow::Result<()> {
    let payloads: Vec<Vec<WasmPayload>> = tx
        .result
        .events
        .into_iter()
        .filter(is_cw721_event)
        .map(to_wasm_payload)
        .collect();

    for payload in payloads {
        let action = payload
            .iter()
            .find(|WasmPayload(key, _)| key == "action")
            .map(|WasmPayload(_, value)| value.to_string())
            .unwrap_or_default();

        if action == MINT_ACTION {
            hanlde_mint(db, client, &payload).await?;
        } else if action == TRANSFER_ACTION {
            hanlde_transfer(db, client, &payload).await?;
        } else if action == SEND_ACTION {
            hanlde_send(db, client, &payload).await?;
        } else {
            println!("unexpected action {} payload {:#?}", action, payload);
        }
    }

    Ok(())
}
