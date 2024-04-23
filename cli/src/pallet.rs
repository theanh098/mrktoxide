use crate::{
    find_attribute,
    shared::{self, CreateActivityTransactionAndPointOnSaleParams},
    to_utf8, Event,
};
use chrono::Utc;
use database::{
    prelude::{DateTimeUtc, Decimal},
    repositories::{self, nft::CreatePalletListingParams, transaction::CreateTransactionParams},
    sea_orm_active_enums::Marketplace,
    DatabaseConnection, TransactionTrait,
};
use service::{CosmosClient, PalletListing};
use tendermint_rpc::endpoint::tx;

async fn handle_create_auction(
    db: &DatabaseConnection,
    client: &CosmosClient,
    event: &Event,
    tx_hash: String,
) -> anyhow::Result<()> {
    let token_address = find_attribute(event, "collection_address")?;
    let token_id = find_attribute(event, "token_id")?;

    let pallet_listing = client.get_pallet_listing(&token_address, &token_id).await?;

    let PalletListing {
        auction,
        token_id,
        owner,
        ..
    } = pallet_listing;

    let nft_id = shared::create_nft_or_update_owner_or_just_find(
        db,
        client,
        token_address.to_owned(),
        token_id,
        None,
    )
    .await?;

    let Some(auction) = auction else {
        return Ok(());
    };

    let price = auction.prices.get(0).ok_or(anyhow::anyhow!(
        "unexpected error can not parse pallet listing price"
    ))?;

    let amount = Decimal::from_str(&price.amount)?;
    let created_date = DateTime::from_timestamp(auction.created_at as i64, 0).ok_or(
        anyhow::anyhow!("unexpected error can not parse pallet listing created_date"),
    )?;

    repositories::nft::create_pallet_listing(
        &db,
        CreatePalletListingParams {
            amount,
            created_date: created_date.into(),
            denom: "usei".to_string(),
            nft_id,
            tx_hash,
            collection_address: token_address,
            expiration_time: Some(auction.expiration_time as i32),
            seller: owner,
        },
    )
    .await?;

    Ok(())
}
async fn handle_buy_now(
    db: &DatabaseConnection,
    client: &CosmosClient,
    event: &Event,
    tx_hash: String,
) -> anyhow::Result<()> {
    let token_address = find_attribute(event, "collection_address")?;
    let token_id = find_attribute(event, "token_id")?;

    let tx = client.get_tx(&tx_hash).await?;
    let tx_head = client.get_tx_header(&tx_hash).await?;

    let buyer = find_buyer_address_from_tx(&tx).ok_or(anyhow::anyhow!(
        "unexpected error can not get buyer from tx {} in buy now event",
        tx_hash
    ))?;

    let nft_id = shared::create_nft_or_update_owner_or_just_find(
        db,
        client,
        token_address.to_owned(),
        token_id,
        None,
    )
    .await?;

    let db = db.begin().await?;

    repositories::nft::delete_listing_if_exist(&db, nft_id).await?;

    let db = shared::create_activity_transaction_and_point_on_sale(
        &db,
        CreateActivityTransactionAndPointOnSaleParams {
            buyer,
            collection_address: token_address,
            date: Utc::now().into(),
            denom: "usei".to_string(),
            marketplace: Marketplace::Pallet,
            metadata: serde_json::json!({}),
            nft_id,
            price: "a".to_string(),
            seller: "sf".to_string(),
            tx_hash,
        },
    )
    .await?;

    //     await this.nftRepository.deleteListingIfExist({
    //       tokenAddress,
    //       tokenId,
    //       marketplace: "pallet"
    //     });

    //     await this.nftRepository.createNftActivity({
    //       eventKind: "sale",
    //       denom: nft.Listing.denom,
    //       sellerAddress: nft.Listing.seller_address,
    //       buyerAddress,
    //       metadata: {},
    //       nft_id: nft.id,
    //       price: Number(nft.Listing.price),
    //       txHash,
    //       createdDate: DateTime.now(),
    //       marketplace: "pallet"
    //     });

    //     await this.transactionRepository.create({
    //       buyerAddress: buyerAddress || "unknown",
    //       sellerAddress: nft.Listing.seller_address,
    //       collection_address: nft.token_address,
    //       createdDate: DateTime.now(),
    //       txHash,
    //       volume: Number(nft.Listing.price),
    //       marketplace: "pallet"
    //     });

    Ok(())
}

// async fn handle_cancel_auction(event: &Event, tx_hash: String) {}

// import { Injectable } from "@nestjs/common";
// import { DateTime } from "luxon";

// import { NftRepository } from "@root/shared/repositories/nft.repository";
// import { TransactionRepository } from "@root/shared/repositories/transaction.repository";
// import { PalletContractQueryService } from "@root/shared/services/query-contract/pallet-contract-query.service";

// import { CommonService } from "./common.service";
// import { findAttributeByKey } from "./helper/shared";
// import type { ContractEvent } from "./helper/type";

// @Injectable()
// export class PalletEventService {
//   constructor(
//     private palletContractQueryService: PalletContractQueryService,
//     private commonService: CommonService,
//     private nftRepository: NftRepository,
//     private transactionRepository: TransactionRepository
//   ) {}

//   public async handleCreateAuctionEvent(event: ContractEvent, txHash: string) {
//     const tokenAddress = findAttributeByKey(event, "collection_address");
//     const tokenId = findAttributeByKey(event, "token_id");

//     if (!tokenAddress || !tokenId) {
//       throw new Error(`Missing event attribute in create_auction ${txHash}`);
//     }

//     const palletListing = await this.palletContractQueryService.getListing({
//       tokenAddress,
//       tokenId
//     });

//     if (!palletListing.auction) {
//       return;
//     }

//     const { amount, denom } = palletListing.auction.prices?.[0] || {};

//     if (!amount || !denom) {
//       throw new Error(`Missing amount or denom from Pallet listing: ${txHash}`);
//     }

//     const nft_id = await this.commonService.createNftIfNotExist(
//       tokenAddress,
//       tokenId
//     );

//     await this.nftRepository.createPalletNftListing({
//       nft_id,
//       txHash,
//       palletListingResponse: palletListing,
//       amount: Number(amount),
//       denom
//     });

//     await this.nftRepository.createNftActivity({
//       eventKind: "list",
//       denom,
//       sellerAddress: palletListing.owner,
//       metadata: {},
//       nft_id,
//       price: Number(amount),
//       txHash,
//       createdDate: DateTime.fromSeconds(palletListing.auction.created_at),
//       marketplace: "pallet"
//     });

//     console.log(
//       `Done handle create_auction at ${DateTime.now().toUTC()}: ${txHash}`
//     );
//   }

//   public async handleCancelAuction(event: ContractEvent, txHash: string) {
//     const tokenAddress = findAttributeByKey(event, "collection_address");
//     const tokenId = findAttributeByKey(event, "token_id");

//     if (!tokenAddress || !tokenId) {
//       throw new Error(`Missing event attribute in cancel_auction ${txHash}`);
//     }

//     const nft = await this.nftRepository.findByAddressAndTokenId({
//       tokenAddress,
//       tokenId,
//       withListing: true
//     });

//     if (!nft || !nft.Listing) {
//       return;
//     }

//     await this.nftRepository.deleteListingIfExist({
//       tokenAddress,
//       tokenId,
//       marketplace: "pallet"
//     });

//     await this.nftRepository.createNftActivity({
//       eventKind: "delist",
//       denom: nft.Listing.denom,
//       sellerAddress: nft.Listing.seller_address,
//       metadata: {},
//       nft_id: nft.id,
//       price: Number(nft.Listing.price),
//       txHash,
//       createdDate: DateTime.now(),
//       marketplace: "pallet"
//     });

//     console.log(
//       `Done handle cancel_auction at ${DateTime.now().toUTC()}: ${txHash}`
//     );
//   }

//   public async handleBuyNow(event: ContractEvent, txHash: string) {
//     const tokenAddress = findAttributeByKey(event, "collection_address");
//     const tokenId = findAttributeByKey(event, "token_id");

//     if (!tokenAddress || !tokenId) {
//       throw new Error(`Missing event attribute in cancel_auction ${txHash}`);
//     }

//     const nft = await this.nftRepository.findByAddressAndTokenId({
//       tokenAddress,
//       tokenId,
//       withListing: true
//     });

//     if (!nft || !nft.Listing) {
//       return;
//     }

//     const tx = await this.palletContractQueryService.getTx(txHash);

//     const buyerAddress = tx?.events
//       .find(
//         e =>
//           e.type === "wasm" && !!e.attributes.find(a => a.key === "recipient")
//       )
//       ?.attributes.find(a => a.key === "recipient")?.value;

//     await this.nftRepository.deleteListingIfExist({
//       tokenAddress,
//       tokenId,
//       marketplace: "pallet"
//     });

//     await this.nftRepository.createNftActivity({
//       eventKind: "sale",
//       denom: nft.Listing.denom,
//       sellerAddress: nft.Listing.seller_address,
//       buyerAddress,
//       metadata: {},
//       nft_id: nft.id,
//       price: Number(nft.Listing.price),
//       txHash,
//       createdDate: DateTime.now(),
//       marketplace: "pallet"
//     });

//     await this.transactionRepository.create({
//       buyerAddress: buyerAddress || "unknown",
//       sellerAddress: nft.Listing.seller_address,
//       collection_address: nft.token_address,
//       createdDate: DateTime.now(),
//       txHash,
//       volume: Number(nft.Listing.price),
//       marketplace: "pallet"
//     });

//     console.log(`Done handle buy_now at ${DateTime.now().toUTC()}: ${txHash}`);
//   }
// }

fn find_buyer_address_from_tx(tx: &tx::Response) -> Option<String> {
    tx.tx_result
        .events
        .iter()
        .find_map(|e| {
            if e.kind != "wasm" {
                None
            } else {
                e.attributes
                    .iter()
                    .find(|attribute| to_utf8(attribute.key.to_owned()) == "recipent")
            }
        })
        .map(|attribute| attribute.value.to_owned())
}
