#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};

use contract::{
    contract_api::{
        account,
        runtime::{self},
        storage::{self, create_contract_package_at_hash},
        system::{self, transfer_from_purse_to_purse, transfer_from_purse_to_account}
    },
    unwrap_or_revert::UnwrapOrRevert
};
use types::{ApiError, CLType, CLTyped, CLValue, ContractHash, Group, Key, Parameter, PublicKey, RuntimeArgs, U256, U512, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args, system::CallStackElement};

pub enum Error {
    DepositAmountTooSmall = 0,
    WithdrawAmountExceedsBalance = 1,
    NoAccessRights = 2,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn lock() {
    let src_purse: URef = runtime::get_named_arg("src_purse");
    let amount: U512 = runtime::get_named_arg("amount");
    if (amount <= U512::from(0)) {
        runtime::revert(Error::DepositAmountTooSmall);
    }
    let contract_purse_key = runtime::get_key("contract_purse").unwrap_or_revert();
    let contract_purse = *contract_purse_key.as_uref().unwrap_or_revert();
    
    transfer_from_purse_to_purse(src_purse, contract_purse, amount, None)
        .unwrap_or_revert();
    
    let balance = get_key::<U512>("balances", &uref_to_str(&src_purse));
    set_key("balances", &uref_to_str(&src_purse), balance + amount);
}

#[no_mangle]
pub extern "C" fn unlock() {
    _authorization_check();
    let target_purse: URef = runtime::get_named_arg("target_purse");
    let target_pubkey: PublicKey = runtime::get_named_arg("target_pubkey");
    let amount: U512 = runtime::get_named_arg("amount");

    let balance = get_key::<U512>("balances", &uref_to_str(&target_purse));
    if (balance < amount) {
        runtime::revert(Error::WithdrawAmountExceedsBalance);
    }
    let contract_purse_key = runtime::get_key("contract_purse").unwrap_or_revert();
    let contract_purse = *contract_purse_key.as_uref().unwrap_or_revert();

    transfer_from_purse_to_account(contract_purse, target_pubkey.to_account_hash(), amount, None)
        .unwrap_or_revert();

    set_key("balances", &uref_to_str(&target_purse), balance - amount);
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let purse: URef = runtime::get_named_arg("purse");
    let val: U512 = get_key("balances", &purse.to_string());
    ret(val)
}

#[no_mangle]
pub extern "C" fn call() {
    let contract_purse: URef = system::create_purse();

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint(
        "lock",
        vec![
            Parameter::new("src_purse", CLType::URef),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "unlock",
        vec![
            Parameter::new("target_purse", CLType::URef),
            Parameter::new("target_pubkey", CLType::PublicKey),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("purse", CLType::URef)],
        CLType::U512,
    ));

    let dictionary_seed_uref = storage::new_dictionary("cspr_holder_data").unwrap_or_revert();
    storage::dictionary_put(
        dictionary_seed_uref,
        "governance",
        runtime::get_named_arg::<AccountHash>("governance")
    );
    let balances_seed_uref = storage::new_dictionary("balances").unwrap_or_revert();
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "cspr_holder_data".to_string(), 
        dictionary_seed_uref.into()
    );
    named_keys.insert(
        "balances".to_string(), 
        balances_seed_uref.into()
    );
    named_keys.insert(
        "contract_purse".to_string(),
        contract_purse.into()
    );

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    // Save contract and contract hash in the caller's context.
    runtime::put_key("CSPR_Holder", contract_hash.into());
    runtime::put_key("CSPR_Holder_hash", storage::new_uref(contract_hash).into());
    // Save access_uref
    runtime::put_key("access_uref", access_uref.into());
    // Save contract_hash under the contract's dictionary to be accessed through the contract's endpoints.
    storage::dictionary_put(
        dictionary_seed_uref,
        "contract_hash",
        contract_hash,
    );
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_dictionary_seed_uref(name: &str) -> URef {
    let dictionary_seed_uref = match runtime::get_key(name) {
        Some(key) => key.into_uref().unwrap_or_revert(),
        None => {
            let new_dict = storage::new_dictionary(name).unwrap_or_revert();
            let key = storage::new_uref(new_dict).into();
            runtime::put_key(name, key);
            new_dict
        },
    };
    dictionary_seed_uref
}

fn get_key<T: FromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> T {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key).unwrap_or_default().unwrap_or_default()
}

fn set_key<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) { 
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn get_caller() -> Key {
    let mut callstack = runtime::get_call_stack();
    callstack.pop();
    match callstack.last().unwrap_or_revert() {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash: _,
            contract_hash,
        } => (*contract_hash).into(),
    }
}

fn _authorization_check() {
    if (
        get_caller() != 
        Key::Account(get_key::<AccountHash>("cspr_holder_data", "governance"))
    ) {
        runtime::revert(Error::NoAccessRights);
    }
}

fn uref_to_str(uref: &URef) -> String {
    hex::encode(uref.addr())
}