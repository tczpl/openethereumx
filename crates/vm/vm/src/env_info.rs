// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

//! Environment information for transaction execution.

use ethereum_types::{Address, H256, U256};
use ethjson;
use hash::keccak;
use std::{cmp, sync::Arc};

type BlockNumber = u64;

/// Simple vector of hashes, should be at most 256 items large, can be smaller if being used
/// for a block whose number is less than 257.
pub type LastHashes = Vec<H256>;

/// Information concerning the execution environment for a message-call/contract-creation.
#[derive(Debug, Clone)]
pub struct EnvInfo {
    /// The block number.
    pub number: BlockNumber,
    /// The block author.
    pub author: Address,
    /// The block timestamp.
    pub timestamp: u64,
    /// The block difficulty.
    pub difficulty: U256,
    /// The block gas limit.
    pub gas_limit: U256,
    /// The last 256 block hashes.
    pub last_hashes: Arc<LastHashes>,
    /// The gas used.
    pub gas_used: U256,
    /// Block base fee.
    pub base_fee: Option<U256>,
    /// The mix hash
    pub mix_hash: H256,

    // XBlock Dencun
    pub blob_base_fee: u128,
}

impl Default for EnvInfo {
    fn default() -> Self {
        EnvInfo {
            number: 0,
            author: Address::default(),
            timestamp: 0,
            difficulty: 0.into(),
            gas_limit: 0.into(),
            last_hashes: Arc::new(vec![]),
            gas_used: 0.into(),
            base_fee: None,
            mix_hash: H256::zero(),
            blob_base_fee: 0,
        }
    }
}

impl From<ethjson::vm::Env> for EnvInfo {
    fn from(e: ethjson::vm::Env) -> Self {
        let number = e.number.into();
        EnvInfo {
            number,
            author: e.author.into(),
            difficulty: e.difficulty.into(),
            gas_limit: e.gas_limit.into(),
            timestamp: e.timestamp.into(),
            last_hashes: Arc::new(
                (1..cmp::min(number + 1, 257))
                    .map(|i| keccak(format!("{}", number - i).as_bytes()))
                    .collect(),
            ),
            gas_used: U256::default(),
            base_fee: e.base_fee.map(|i| i.into()),
            mix_hash: e.mix_hash.into(),
            blob_base_fee: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{Address, U256};
    use ethjson;
    use std::str::FromStr;

    #[test]
    fn it_serializes_from_json() {
        let env_info = EnvInfo::from(ethjson::vm::Env {
            author: ethjson::hash::Address(
                Address::from_str("000000f00000000f000000000000f00000000f00").unwrap(),
            ),
            number: ethjson::uint::Uint(U256::from(1_112_339)),
            difficulty: ethjson::uint::Uint(U256::from(50_000)),
            gas_limit: ethjson::uint::Uint(U256::from(40_000)),
            timestamp: ethjson::uint::Uint(U256::from(1_100)),
            base_fee: None,
        });

        assert_eq!(env_info.number, 1112339);
        assert_eq!(
            env_info.author,
            Address::from_str("000000f00000000f000000000000f00000000f00").unwrap()
        );
        assert_eq!(env_info.gas_limit, 40000.into());
        assert_eq!(env_info.difficulty, 50000.into());
        assert_eq!(env_info.gas_used, 0.into());
    }

    #[test]
    fn it_can_be_created_as_default() {
        let default_env_info = EnvInfo::default();

        assert_eq!(default_env_info.difficulty, 0.into());
    }
}
