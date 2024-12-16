
#![allow(unused_imports)]
#![allow(dead_code)]

use sp_core::ecdsa;
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

        
        // // 貌似这个方法不支持BIP44
        // // f62b4c3327f61ac50c34f217e3dc61ac1a147b25951e326c702de9d8a281648a 私钥
        // // 7dd7b004a42508ad7a0b39e7e7483f4208d084d0 地址
        // // 直接根据种子生成是相同的 fixme 
        // let v = Vec::from_hex("f62b4c3327f61ac50c34f217e3dc61ac1a147b25951e326c702de9d8a281648a").unwrap();
        // println!("v len: {:?}", v.len());
        // let pair = Pair::from_seed_slice(v.as_ref()).unwrap();
		
        // let account_id: AccountId20 = pair.public().into();
        // println!("account id: {:?}", account_id);

        // bip39由助记词生成种子
        let mn = Mnemonic::from_phrase(&DEV_PHRASE, Language::English).unwrap();
        let seed: Seed = Seed::new(&mn, "");
        println!("seed: {:?}", hex::encode(seed.as_bytes()));
        // let m = MiniSecretKey::from_bytes(&seed.as_bytes()[..32]).expect("Length is always correct; qed");
        // println!("m: \n {:?}", hex::encode(m.to_bytes()));

        // bip44生成派生地址
        let path = DerivationPath::bip44(60,0,0,0).unwrap();
        println!("path: \n {:?}", path.to_string());

        // bip32由种子和派生地址生成公私钥
        let child_xprv = XPrv::derive_from_path(&seed, &path.to_string().parse().unwrap()).unwrap();
        // 这个就是私钥
        println!("Rchild_xprv: {:?}", hex::encode(child_xprv.private_key().to_bytes()));

        // 通过特定算法 根据私钥生成公钥

        // 根据公钥变成地址

        // fixme 根据私钥生成是准确的
        let p = Pair::from_seed_slice(child_xprv.to_bytes().as_ref()).unwrap();
        let account_id: AccountId20 = p.public().into();
        println!("account id: {:?}", account_id);

        





    }
}
