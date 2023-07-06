use ethereum_types::{Address, Bloom, H256, U256, H64};
use rlp::{self, DecoderError, Rlp, RlpStream};
use parity_util_mem::MallocSizeOf;

#[derive(Debug, Clone, PartialEq, MallocSizeOf)]
pub struct Withdrawal {
    pub index: u64,
    pub validator: u64,
    pub address: Address,
    pub amount: u64,
}

// XBlock Shanghai
impl Withdrawal {
    /// encode raw transaction
    fn encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        self.encode_rlp(&mut stream);
        stream.drain()
    }

    pub fn rlp_append(
        &self,
        rlp: &mut RlpStream,
    ) {
        self.encode_rlp(rlp);
    }

    fn encode_rlp(
        &self,
        rlp: &mut RlpStream,
    ) {
        let list_size = 4;
        rlp.begin_list(list_size);

        rlp.append(&self.index);
        rlp.append(&self.validator);
        rlp.append(&self.address);
        rlp.append(&self.amount);
    }

    fn decode_rlp(d: &Rlp) -> Result<Withdrawal, DecoderError> {
        Ok(Withdrawal {
            index: d.val_at(0)?,
            validator: d.val_at(1)?,
            address: d.val_at(2)?,
            amount: d.val_at(3)?,
        })
    }


    pub fn decode_rlp_list(rlp: &Rlp) -> Result<Vec<Withdrawal>, DecoderError> {
        if !rlp.is_list() {
            // at least one byte needs to be present
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let mut output = Vec::with_capacity(rlp.item_count()?);
        for tx in rlp.iter() {
            output.push(Self::decode_rlp(&tx)?);
        }
        Ok(output)
    }
}