use std::sync::Arc;

use bdk::prelude::axum::extract::{Json, Path, State};
use ethers::providers::{Http, Provider};
use serde_json::json;

use crate::{
    AppState, Error,
    aide::NoApi,
    config,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::artworks::{
        MintSpaceArtworkRequest, MintSpaceArtworkResponse, SpaceArtwork, SpaceArtworkTrade,
    },
    models::{Post, PostArtwork, SpaceCommon, User, UserEvmAddress},
    transact_write,
    types::{EntityType, Partition, TeamGroupPermission},
    utils::{
        contracts::erc1155::Erc1155Contract,
        wallets::{kaia_local_wallet::KaiaLocalWallet, local_fee_payer::LocalFeePayer},
    },
};

const ART_TWIN_TOKEN_ID: u64 = 1;

pub async fn mint_space_artwork_handler(
    State(AppState { dynamo, s3, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(_req): Json<MintSpaceArtworkRequest>,
) -> Result<Json<MintSpaceArtworkResponse>, Error> {
    let space_pk = match space_pk {
        Partition::Space(_) => space_pk,
        _ => return Err(Error::InvalidSpacePartitionKey),
    };

    // Check permission
    let (_space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    // Check if SpaceArtwork already exists
    let existing_artwork =
        SpaceArtwork::get(&dynamo.client, &space_pk, Some(EntityType::SpaceArtwork)).await?;
    if existing_artwork.is_some() {
        return Err(Error::ArtworkAlreadyMinted);
    }

    let owner_evm_address =
        UserEvmAddress::get(&dynamo.client, &user.pk, Some(EntityType::UserEvmAddress))
            .await?
            .ok_or(Error::InvalidUserEvmAddress)?
            .evm_address;

    let post_pk = space_pk.clone().to_post_key()?;
    let post_artwork = PostArtwork::get(&dynamo.client, &post_pk, Some(EntityType::PostArtwork))
        .await?
        .ok_or(Error::ArtworkMetadataMissingOrInvalid)?;

    let post = Post::get(&dynamo.client, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error::PostNotFound)?;

    // Build metadata JSON
    let mut metadata_map = serde_json::Map::new();
    for item in post_artwork.metadata.into_iter() {
        metadata_map.insert(item.trait_type, json!(item.value));
    }

    metadata_map.insert(
        "name".to_string(),
        serde_json::Value::String(post.title.clone()),
    );
    metadata_map.insert(
        "description".to_string(),
        serde_json::Value::String(post.html_contents.clone()),
    );

    let image_url = post
        .urls
        .get(0)
        .ok_or(Error::ArtworkMetadataMissingOrInvalid)?;
    metadata_map.insert(
        "image".to_string(),
        serde_json::Value::String(image_url.clone()),
    );

    let metadata_string =
        serde_json::to_string_pretty(&metadata_map).map_err(|e| Error::JsonError(e.to_string()))?;

    // Upload metadata to S3
    let conf = config::get();
    let space_id = match &space_pk {
        Partition::Space(id) => id.clone(),
        _ => unreachable!(),
    };
    let key = format!("json/{}/{}/{}.json", conf.env, space_id, ART_TWIN_TOKEN_ID);

    let metadata_uri = s3
        .upload_object(
            &key,
            metadata_string.clone().into_bytes(),
            "application/json",
        )
        .await?;

    // Get contract address from space or post data
    // TODO: Retrieve contract_address from Space/Post configuration
    let contract_address = "0x0000000000000000000000000000000000000000".to_string();
    let tx_hash = "0x0000000000000000000000000000000000000000".to_string();
    //
    #[cfg(not(feature = "bypass"))]
    let tx_hash = {
        let conf = config::get();
        let provider = Provider::<Http>::try_from(&conf.kaia.endpoint as &str)
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let provider = Arc::new(provider);

        let owner = KaiaLocalWallet::new(&conf.kaia.owner_key, provider.clone()).await?;
        let feepayer = LocalFeePayer::new(
            &conf.kaia.feepayer_address,
            &conf.kaia.feepayer_key,
            provider.clone(),
        )
        .await?;

        let mut contract = Erc1155Contract::new(&contract_address, provider.clone());
        contract.set_wallet(owner);
        contract.set_fee_payer(feepayer);

        // Mint NFT (token_id = 1, amount = 1)
        let tx_hash = contract
            .mint_batch(owner_evm_address.clone(), vec![ART_TWIN_TOKEN_ID], vec![1])
            .await?;
        tx_hash
    };

    let artwork = SpaceArtwork::new(
        space_pk.clone(),
        contract_address,
        metadata_uri.clone(),
        metadata_string,
        owner_evm_address.clone(),
    );

    // Create trade record
    let trade = SpaceArtworkTrade::new_mint(space_pk.clone(), owner_evm_address.clone(), tx_hash);

    transact_write!(
        dynamo.client,
        artwork.create_transact_write_item(),
        trade.create_transact_write_item()
    )?;
    Ok(Json(MintSpaceArtworkResponse::from((artwork, trade))))
}
