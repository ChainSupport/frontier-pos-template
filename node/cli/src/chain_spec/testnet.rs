// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::clone_on_copy)]

use common_runtime::AccountId;
use ecdsa_keyring::Keyring;
use hex_literal::hex;
use kitchensink_testnet_runtime::{
    constants::currency::*, wasm_binary_unwrap, Block, MaxNominations, SessionKeys, StakerStatus,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_sdk::*;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, ecdsa, sr25519, Pair, Public, H160, U256};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use sp_std::collections::btree_map::BTreeMap;
use std::str::FromStr;

pub use kitchensink_testnet_runtime::RuntimeGenesisConfig;
pub use node_primitives::{Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 100 * DOLLARS;
const STASH: Balance = ENDOWMENT;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

fn session_keys(
    // ed25519
    grandpa: GrandpaId,
    // sr25519
    babe: BabeId,
    // sr25519
    im_online: ImOnlineId,
    // sr25519
    authority_discovery: AuthorityDiscoveryId,
    // sr25519
    mixnet: MixnetId,
    // ecdsa
    beefy: BeefyId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
        mixnet,
        beefy,
    }
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_alice() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Alice";
    (
        Keyring::Alith.into(),
        Keyring::Alith.into(),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

fn configure_accounts(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    stash: Balance,
) -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    Vec<AccountId>,
    usize,
    Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            Keyring::Alith.into(),
            Keyring::Baltathar.into(),
            Keyring::CharLeth.into(),
            Keyring::Dorothy.into(),
            Keyring::Ethan.into(),
            Keyring::Faith.into(),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(x) {
                endowed_accounts.push(x.clone())
            }
        });

    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.0.clone(), stash, StakerStatus::Validator))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();

    (
        initial_authorities,
        endowed_accounts,
        num_endowed_accounts,
        stakers,
    )
}

/// Helper function to create RuntimeGenesisConfig json patch for testing.
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    extra_endowed_accounts_balance: Vec<(AccountId, u128)>,
    evm_chain_id: u32,
) -> serde_json::Value {
    let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) = configure_accounts(
        initial_authorities,
        initial_nominators,
        endowed_accounts,
        STASH,
    );

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).chain(extra_endowed_accounts_balance).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "staking": {
            "validatorCount": initial_authorities.len() as u32,
            "minimumValidatorCount": initial_authorities.len() as u32,
            "invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
            "slashRewardFraction": Perbill::from_percent(10),
            "stakers": stakers.clone(),
        },
        "sudo": { "key": Some(root_key.clone()) },
        "babe": {
            "epochConfig": Some(kitchensink_testnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        "nominationPools": {
            "minCreateBond": 10 * DOLLARS,
            "minJoinBond": 1 * DOLLARS,
        },
        "evmChainId": { "chainId": evm_chain_id },
    })
}

fn development_config_genesis_json() -> serde_json::Value {
    let extra_endowed_accounts_balance = vec![(Keyring::Baltathar.into(), 1000_000_000 * DOLLARS)];
    testnet_genesis(
        vec![authority_keys_from_alice()], // vec![AccountId::from(hex!("d43593c715fdd31c61141abd04a99fd6822c8558"))],
        vec![],
        Keyring::Alith.into(),
        Some(vec![Keyring::Alith.into()]),
        extra_endowed_accounts_balance,
        42u32,
    )
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("Development")
        .with_id("dev")
        // .with_id()
        .with_chain_type(ChainType::Development)
        // .with_properties(serde_json::from_str("{\"tokenDecimals\": 18, \"tokenSymbol\": \"UNIT\"}")
        .with_properties(
            serde_json::from_str(
                "{\"isEthereum\": true, \"tokenDecimals\": 18, \"tokenSymbol\": \"UNIT\"}",
            )
            .expect("Provided valid json map"),
        )
        .with_genesis_config_patch(development_config_genesis_json())
        .build()
}
