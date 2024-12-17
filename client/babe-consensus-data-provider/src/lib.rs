// This file is part of frontier-pos-template.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate is used for the EVM pending API.
//! The implementations of `make_primary_pre_digest`, `make_secondary_plain_pre_digest`, and `make_secondary_vrf_pre_digest` are fully based on 
//! https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/babe/src/mock.rs, 
//! so these methods do not require additional testing.
//!
//! Where is this crate used in this project?
//! https://github.com/ChainSupport/frontier-pos-template/blob/main/node/rpc/src/eth.rs#L150


#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(clippy::clone_on_copy)]
use fc_rpc::pending::ConsensusDataProvider;
use sc_client_api::{
    backend::{AuxStore, Backend, StorageProvider},
    UsageProvider,
};
use sc_service::{
    error::Error as ServiceError, ChainSpec, Configuration, PartialComponents, TFullBackend,
    TFullClient, TaskManager,
};
use schnorrkel::PublicKey;
use sp_api::{ApiExt, ApiRef, Core, ProvideRuntimeApi};
use sp_application_crypto::{AppCrypto, ByteArray};
use sp_consensus_babe::{
    digests::PreDigest, inherents::BabeInherentData, make_vrf_transcript, AllowedSlots,
    AuthorityId, AuthorityPair, BabeApi, BabeConfiguration, Epoch, Randomness, Slot, VrfSignature,
};
use sp_core::{Encode, H256};
use sp_inherents::{CreateInherentDataProviders, InherentData, InherentDataProvider};
use sp_keystore::{Keystore, KeystorePtr};
use sp_runtime::{
    generic::{Digest, DigestItem},
    traits::{Block as BlockT, Header as HeaderT, One},
    TransactionOutcome,
};
use std::{marker::PhantomData, sync::Arc};
pub struct BabeConsensusDataProvider<B, C> {
    client: Arc<C>,
    keystore: Arc<dyn Keystore>,
    _phantom: PhantomData<B>,
}
use sp_consensus_babe::SlotDuration;

impl<B, C> BabeConsensusDataProvider<B, C>
where
    B: sp_runtime::traits::Block<Hash = sp_core::H256>,
    C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
    C::Api: BabeApi<B>,
{
    pub fn new(client: Arc<C>, keystore: Arc<dyn Keystore>) -> Self {
        Self {
            client,
            keystore,
            _phantom: Default::default(),
        }
    }
}

pub fn make_primary_pre_digest(
    authority_index: sp_consensus_babe::AuthorityIndex,
    slot: sp_consensus_babe::Slot,
    vrf_signature: VrfSignature,
) -> Digest {
    let digest_data = sp_consensus_babe::digests::PreDigest::Primary(
        sp_consensus_babe::digests::PrimaryPreDigest {
            authority_index,
            slot,
            vrf_signature,
        },
    );
    let log = DigestItem::PreRuntime(sp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
    Digest { logs: vec![log] }
}

pub fn make_secondary_plain_pre_digest(
    authority_index: sp_consensus_babe::AuthorityIndex,
    slot: sp_consensus_babe::Slot,
) -> Digest {
    let digest_data = sp_consensus_babe::digests::PreDigest::SecondaryPlain(
        sp_consensus_babe::digests::SecondaryPlainPreDigest {
            authority_index,
            slot,
        },
    );
    let log = DigestItem::PreRuntime(sp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
    Digest { logs: vec![log] }
}

pub fn make_secondary_vrf_pre_digest(
    authority_index: sp_consensus_babe::AuthorityIndex,
    slot: sp_consensus_babe::Slot,
    vrf_signature: VrfSignature,
) -> Digest {
    let digest_data = sp_consensus_babe::digests::PreDigest::SecondaryVRF(
        sp_consensus_babe::digests::SecondaryVRFPreDigest {
            authority_index,
            slot,
            vrf_signature,
        },
    );
    let log = DigestItem::PreRuntime(sp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
    Digest { logs: vec![log] }
}

fn make_vrf_signature(
    randomness: &Randomness,
    slot: Slot,
    epoch: u64,
    key: sp_consensus_babe::AuthorityId,
    keystore: &KeystorePtr,
) -> Option<VrfSignature> {
    // VRF input
    let transcript = make_vrf_transcript(randomness, slot, epoch);
    let try_sign = Keystore::sr25519_vrf_sign(
        &**keystore,
        sp_consensus_babe::KEY_TYPE,
        key.as_ref(),
        &transcript.clone().into_sign_data(),
    );
    if let Ok(Some(signature)) = try_sign {
        let public = PublicKey::from_bytes(&key.to_raw_vec()).ok()?;
        if signature
            .pre_output
            .0
            .attach_input_hash(&public, transcript.0.clone())
            .is_err()
        {
            // VRF signature cannot be validated using key and transcript
            return None;
        }
        return Some(signature); 
    } else {
        // VRF key not found in keystore or VRF signing failed
        None
    }
}

impl<B: sp_runtime::traits::Block<Hash = sp_core::H256>, C: Send + Sync + ProvideRuntimeApi<B>>
    ConsensusDataProvider<B> for BabeConsensusDataProvider<B, C>
where
    B: sp_runtime::traits::Block<Hash = sp_core::H256>,
    C: sp_api::ProvideRuntimeApi<B>,
    C::Api: BabeApi<B>,
{
    fn create_digest(
        &self,
        parent: &B::Header,
        data: &InherentData,
    ) -> Result<Digest, sp_inherents::Error> {
        let best_block_hash: sp_core::H256 = parent.hash();
        let slot = data
            .babe_inherent_data()
            .expect("Timestamp is always present; qed");
        let runtime_api = self.client.runtime_api();
        let current_epoch: Epoch = runtime_api.current_epoch(best_block_hash).unwrap();
        let allowed_slots = current_epoch.config.allowed_slots.clone();
        let authorities: Vec<_> = current_epoch.authorities.clone();
        let randomness: Randomness = current_epoch.randomness.clone();
        let epoch_index: u64 = current_epoch.epoch_index.clone();
        let public_keys = self
            .keystore
            .sr25519_public_keys(sp_consensus_babe::KEY_TYPE);
        if public_keys.len() > 0 && slot.is_some() {
            // Retrieve the first value (note that the keystore of the node must ensure there is only one BABE key pair).
            let validator_public_key: sp_consensus_babe::AuthorityId = public_keys[0].into();
            let maybe_pos = authorities
                .iter()
                .position(|a| &validator_public_key == &a.0);
            // This will only be executed by validators.
            if let Some(authority_index) = maybe_pos {
                match allowed_slots {
                    AllowedSlots::PrimaryAndSecondaryPlainSlots => {
                        return Ok(make_secondary_plain_pre_digest(
                            authority_index as u32,
                            slot.unwrap(),
                        ));
                    }
                    AllowedSlots::PrimaryAndSecondaryVRFSlots => {
                        if let Some(vrf_signature) = make_vrf_signature(
                            &randomness,
                            slot.unwrap(),
                            epoch_index,
                            validator_public_key,
                            &self.keystore,
                        ) {
                            return Ok(make_secondary_vrf_pre_digest(
                                authority_index as u32,
                                slot.unwrap(),
                                vrf_signature,
                            ));
                        }
                    }
                    _ => {
                        if let Some(vrf_signature) = make_vrf_signature(
                            &randomness,
                            slot.unwrap(),
                            epoch_index,
                            validator_public_key,
                            &self.keystore,
                        ) {
                            return Ok(make_primary_pre_digest(
                                authority_index as u32,
                                slot.unwrap(),
                                vrf_signature,
                            ));
                        }
                    }
                }
            }
        }

        Ok(Default::default())
    }
}


#[cfg(test)]
pub mod test {
    #![allow(unused_variables)]
    use sp_keyring::sr25519::{self,Keyring};
    use sp_keystore::{Keystore, KeystorePtr, testing::MemoryKeystore};
    use sp_core::crypto::key_types::BABE;
    use sp_consensus_babe::{Randomness, Slot, SlotDuration, Epoch};
    use sp_core::sr25519::Public;
    use sp_timestamp::Timestamp;
    use std::sync::Arc;
    use sp_core::crypto::VrfPublic;
    use sp_consensus_babe::make_vrf_transcript;
    use sp_consensus_babe::RANDOMNESS_VRF_CONTEXT;

    use crate::make_vrf_signature;
    #[test]
    fn make_vrf_signature_should_works() {

        let seed = Keyring::Alice.to_seed();
        let keystore = Arc::new(MemoryKeystore::new());
        keystore.sr25519_generate_new(BABE, Some(&seed)).unwrap();
        let alice_public: Public  = Keyring::Alice.public().into();
        let randomness = [1; 32];
        let keystore: Arc<dyn Keystore> = keystore.clone() as Arc<dyn Keystore>;
        let epoch = 1;
        let slot = Slot::from_timestamp(Timestamp::new(3000), SlotDuration::from_millis(1000));
        let transcript = make_vrf_transcript(&randomness, slot, epoch);
        // The reason no new randomness is output here is that it can be obtained on-chain.
        let vrf_signature = make_vrf_signature(&randomness, slot, epoch, alice_public.into(), &keystore);
        assert!(alice_public.vrf_verify(&transcript.clone().into_sign_data(), &vrf_signature.clone().unwrap()), "");
        // Randomness generated on-chain
        let new_randomness:[u8; 32] = alice_public.make_bytes(RANDOMNESS_VRF_CONTEXT, &transcript, &vrf_signature.unwrap().pre_output).unwrap();
        println!("new_randomness: {:?}", new_randomness);

    }

}
