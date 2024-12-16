
#![allow(unused_imports)]
#![allow(dead_code)]

use sp_core::{ecdsa, H160};
#[cfg(feature = "std")]
use sp_core::ecdsa::Signature;
use sp_core::{
    ecdsa::{Pair, Public},
    crypto::DEV_PHRASE,
    hex2array, ByteArray, Pair as PairT, H256,
};
use sp_keyring::sr25519::ParseKeyringError;
use strum::IntoEnumIterator;
use fp_account::AccountId20;
use bip39::{Mnemonic, MnemonicType, Language, Seed};
use derivation_path::DerivationPath;
use bip32::XPrv;

extern crate alloc;
use bip39;
use alloc::{format, str::FromStr, string::String, vec::Vec};


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

    pub fn to_account_id(self) -> AccountId20 {
		Into::<Public>::into(self.to_raw_public()).into()
	}

    #[cfg(feature = "std")]
	pub fn sign(self, msg: &[u8]) -> Signature {
		Pair::from(self).sign(msg)
	}

    pub fn pair(self) -> Pair {
        let seed: Seed = Seed::new(&Mnemonic::from_phrase(&DEV_PHRASE, Language::English).unwrap(), "");
        let path = DerivationPath::bip44(60, 0, 0, self.into()).unwrap().to_string();
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

    use super::Pair;
    use super::PairT;
    use super::AccountId20;
    use sp_core::crypto::DEV_PHRASE;
    use bip32::PrivateKey;
    use hex::{self, FromHex, ToHex};
    use bip32::XPrv;
    use bip39::{Mnemonic, MnemonicType, Language, Seed};
    use derivation_path::DerivationPath;

    #[test]
    pub fn test() {

        // bip39由助记词生成种子
        let mn = Mnemonic::from_phrase(&DEV_PHRASE, Language::English).unwrap();
        let seed: Seed = Seed::new(&mn, "");
        println!("seed: {:?}", hex::encode(seed.as_bytes()));

        // bip44生成派生地址
        let path = DerivationPath::bip44(60,0,0,0).unwrap();
        println!("path: \n {:?}", path.to_string());

        // bip32由种子和派生地址生成公私钥
        let child_xprv = XPrv::derive_from_path(&seed, &path.to_string().parse().unwrap()).unwrap();
        // 这个就是私钥
        println!("Rchild_xprv: {:?}", hex::encode(child_xprv.private_key().to_bytes()));

        // fixme 根据私钥生成是准确的
        let p = Pair::from_seed_slice(child_xprv.to_bytes().as_ref()).unwrap();
        let account_id: AccountId20 = p.public().into();
        println!("account id: {:?}", account_id);

        // 通过特定算法 根据私钥生成公钥

        // 根据公钥变成地址

        

        





    }
}
