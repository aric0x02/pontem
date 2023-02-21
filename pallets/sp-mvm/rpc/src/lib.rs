use std::sync::Arc;
use std::convert::From;
use codec::{self, Codec};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT},
};
use sp_api::ProvideRuntimeApi;
use sp_mvm_rpc_runtime::{MVMApiRuntime, types::MVMApiEstimation};
use frame_support::weights::Weight;
use serde::{Serialize, Deserialize};
use fc_rpc_core::types::Bytes;
pub mod bytecode;
pub mod addr;
pub mod address;
pub mod constant;
pub mod info;
pub mod fn_call;
pub mod model;
pub mod wrappers;
pub mod move_types;
pub mod convert;
pub use crate::move_types::MoveModuleBytecode;
// Estimation struct with serde.
#[derive(Serialize, Deserialize)]
pub struct Estimation {
    pub gas_used: u64,
    pub status_code: u64,
}

impl From<MVMApiEstimation> for Estimation {
    fn from(e: MVMApiEstimation) -> Self {
        Self {
            gas_used: e.gas_used,
            status_code: e.status_code,
        }
    }
}

// RPC calls.
#[rpc]
pub trait MVMApiRpc<BlockHash, AccountId> {
    #[rpc(name = "mvm_gasToWeight")]
    fn gas_to_weight(&self, gas: u64, at: Option<BlockHash>) -> Result<Weight>;

    #[rpc(name = "mvm_weightToGas")]
    fn weight_to_gas(&self, weight: Weight, at: Option<BlockHash>) -> Result<u64>;

    #[rpc(name = "mvm_estimateGasPublish")]
    fn estimate_gas_publish(
        &self,
        account: AccountId,
        module_bc: Bytes,
        gas_limit: u64,
        at: Option<BlockHash>,
    ) -> Result<Estimation>;

    #[rpc(name = "mvm_estimateGasExecute")]
    fn estimate_gas_execute(
        &self,
        account: AccountId,
        tx_bc: Bytes,
        gas_limit: u64,
        at: Option<BlockHash>,
    ) -> Result<Estimation>;

    #[rpc(name = "mvm_getResource")]
    fn get_resource(
        &self,
        account_id: AccountId,
        tag: Bytes,
        at: Option<BlockHash>,
    ) -> Result<Option<Bytes>>;

    #[rpc(name = "mvm_getModuleABI")]
    fn get_module_abi(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;

    #[rpc(name = "mvm_getModule")]
    fn get_module(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;

    #[rpc(name = "mvm_encodeSubmission")]
    fn encode_submission(&self, function: Vec<Bytes>, arguments: Vec<Bytes>, type_parameters: Vec<Bytes>,at: Option<BlockHash>) -> Result<Option<Bytes>>;

    #[rpc(name = "mvm_getModuleABIs")]
    fn get_module_abis(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;
    
    #[rpc(name = "mvm_getModuleABIs2")]
    fn get_module_abis2(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<MoveModuleBytecode>>;
}

pub struct MVMApi<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> MVMApi<C, P> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> MVMApiRpc<<Block as BlockT>::Hash, AccountId> for MVMApi<C, Block>
where
    Block: BlockT,
    AccountId: Clone + std::fmt::Display + Codec,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: MVMApiRuntime<Block, AccountId>,
{
    fn gas_to_weight(&self, gas: u64, at: Option<<Block as BlockT>::Hash>) -> Result<Weight> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api.gas_to_weight(&at, gas);

        res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during requesting Runtime API".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn weight_to_gas(&self, weight: Weight, at: Option<<Block as BlockT>::Hash>) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api.weight_to_gas(&at, weight);

        res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during requesting Runtime API".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn estimate_gas_publish(
        &self,
        account: AccountId,
        module_bc: Bytes,
        gas_limit: u64,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Estimation> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api
            .estimate_gas_publish(&at, account, module_bc.into_vec(), gas_limit)
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error during requesting Runtime API".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

        let mvm_estimation = res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during publishing module for estimation".into(),
            data: Some(format!("{:?}", e).into()),
        })?;

        Ok(Estimation::from(mvm_estimation))
    }

    fn estimate_gas_execute(
        &self,
        account: AccountId,
        tx_bc: Bytes,
        gas_limit: u64,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Estimation> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let res = api
            .estimate_gas_execute(&at, account, tx_bc.into_vec(), gas_limit)
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error during requesting Runtime API".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

        let mvm_estimation = res.map_err(|e| RpcError {
            code: ErrorCode::ServerError(500),
            message: "Error during script execution for estimation".into(),
            data: Some(format!("{:?}", e).into()),
        })?;

        Ok(Estimation::from(mvm_estimation))
    }

    fn get_resource(
        &self,
        account_id: AccountId,
        tag: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<Bytes>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let f: Option<Vec<u8>> = api
            .get_resource(&at, account_id, tag.into_vec())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "ABI error".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error from method".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
        Ok(f.map(Into::into))
    }

    fn get_module_abi(
        &self,
        module_id: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<Bytes>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let f: Option<Vec<u8>> = api
            .get_module_abi(&at, module_id.into_vec())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "API error".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Error from method".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
        Ok(f.map(Into::into))
    }

    fn get_module(
        &self,
        module_id: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<Bytes>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let f: Option<Vec<u8>> = api
            .get_module(&at, module_id.into_vec())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "API error.".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Nope, error.".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
        Ok(f.map(Into::into))
    }

    fn encode_submission(
        &self,
        function: Vec<Bytes>,  
        arguments: Vec<Bytes>, 
        type_parameters: Vec<Bytes>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<Bytes>> {
       
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        let ff = function.into_iter().map(|func|String::from_utf8(func.into_vec()).unwrap()).collect::<Vec<String>>();
        let ((module_id,module_address),module_name,func) = (crate::fn_call::parse_function_string(&ff[0],&ff[1]).unwrap(),ff[1].clone(),ff[2].clone());
 println!("{:?},{:?},{:?},{:?},{:?}",module_id,module_address,ff[0],module_name,func);
        let f: Option<Vec<u8>> = api
            .get_module(&at, module_id.unwrap())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "API error.".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "Nope, error.".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
println!("make_function_call====");
        let f = crate::fn_call::make_function_call(&f.as_ref().unwrap(),module_address,module_name,func,type_parameters.into_iter().map(|a| String::from_utf8(a.into_vec()).unwrap()).collect(),arguments.into_iter().map(|a| String::from_utf8(a.into_vec()).unwrap()).collect()).map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "call Nope, error.".into(),
                data: Some(
                   format!("{:?}",e)
                        .into(),
                ),
            }).ok();
println!("make_function_call=result==={:?}===",f);
//   MoveModuleBytecode::new(module.clone())
//                             .try_parse_abi()
//                             .context("Failed to parse move module ABI")
//                             .map_err(|err| {
//                                 BasicErrorWith404::internal_with_code(
//                                     err,
//                                     AptosErrorCode::InternalError,
//                                     &self.latest_ledger_info,
//                                 )
//                             })?,

        Ok(f.map(Into::into))
    }

    fn get_module_abis(
        &self,
        module_id: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<Bytes>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let f: Option<Vec<u8>> = api
            .get_module(&at, module_id.into_vec())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "abi API error.".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "abi Nope, error.".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
 let f = crate::fn_call::make_abi(&f.as_ref().unwrap()).map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "make_abi Nope, error.".into(),
                data: Some(
                   format!("{:?}",e)
                        .into(),
                ),
            }).ok();
        let ff=serde_json::to_vec(&f.as_ref().unwrap()).ok();
        println!("test_get_module_abis=result==={:?}=={:?}=",f,ff);
        // let f:Option<Vec<u8>>=Some(ff.bytes().collect());
        let f=ff;
        Ok(f.map(Into::into))
    }

 fn get_module_abis2(
        &self,
        module_id: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<MoveModuleBytecode>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let f: Option<Vec<u8>> = api
            .get_module(&at, module_id.into_vec())
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "abi2 API error.".into(),
                data: Some(e.to_string().into()),
            })?
            .map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "abi2 Nope, error.".into(),
                data: Some(
                    std::str::from_utf8(e.as_slice())
                        .unwrap_or("can't decode error")
                        .into(),
                ),
            })?;
        let f = crate::fn_call::make_abi(&f.as_ref().unwrap()).map_err(|e| RpcError {
                code: ErrorCode::ServerError(500),
                message: "make_abi2 Nope, error.".into(),
                data: Some(
                   format!("{:?}",e)
                        .into(),
                ),
            }).ok();
        // let ff=serde_json::to_vec(&f.as_ref().unwrap()).ok();
        println!("test_get_module_abis2=result==={:?}===",f);
        // let f:Option<Vec<u8>>=Some(ff.bytes().collect());
        Ok(f.map(Into::into))
    }
}
