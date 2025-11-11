use std::sync::Arc;

use bdk::prelude::axum::extract::{Json, Path, State};
use ethers::providers::{Http, Provider};

use crate::{
    AppState, Error, Permissions,
    aide::NoApi,
    config,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::artworks::{
        SpaceArtwork, SpaceArtworkTrade, TransferSpaceArtworkRequest, TransferSpaceArtworkResponse,
    },
    models::{User, UserEvmAddress},
    transact_write,
    types::{EntityType, Partition, TeamGroupPermission},
    utils::{
        time::get_now_timestamp_millis,
        wallets::{kaia_local_wallet::KaiaLocalWallet, local_fee_payer::LocalFeePayer},
    },
};

#[cfg(not(feature = "bypass"))]
const ART_TWIN_TOKEN_ID: u64 = 1;

pub async fn transfer_space_artwork_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(_user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<TransferSpaceArtworkRequest>,
) -> Result<Json<TransferSpaceArtworkResponse>, Error> {
    let space_pk = match space_pk {
        Partition::Space(_) => space_pk,
        _ => return Err(Error::InvalidSpacePartitionKey),
    };

    // Check permission - user must have SpaceEdit permission
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Get existing artwork (metadata only, no modification)
    let space_artwork =
        SpaceArtwork::get(&dynamo.client, &space_pk, Some(EntityType::SpaceArtwork))
            .await?
            .ok_or(Error::ArtworkNotFound)?;

    let receiver_evm_address = UserEvmAddress::get(
        &dynamo.client,
        &req.to_user_pk,
        Some(EntityType::UserEvmAddress),
    )
    .await?
    .ok_or(Error::InvalidUserEvmAddress)?
    .evm_address;

    let _tx_hash = "0x0000000000000000000000000000000000000000".to_string();

    #[cfg(not(feature = "bypass"))]
    let _tx_hash = {
        // Prepare blockchain transaction
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

        // Execute safe transfer
        let mut contract = crate::utils::contracts::erc1155::Erc1155Contract::new(
            &space_artwork.contract_address,
            provider.clone(),
        );
        contract.set_wallet(owner);
        contract.set_fee_payer(feepayer);

        let tx_hash = contract
            .safe_transfer_from(
                space_artwork.owner_evm_address.clone(),
                receiver_evm_address.clone(),
                ART_TWIN_TOKEN_ID,
                1,
                vec![],
            )
            .await?;
        tx_hash
    };

    // Create trade record
    let trade = SpaceArtworkTrade::new_transfer(
        space_pk.clone(),
        space_artwork.owner_evm_address.clone(),
        receiver_evm_address.clone(),
        _tx_hash,
    );

    let updater = SpaceArtwork::updater(&space_pk, EntityType::SpaceArtwork)
        .with_owner_evm_address(receiver_evm_address.clone())
        .with_updated_at(get_now_timestamp_millis());

    transact_write!(
        dynamo.client,
        trade.create_transact_write_item(),
        updater.transact_write_item()
    )?;
    Ok(Json(TransferSpaceArtworkResponse::from((
        space_artwork,
        trade,
    ))))
}
