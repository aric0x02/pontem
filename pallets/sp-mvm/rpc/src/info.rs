// use sp_std::prelude::*;
use anyhow::Error;
use move_binary_format::access::{ModuleAccess, ScriptAccess};
use move_binary_format::CompiledModule;
use move_binary_format::file_format::{
    Ability, AbilitySet, SignatureToken, StructHandleIndex, Visibility,
};
use move_core_types::account_address::AccountAddress;
// use crate::bytecode::accessor::{Bytecode, BytecodeRef};
use move_core_types::identifier::{IdentStr, Identifier};
use move_core_types::value::{MoveTypeLayout};
use crate::constant::sig_to_ty;

//    use sp_std::{vec::Vec, prelude::*, default::Default};
//  use scale_info::prelude::string::String;
// use sp_std::boxed::Box;
#[derive(Debug)]
pub struct BytecodeInfo {
    // bytecode: Bytecode,
}

// impl From<Bytecode> for BytecodeInfo {
//     fn from(bytecode: Bytecode) -> Self {
//         BytecodeInfo { bytecode }
//     }
// }

// impl BytecodeInfo {
    // pub fn bytecode_ref(&self) -> &BytecodeRef {
    //     match &self.bytecode {
    //         Bytecode::Script(_, _, _, rf) => rf,
    //         Bytecode::Module(_, rf) => rf,
    //     }
    // }

    // pub fn serialize(&self, binary: &mut Vec<u8>) -> Result<(), Error> {
    //     match &self.bytecode {
    //         Bytecode::Script(_, script, _, _) => script.serialize(binary),
    //         Bytecode::Module(module, _) => module.serialize(binary),
    //     }
    // }

    // pub fn is_module(&self) -> bool {
    //     match &self.bytecode {
    //         Bytecode::Script(_, _, _, _) => false,
    //         Bytecode::Module(_, _) => true,
    //     }
    // }

    // pub fn address(&self) -> Option<AccountAddress> {
    //     match &self.bytecode {
    //         Bytecode::Script(_, _, _, _) => None,
    //         Bytecode::Module(bytecode, _) => Some(*bytecode.address()),
    //     }
    // }

    // pub fn name(&self) -> String {
    //     match &self.bytecode {
    //         Bytecode::Script(name, _, _, _) => name.to_string(),
    //         Bytecode::Module(bytecode, _) => bytecode.self_id().name().to_string(),
    //     }
    // }

    pub fn find_script_function(module: &CompiledModule, need_name: &str) -> Option<Script> {
        module
                .function_defs()
                .iter()
                .filter(|def| def.visibility == Visibility::Script)
                .find(|def| {
                    let handle = module.function_handle_at(def.function);
                    module.identifier_at(handle.name).as_str() == need_name
                })
                .map(|def| {
                    let handle = module.function_handle_at(def.function);
                    let parameters = module
                        .signature_at(handle.parameters)
                        .0
                        .iter()
                        .map(|p| make_type(p))
                        .collect();

                    let type_parameters = handle
                        .type_parameters
                        .iter()
                        .map(TypeAbilities::from)
                        .collect();
                    let return_ = &module.signature_at(handle.return_).0;

                    Script {
                        name: String::from(module.identifier_at(handle.name).as_str()),
                        parameters,
                        type_parameters,
                        returns: return_.iter().map(|st| make_type(st)).collect(),
                    }
                })
          
    }

// }

#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub parameters: Vec<MoveTypeLayout>,
    pub type_parameters: Vec<TypeAbilities>,
    pub returns: Vec<MoveTypeLayout>,
}

impl Script {
    pub fn type_params_count(&self) -> usize {
        self.type_parameters.len()
    }
}

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub enum Type {
//     Bool,
//     U8,
//     U64,
//     U128,
//     Address,
//     Signer,
//     Vector(Box<Type>),
//     Struct(StructDef),
//     Reference(Box<Type>),
//     MutableReference(Box<Type>),
//     TypeParameter(u16),
// }

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub struct StructDef {
//     pub address: AccountAddress,
//     pub module_name: String,
//     pub name: String,
//     pub type_parameters: Vec<Type>,
// }

fn make_type(tok: &SignatureToken) -> MoveTypeLayout {
    // match tok {
    //     SignatureToken::Bool => Type::Bool,
    //     SignatureToken::U8 => Type::U8,
    //     SignatureToken::U64 => Type::U64,
    //     SignatureToken::U128 => Type::U128,
    //     SignatureToken::Address => Type::Address,
    //     SignatureToken::Signer => Type::Signer,
    //     SignatureToken::Vector(tp) => Type::Vector(Box::new(make_type(tp, module))),
    //     SignatureToken::Struct(idx) => Type::Struct(make_struct_def(*idx, &[], module)),
    //     SignatureToken::StructInstantiation(idx, tps) => {
    //         Type::Struct(make_struct_def(*idx, tps, module))
    //     }
    //     SignatureToken::Reference(rf) => Type::Reference(Box::new(make_type(rf, module))),
    //     SignatureToken::MutableReference(tp) => {
    //         Type::MutableReference(Box::new(make_type(tp, module)))
    //     }
    //     SignatureToken::TypeParameter(val) => Type::TypeParameter(*val),
    // }
    sig_to_ty(tok).unwrap()
}

// fn make_struct_def(
//     idx: StructHandleIndex,
//     tps: &[SignatureToken],
//     module: &CompiledModule,
// ) -> StructDef {
//     let struct_handle = module.struct_handle_at(idx);
//     let struct_module_handle = module.module_handle_at(struct_handle.module);

//     StructDef {
//         address: *module.address_identifier_at(struct_module_handle.address),
//         name: String::from(module.identifier_at(struct_handle.name).as_str()),
//         type_parameters: tps.iter().map(|tok| make_type(tok, module)).collect(),
//         module_name: String::from(module.identifier_at(struct_module_handle.name).as_str()),
//     }
// }

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeAbilities {
    pub abilities: Vec<TypeAbility>,
}

impl From<&AbilitySet> for TypeAbilities {
    fn from(val: &AbilitySet) -> Self {
        TypeAbilities {
            abilities: val
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => TypeAbility::Copy,
                    Ability::Drop => TypeAbility::Drop,
                    Ability::Store => TypeAbility::Store,
                    Ability::Key => TypeAbility::Key,
                })
                .collect::<Vec<TypeAbility>>(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeAbility {
    Copy,
    Drop,
    Store,
    Key,
}
