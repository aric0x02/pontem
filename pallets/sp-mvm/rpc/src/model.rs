use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use anyhow::{bail,Error};
use move_core_types::value::MoveValue;
use core::str::FromStr;
use move_core_types::account_address::AccountAddress;
use move_core_types::transaction_argument::TransactionArgument;
use serde::{Deserialize, Serialize};
    use move_vm::types::{Transaction,Call,Signer,TxV1};
// use sp_std::vec::Vec;
// use scale_info::prelude::string::String;
// use  bcs_alt as bcs;
// use move_symbol_pool::Symbol;
// use lang::bytecode::info::BytecodeInfo;
// /// Transaction model.
// #[derive(Debug)]
// pub enum Transaction {
//     /// Version 1.
//     V1(V1),
// }

// /// Transaction model.
// #[derive( Debug)]
// pub struct V1 {
//     /// Signers.
//     pub signers: Vec<Signer>,
//     /// Call declaration.
//     pub call: Call,
//     /// Script args.
//     pub args: Vec<MoveValue>,
//     /// Script type arguments.
//     pub type_args: Vec<Vec<u8>>,
// }

// /// Call declaration.
// #[derive(Serialize, Deserialize, Debug)]
// pub enum Call {
//     /// Script
//     Script {
//         /// Script bytecode.
//         code: Vec<u8>,
//     },
//     /// Function in module with script viability.
//     ScriptFunction {
//         /// Module address.
//         mod_address: String,
//         /// Module name.
//         mod_name: String,
//         /// Function name.
//         func_name: String,
//     },
// }

// impl Transaction {
    /// Create a new function transaction.
    pub fn new_func_tx(
        signers: Vec<Signer>,
        mod_address: AccountAddress,
        mod_name: Identifier,
        func_name: Identifier,
        args: Vec<MoveValue>,
        type_args: Vec<TypeTag>,
    ) -> Result<Vec<u8>, Error> {
        Ok(Transaction::V1(TxV1 {
            signers,
            call: Call::ScriptFunction {
                mod_address,
                func_name,
                mod_name,
            },
            args: Transaction::args_to_vec(args).unwrap(),
            type_args,
        }).to_vec().unwrap())
    }

    // fn make_args(args: Vec<MoveValue>) -> Result<Vec<MoveValue>, Error> {
    //     // args.into_iter()
    //     //     .map(ScriptArg::into)c
    //     //     .map(|val: MoveValue| bcs::to_bytes(&val))
    //     //     .collect::<Result<_, _>>()
    //     //     .map_err(Error::msg)
    //     Ok(args)
    // }

    // /// Returns last version.
    // pub fn inner_mut(&mut self) -> &mut V1 {
    //     match self {
    //         Transaction::V1(v) => v,
    //     }
    // }

    // /// Returns last version.
    // pub fn inner(self) -> V1 {
    //     match self {
    //         Transaction::V1(v) => v,
    //     }
    // }
// }

// /// Script argument type.
// #[derive(Debug, PartialEq, Eq)]
// pub enum ScriptArg {
//     /// u8
//     U8(u8),
//     /// u64
//     U64(u64),
//     /// u128
//     U128(u128),
//     /// bool
//     Bool(bool),
//     /// address
//     Address(AccountAddress),
//     /// vector<u8>
//     VectorU8(Vec<u8>),
//     /// vector<u64>
//     VectorU64(Vec<u64>),
//     /// vector<u128>
//     VectorU128(Vec<u128>),
//     /// vector<bool>
//     VectorBool(Vec<bool>),
//     /// vector<address>
//     VectorAddress(Vec<AccountAddress>),
// }

// impl From<ScriptArg> for MoveValue {
//     fn from(arg: ScriptArg) -> Self {
//         match arg {
//             ScriptArg::U8(val) => MoveValue::U8(val),
//             ScriptArg::U64(val) => MoveValue::U64(val),
//             ScriptArg::U128(val) => MoveValue::U128(val),
//             ScriptArg::Bool(val) => MoveValue::Bool(val),
//             ScriptArg::Address(val) => MoveValue::Address(val),
//             ScriptArg::VectorU8(val) => MoveValue::vector_u8(val),
//             ScriptArg::VectorU64(val) => {
//                 MoveValue::Vector(val.into_iter().map(MoveValue::U64).collect())
//             }
//             ScriptArg::VectorU128(val) => {
//                 MoveValue::Vector(val.into_iter().map(MoveValue::U128).collect())
//             }
//             ScriptArg::VectorBool(val) => {
//                 MoveValue::Vector(val.into_iter().map(MoveValue::Bool).collect())
//             }
//             ScriptArg::VectorAddress(val) => {
//                 MoveValue::Vector(val.into_iter().map(MoveValue::Address).collect())
//             }
//         }
//     }
// }

// impl TryInto<TransactionArgument> for ScriptArg {
//     type Error = Error;

//     fn try_into(self) -> Result<TransactionArgument, Self::Error> {
//         Ok(match self {
//             ScriptArg::U8(val) => TransactionArgument::U8(val),
//             ScriptArg::U64(val) => TransactionArgument::U64(val),
//             ScriptArg::U128(val) => TransactionArgument::U128(val),
//             ScriptArg::Bool(val) => TransactionArgument::Bool(val),
//             ScriptArg::Address(val) => TransactionArgument::Address(val),
//             ScriptArg::VectorU8(val) => TransactionArgument::U8Vector(val),
//             ScriptArg::VectorU64(_)
//             | ScriptArg::VectorU128(_)
//             | ScriptArg::VectorBool(_)
//             | ScriptArg::VectorAddress(_) => bail!("Unsupported transaction args."),
//         })
//     }
// }

/// Signer type.
// #[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub enum Signer {
//     /// Root signer.
//     Root,
//     /// Template to replace.
//     Placeholder,
//     /// Named address.
//     Name(String),
// }

// impl FromStr for Signer {
//     type Err = Error;

   pub  fn from_str(s: &str) -> Option<Signer>{
        Some(match s.to_lowercase().as_str() {
            "root" | "rt" | "dr" => Signer::Root,
            "_" => Signer::Placeholder,
            _ => Signer::Name(String::from(s)),
        })
    }
// }

#[derive(Debug, PartialEq)]
pub(crate) enum Signers {
    Explicit(Vec<AccountAddress>),
    Implicit(Vec<Signer>),
}

impl Signers {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        match self {
            Signers::Explicit(v) => v.len(),
            Signers::Implicit(v) => v.len(),
        }
    }
}

