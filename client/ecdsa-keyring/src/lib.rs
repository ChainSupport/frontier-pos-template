
use sp_core::ecdsa;
#[cfg(feature = "std")]
use sp_core::ecdsa::Signature;
use sp_core::{
    ecdsa::{Pair, Public},
    hex2array, ByteArray, Pair as PairT, H256,
};
use strum::IntoEnumIterator;
use fp_account::AccountId20;

extern crate alloc;
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

// impl Keyring {
//     pub fn from_public(who: &Public) -> Option<Keyring> {
// 		Self::iter().find(|&k| &Public::from(k) == who)
// 	} 

//     pub fn from_account_id(who: &AccountId20) -> Option<Keyring> {
// 		Self::iter().find(|&k| &k.to_account_id() == who)
// 	}

//     pub fn to_raw_public(self) -> [u8; 33] {
// 		*Public::from(self).as_array_ref()
// 	}

//     pub fn to_raw_public_vec(self) -> Vec<u8> {
// 		Public::from(self).to_raw_vec()
// 	}

//     // todo 公钥和地址之间的关系是什么？？？
//     pub fn to_account_id(self) -> AccountId20 {
// 		self.to_raw_public().into()
// 	}

//     #[cfg(feature = "std")]
// 	pub fn sign(self, msg: &[u8]) -> Signature {
// 		Pair::from(self).sign(msg)
// 	}
    
//     pub fn pair(self) -> Pair {
// 		Pair::from_string(&format!("/m/44'/60'/0'/0/{}", <&'static str>::from(self)), None)
// 			.expect("static values are known good; qed")
// 	}

    
// }

// impl From<Keyring> for &'static str {
// 	fn from(k: Keyring) -> Self {
// 		match k {
// 			Keyring::Alith => "0",
//             Keyring::Baltathar => "1",
//             Keyring::CharLeth => "2",
//             Keyring::Dorothy => "3",
//             Keyring::Ethan => "4",
//             Keyring::Faith => "5",
// 		}
// 	}
// }

// impl From<Keyring> for sp_runtime::MultiSigner {
// 	fn from(x: Keyring) -> Self {
// 		sp_runtime::MultiSigner::Ecdsa(x.into())
// 	}
// }

// impl From<Keyring> for Public {
// 	fn from(k: Keyring) -> Self {
// 		Public::from_raw(k.into())
// 	}
// }

// impl From<Keyring> for AccountId20 {
// 	fn from(k: Keyring) -> Self {
// 		k.to_account_id()
// 	}
// }

// impl From<Keyring> for Pair {
// 	fn from(k: Keyring) -> Self {
// 		k.pair()
// 	}
// }


#[cfg(test)]
pub mod test {
    use super::Pair;
    use super::PairT;
    use super::AccountId20;

    #[test]
    pub fn test() {
        // 貌似这个方法不支持BIP44
        let pair = Pair::from_string_with_seed("invest cable warrior mutual opera pioneer energy success suggest coyote unaware trial/m/44'/60'/0'/0/0", None)
			.unwrap();
        let account_id: AccountId20 = pair.0.public().into();
        println!("account id: {:?}", account_id);

    }
}
