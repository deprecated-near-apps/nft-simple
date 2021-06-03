use std::collections::HashMap;
use std::cmp::min;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U64, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, StorageUsage,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::token::*;
pub use crate::enumerable::*;

mod internal;
mod metadata;
mod mint;
mod nft_core;
mod token;
mod enumerable;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    pub tokens_by_id: LookupMap<TokenId, Token>,

    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    pub owner_id: AccountId,

    /// The storage size in bytes for one account.
    pub extra_storage_in_bytes_per_token: StorageUsage,

    pub metadata: LazyOption<NFTMetadata>,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NftMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTMetadata) -> Self {
        let mut this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id: owner_id.into(),
            extra_storage_in_bytes_per_token: 0,
            metadata: LazyOption::new(
                StorageKey::NftMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        this.measure_min_token_storage_cost();

        this
    }

    fn measure_min_token_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = "a".repeat(64);
        let u = UnorderedSet::new(
            StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&tmp_account_id),
            }
            .try_to_vec()
            .unwrap(),
        );
        self.tokens_per_owner.insert(&tmp_account_id, &u);

        let tokens_per_owner_entry_in_bytes = env::storage_usage() - initial_storage_usage;
        let owner_id_extra_cost_in_bytes = (tmp_account_id.len() - self.owner_id.len()) as u64;

        self.extra_storage_in_bytes_per_token =
            tokens_per_owner_entry_in_bytes + owner_id_extra_cost_in_bytes;

        self.tokens_per_owner.remove(&tmp_account_id);
    }

}
