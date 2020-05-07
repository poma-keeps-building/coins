//! Bitcoin TxOut and Vout types.

use std::io::{Read, Write};

use riemann_core::{
    ser::{ByteFormat, SerError, SerResult},
    types::{
        tx::Output,
    },
};

use crate::types::script::ScriptPubkey;

/// An Output. This describes a new UTXO to be created. The value is encoded as an LE u64. The
/// script pubkey encodes the spending constraints.
///
/// `TxOut::null()` and `TxOut::default()` return the "null" TxOut, which has a value of
/// 0xffff_ffff_ffff_ffff, and an empty `script_pubkey`. This null output is used within legacy
/// sighash calculations.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TxOut {
    /// The value of the output in satoshis
    pub value: u64,
    /// The `ScriptPubkey` which locks the UTXO.
    pub script_pubkey: ScriptPubkey,
}

impl Output for TxOut {
    type Value = u64;
    type RecipientIdentifier = ScriptPubkey;
}

impl Default for TxOut {
    fn default() -> Self {
        Self::null()
    }
}

impl TxOut {
    /// Instantiate a new TxOut.
    pub fn new<T>(value: u64, script_pubkey: T) -> Self
    where
        T: Into<ScriptPubkey>,
    {
        TxOut {
            value,
            script_pubkey: script_pubkey.into(),
        }
    }

    /// Instantiate the null TxOut, which is used in Legacy Sighash.
    pub fn null() -> Self {
        TxOut {
            value: 0xffff_ffff_ffff_ffff,
            script_pubkey: ScriptPubkey::null(),
        }
    }
}

impl ByteFormat for TxOut {
    type Error = SerError;

    fn serialized_length(&self) -> usize {
        let mut len = 8; // value
        len += self.script_pubkey.serialized_length();
        len
    }

    fn read_from<R>(reader: &mut R, _limit: usize) -> SerResult<Self>
    where
        R: Read,
        Self: std::marker::Sized,
    {
        let value = Self::read_u64_le(reader)?;
        Ok(TxOut {
            value,
            script_pubkey: ScriptPubkey::read_from(reader, 0)?,
        })
    }

    fn write_to<W>(&self, writer: &mut W) -> SerResult<usize>
    where
        W: Write,
    {
        let mut len = Self::write_u64_le(writer, self.value)?;
        len += self.script_pubkey.write_to(writer)?;
        Ok(len)
    }
}

/// Vout is a type alias for `Vec<TxOut>`. A transaction's Vout is the Vector of
/// OUTputs, with a length prefix.
pub type Vout = Vec<TxOut>;

#[cfg(test)]
mod test {
    use super::*;
    use riemann_core::ser::ByteFormat;

    #[test]
    fn it_serializes_and_derializes_outputs() {
        let cases = [
            (TxOut::new(0, vec![]), "000000000000000000", 9),
            (TxOut::null(), "ffffffffffffffff00", 9),
        ];
        for case in cases.iter() {
            assert_eq!(case.0.serialized_length(), case.2);
            assert_eq!(case.0.serialize_hex().unwrap(), case.1);
            assert_eq!(TxOut::deserialize_hex(case.1).unwrap(), case.0);
        }
    }
}
