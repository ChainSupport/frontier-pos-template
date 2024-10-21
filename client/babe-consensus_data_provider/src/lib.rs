
use sp_application_crypto::{AppCrypto, ByteArray};
use sp_core::H256;
use sp_consensus_babe::BabeConfiguration;
use std::sync::Arc;
use sp_inherents::{CreateInherentDataProviders, InherentData, InherentDataProvider};
use sp_keystore::{Keystore, KeystorePtr};
use sp_consensus_babe::{AuthorityId, AuthorityPair, Randomness, Slot, VrfSignature, make_vrf_transcript, BabeApi};
use std::marker::PhantomData;
use sp_api::{ApiExt, ApiRef, Core, ProvideRuntimeApi};
use sc_service::{
    error::Error as ServiceError, ChainSpec, Configuration, PartialComponents, TFullBackend,
    TFullClient, TaskManager,
};
use sc_client_api::{
    backend::{AuxStore, Backend, StorageProvider},
    UsageProvider,
};
use fc_rpc::pending::ConsensusDataProvider;
use sp_runtime::{
    generic::{Digest, DigestItem},
    traits::{Block as BlockT, Header as HeaderT, One},
    TransactionOutcome,
};


/// Consensus data provider for Babe.
/// fixme: why aura need slot duration???
pub struct BabeConsensusDataProvider<B, C> {
    client: Arc<C>,
    keystore: Arc<dyn Keystore>,
    _phantom: PhantomData<B>,
}

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


impl<B: sp_runtime::traits::Block<Hash = sp_core::H256>, C: Send + Sync + ProvideRuntimeApi<B>> ConsensusDataProvider<B> for BabeConsensusDataProvider<B, C>
    where
        B: sp_runtime::traits::Block<Hash = sp_core::H256>,
        C: sp_api::ProvideRuntimeApi<B>,
        C::Api: BabeApi<B>,
{
    fn create_digest(
        &self,
        parent: &B::Header,
        _data: &InherentData,
    ) -> Result<Digest, sp_inherents::Error> {
        let best_block_hash: sp_core::H256 = parent.hash();
        let runtime_api = self.client.runtime_api();
        let babe_config: BabeConfiguration = runtime_api.configuration(best_block_hash).unwrap();

        todo!()
    }
}

