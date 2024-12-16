#![allow(unused_imports)]
#![allow(dead_code)]

use bip32::XPrv;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use derivation_path::DerivationPath;
use fp_account::AccountId20;
#[cfg(feature = "std")]
use sp_core::ecdsa::Signature;
use sp_core::{
    crypto::DEV_PHRASE,
    ecdsa::{Pair, Public},
    hex2array, ByteArray, Pair as PairT, H256,
};
use sp_core::{ecdsa, H160};
use sp_keyring::sr25519::ParseKeyringError;
use strum::IntoEnumIterator;

extern crate alloc;
use alloc::{format, str::FromStr, string::String, vec::Vec};
use bip39;

/// Set of test accounts.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, Ord, PartialOrd,
)]
pub enum Keyring {
    Alith,
    Baltathar,
    CharLeth,
    Dorothy,
    Ethan,
    Faith,
}

impl Keyring {
    pub fn from_public(who: &Public) -> Option<Keyring> {
        Self::iter().find(|&k| &Public::from(k) == who)
    }

    pub fn from_account_id(who: &AccountId20) -> Option<Keyring> {
        Self::iter().find(|&k| &k.to_account_id() == who)
    }

    pub fn to_raw_public(self) -> [u8; 33] {
        *Public::from(self).as_array_ref()
    }

    pub fn to_raw_public_vec(self) -> Vec<u8> {
        Public::from(self).to_raw_vec()
    }

    pub fn from_h256_public(who: H160) -> Option<Keyring> {
        Self::iter().find(|k| k.to_account_id() == who.into())
    }

    pub fn to_h256_public(self) -> H160 {
        self.to_account_id().into()
    }

    pub fn to_account_id(self) -> AccountId20 {
        Into::<Public>::into(self.to_raw_public()).into()
    }

    #[cfg(feature = "std")]
    pub fn sign(self, msg: &[u8]) -> Signature {
        Pair::from(self).sign(msg)
    }

    pub fn pair(self) -> Pair {
        let seed: Seed = Seed::new(
            &Mnemonic::from_phrase(&DEV_PHRASE, Language::English).unwrap(),
            "",
        );
        let path = DerivationPath::bip44(60, 0, 0, self.into())
            .unwrap()
            .to_string();
        let child_xprv = XPrv::derive_from_path(seed, &path.parse().unwrap()).unwrap();
        Pair::from_seed_slice(child_xprv.to_bytes().as_ref()).unwrap()
    }

    /// Returns an iterator over all test accounts.
    pub fn iter() -> impl Iterator<Item = Keyring> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn public(self) -> Public {
        Public::from(self)
    }

    pub fn well_known() -> impl Iterator<Item = Keyring> {
        Self::iter().take(6)
    }

    pub fn invulnerable() -> impl Iterator<Item = Keyring> {
        Self::iter().take(6)
    }
}

impl From<Keyring> for &'static str {
    fn from(k: Keyring) -> Self {
        match k {
            Keyring::Alith => "Alith",
            Keyring::Baltathar => "Baltathar",
            Keyring::CharLeth => "CharLeth",
            Keyring::Dorothy => "Dorothy",
            Keyring::Ethan => "Ethan",
            Keyring::Faith => "Faith",
        }
    }
}

impl From<Keyring> for u32 {
    fn from(value: Keyring) -> Self {
        match value {
            Keyring::Alith => 0,
            Keyring::Baltathar => 1,
            Keyring::CharLeth => 2,
            Keyring::Dorothy => 3,
            Keyring::Ethan => 4,
            Keyring::Faith => 5,
        }
    }
}

impl FromStr for Keyring {
    type Err = ParseKeyringError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "Alith" | "alith" => Ok(Keyring::Alith),
            "Baltathar" | "baltathar" => Ok(Keyring::Baltathar),
            "CharLeth" | "charLeth" => Ok(Keyring::CharLeth),
            "Dorothy" | "dorothy" => Ok(Keyring::Dorothy),
            "Ethan" | "ethan" => Ok(Keyring::Ethan),
            "Faith" | "faith" => Ok(Keyring::Faith),
            _ => Err(ParseKeyringError),
        }
    }
}

impl From<Keyring> for sp_runtime::MultiSigner {
    fn from(x: Keyring) -> Self {
        sp_runtime::MultiSigner::Ecdsa(x.into())
    }
}

impl From<Keyring> for Public {
    fn from(k: Keyring) -> Self {
        Into::<Pair>::into(k).public()
    }
}

impl From<Keyring> for AccountId20 {
    fn from(k: Keyring) -> Self {
        k.to_account_id()
    }
}

impl From<Keyring> for Pair {
    fn from(k: Keyring) -> Self {
        k.pair()
    }
}

impl From<Keyring> for [u8; 20] {
    fn from(value: Keyring) -> Self {
        match value {
            Keyring::Alith => hex2array!("f24ff3a9cf04c71dbc94d0b566f7a27b94566cac"),
            Keyring::Baltathar => hex2array!("3cd0a705a2dc65e5b1e1205896baa2be8a07c6e0"),
            Keyring::CharLeth => hex2array!("798d4ba9baf0064ec19eb4f0a1a45785ae9d6dfc"),
            Keyring::Dorothy => hex2array!("773539d4ac0e786233d90a233654ccee26a613d9"),
            Keyring::Ethan => hex2array!("ff64d3f6efe2317ee2807d223a0bdc4c0c49dfdb"),
            Keyring::Faith => hex2array!("c0f0f4ab324c46e55d02d0033343b4be8a55532d"),
        }
    }
}

impl From<Keyring> for H160 {
    fn from(value: Keyring) -> Self {
        value.into()
    }
}

#[cfg(test)]
pub mod test {
    use std::path::Iter;
    use std::str::FromStr;
    use std::str::ParseBoolError;

    use crate::Keyring;

    use super::AccountId20;
    use super::Pair;
    use super::PairT;
    use bip32::secp256k1::sha2::digest::typenum::array;
    use bip32::secp256k1::sha2::digest::Key;
    use bip32::PrivateKey;
    use bip32::XPrv;
    use bip39::{Language, Mnemonic, MnemonicType, Seed};
    use derivation_path::DerivationPath;
    use hex::{self, FromHex, ToHex};
    use sp_core::crypto::DEV_PHRASE;
    use sp_core::ecdsa::Public;
    use sp_core::hex2array;
    use sp_keyring::sr25519::ParseKeyringError;

    #[test]
    pub fn test() {
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("Alith").unwrap()),
            0
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("alith").unwrap()),
            0
        );

        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("Baltathar").unwrap()),
            1
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("baltathar").unwrap()),
            1
        );

        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("CharLeth").unwrap()),
            2
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("charLeth").unwrap()),
            2
        );

        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("Dorothy").unwrap()),
            3
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("dorothy").unwrap()),
            3
        );

        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("Ethan").unwrap()),
            4
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("ethan").unwrap()),
            4
        );

        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("Faith").unwrap()),
            5
        );
        assert_eq!(
            <Keyring as Into<u32>>::into(Keyring::from_str("faith").unwrap()),
            5
        );
        assert!(Keyring::from_str("test").is_err(), "");

        assert_eq!(
            Into::<AccountId20>::into(Keyring::Alith.pair().public()),
            AccountId20(hex2array!("f24ff3a9cf04c71dbc94d0b566f7a27b94566cac"))
        );
        assert_eq!(
            Into::<AccountId20>::into(Keyring::Baltathar.pair().public()),
            AccountId20(hex2array!("3cd0a705a2dc65e5b1e1205896baa2be8a07c6e0"))
        );
        assert_eq!(
            Into::<AccountId20>::into(Keyring::CharLeth.pair().public()),
            AccountId20(hex2array!("798d4ba9baf0064ec19eb4f0a1a45785ae9d6dfc"))
        );
        assert_eq!(
            Into::<AccountId20>::into(Keyring::Dorothy.pair().public()),
            AccountId20(hex2array!("773539d4ac0e786233d90a233654ccee26a613d9"))
        );
        assert_eq!(
            Into::<AccountId20>::into(Keyring::Ethan.pair().public()),
            AccountId20(hex2array!("ff64d3f6efe2317ee2807d223a0bdc4c0c49dfdb"))
        );
        assert_eq!(
            Into::<AccountId20>::into(Keyring::Faith.pair().public()),
            AccountId20(hex2array!("c0f0f4ab324c46e55d02d0033343b4be8a55532d"))
        );

        Keyring::iter().for_each(|k| {
            let p = k.pair().public();
            assert_eq!(Keyring::from_public(&p), Some(k));
        });

        assert_eq!(
            Keyring::Alith.to_account_id(),
            AccountId20(hex2array!("f24ff3a9cf04c71dbc94d0b566f7a27b94566cac"))
        );

        assert_eq!(
            Keyring::iter().map(|k| k).collect::<Vec<Keyring>>(),
            vec![
                Keyring::Alith,
                Keyring::Baltathar,
                Keyring::CharLeth,
                Keyring::Dorothy,
                Keyring::Ethan,
                Keyring::Faith,
            ]
        );

        assert_eq!(
            Keyring::invulnerable()
                .into_iter()
                .collect::<Vec<Keyring>>(),
            vec![
                Keyring::Alith,
                Keyring::Baltathar,
                Keyring::CharLeth,
                Keyring::Dorothy,
                Keyring::Ethan,
                Keyring::Faith,
            ]
        );

        assert_eq!(
            Keyring::well_known().into_iter().collect::<Vec<Keyring>>(),
            vec![
                Keyring::Alith,
                Keyring::Baltathar,
                Keyring::CharLeth,
                Keyring::Dorothy,
                Keyring::Ethan,
                Keyring::Faith,
            ]
        );

        Keyring::iter().for_each(|k| {
            let h160 = k.to_h256_public();
            assert_eq!(Keyring::from_h256_public(h160), Some(k));
        });

        Keyring::iter().for_each(|k| {
            let msg = "test";
            let sig_msg = k.sign(msg.as_ref());
            Pair::verify(&sig_msg, msg.as_bytes(), &k.pair().public());
        });

        let raw_public = Keyring::Alith.to_raw_public();
        assert_eq!(Public::from_raw(raw_public), Keyring::Alith.public());

        let raw_public_vec = Keyring::Alith.to_raw_public_vec();
        let mut arr = [0u8; 33];
        arr[..].copy_from_slice(&raw_public_vec);
        assert_eq!(Public::from_raw(arr), Keyring::Alith.public());
    }
}
