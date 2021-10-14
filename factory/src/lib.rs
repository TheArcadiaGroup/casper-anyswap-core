#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;
use std::ops::{Add, Sub};

use contract::{contract_api::{runtime::{self, blake2b}, storage::{self, create_contract_package_at_hash}}, unwrap_or_revert::UnwrapOrRevert};
use types::{ApiError, CLType, CLTyped, CLValue, ContractHash, Group, Key, Parameter, RuntimeArgs, U256, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args, system::CallStackElement};

pub enum Error {
    CannotMintToZeroHash = 0,
    CannotBurnFromZeroHash = 1,
    BurnAmountExceedsBalance = 2,
    NoAccessRights = 3,
    TokenExists = 4,
    TokenNotFound = 5,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

// ERC20 endpoints - Start
#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("token_metadata", "name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("token_metadata", "symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = get_key("token_metadata", "decimals");
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("token_metadata", "total_supply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let val: U256 = get_key("balances", &key_to_str(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let val: U256 = get_key_runtime(&allowance_key(&owner, &spender));
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    _approve(
        get_caller(),
        spender,
        amount
    );
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer(
        get_caller(),
        recipient,
        amount
    );
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer_from(owner, recipient, amount);
}

#[no_mangle]
pub extern "C" fn mint() {
    _authorization_check_erc20();
    let owner: Key = runtime::get_named_arg("owner");
    let amount: U256 = runtime::get_named_arg("amount");
    if (owner == Key::Hash([0u8; 32]) || owner == Key::Account(AccountHash::new([0u8; 32]))) {
        runtime::revert(Error::CannotMintToZeroHash);
    }
    let total_supply = get_key::<U256>("token_metadata", "total_supply");
    set_key("token_metadata", "total_supply", total_supply.add(amount));
    let balance = get_key::<U256>("balances", &key_to_str(&owner));
    set_key("balances", &key_to_str(&owner), balance.add(amount));
}

#[no_mangle]
pub extern "C" fn burn() {
    _authorization_check_erc20();
    let owner: Key = runtime::get_named_arg("owner");
    let amount: U256 = runtime::get_named_arg("amount");
    if (owner == Key::Hash([0u8; 32]) || owner == Key::Account(AccountHash::new([0u8; 32]))) {
        runtime::revert(Error::CannotBurnFromZeroHash);
    }
    let balance = get_key::<U256>("balances", &key_to_str(&owner));
    if (balance < amount) {
        runtime::revert(Error::BurnAmountExceedsBalance);
    }
    set_key("balances", &key_to_str(&owner), balance.sub(amount));
    let total_supply = get_key::<U256>("token_metadata", "total_supply");
    set_key("token_metadata", "total_supply", total_supply.sub(amount));
}
// ERC20 endpoints - END
#[no_mangle]
pub extern "C" fn get_erc20_hash() {
    let token_name: String = runtime::get_named_arg("token_name");
    let token_hash = get_key::<ContractHash>("tokens", &token_name);
    if (token_hash == ContractHash::default()) {
        runtime::revert(Error::TokenNotFound);
    }
    ret(token_hash)
}

#[no_mangle]
pub extern "C" fn create_erc20() {
    _authorization_check();
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let token_decimals: u8 = runtime::get_named_arg("token_decimals");
    let token_total_supply: U256 = runtime::get_named_arg("token_total_supply");
    let governance: AccountHash = runtime::get_named_arg("governance");

    if (get_key::<ContractHash>("tokens", &token_name) != ContractHash::default()) {
        runtime::revert(Error::TokenExists);
    }

    let entry_points = set_erc20_entry_points();

    // remove old dictionaries:
    runtime::remove_key("token_metadata");
    runtime::remove_key("external");
    runtime::remove_key("balances");
    runtime::remove_key("internal");

    let dictionary_seed_uref = storage::new_dictionary("token_metadata").unwrap_or_revert();
    storage::dictionary_put(
        dictionary_seed_uref,
        "name",
        token_name.clone()
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "symbol",
        token_symbol
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "decimals",
        token_decimals
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "total_supply",
        token_total_supply
    );
    let external_seed_uref = storage::new_dictionary("external").unwrap_or_revert();
    storage::dictionary_put(
        external_seed_uref,
        "governance",
        governance
    );
    let balances_seed_uref = storage::new_dictionary("balances").unwrap_or_revert();
    storage::dictionary_put(
        balances_seed_uref,
        &key_to_str(&Key::Account(runtime::get_caller())),
        token_total_supply
    );
    let internal_seed_uref = storage::new_dictionary("internal").unwrap_or_revert();
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "token_metadata".to_string(), 
        dictionary_seed_uref.into()
    );
    named_keys.insert(
        "external".to_string(), 
        external_seed_uref.into()
    );
    named_keys.insert(
        "balances".to_string(), 
        balances_seed_uref.into()
    );
    named_keys.insert(
        "internal".to_string(), 
        internal_seed_uref.into()
    );

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key(&token_name, contract_hash.into());
    runtime::put_key([&token_name, "_hash"].join("").as_str(), storage::new_uref(contract_hash).into());
    // Save the contract's hash in its internal dictionary.
    storage::dictionary_put(
        internal_seed_uref,
        "contract_hash",
        contract_hash,
    );
    // Save the contract's hash in factory's tokens dictionary.
    set_key(
        "tokens",
        &token_name,
        contract_hash
    );
}

#[no_mangle]
pub extern "C" fn call() {
    let governance: AccountHash = runtime::get_named_arg("governance");

    let external_seed_uref = storage::new_dictionary("factory_external").unwrap_or_revert();
    storage::dictionary_put(
        external_seed_uref,
        "governance",
        governance
    );
    let internal_seed_uref = storage::new_dictionary("factory_internal").unwrap_or_revert();
    let tokens_seed_uref = storage::new_dictionary("tokens").unwrap_or_revert();
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "factory_external".to_string(), 
        external_seed_uref.into()
    );
    named_keys.insert(
        "factory_internal".to_string(), 
        internal_seed_uref.into()
    );
    named_keys.insert(
        "tokens".to_string(), 
        tokens_seed_uref.into()
    );

    let mut entry_points = set_erc20_entry_points();
    entry_points.add_entry_point(endpoint(
        "get_erc20_hash",
        vec![
            Parameter::new("token_name", CLType::String)
        ],
        ContractHash::cl_type(),
    ));
    entry_points.add_entry_point(endpoint(
        "create_erc20",
        vec![
            Parameter::new("token_name", CLType::String),
            Parameter::new("token_symbol", CLType::String),
            Parameter::new("token_decimals", CLType::U8),
            Parameter::new("token_total_supply", CLType::U256),
            Parameter::new("governance", AccountHash::cl_type()),
        ],
        CLType::Unit,
    ));

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    // Save contract and contract hash in the caller's context.
    runtime::put_key("Factory", contract_hash.into());
    runtime::put_key("Factory_hash", storage::new_uref(contract_hash).into());
    // Save access_uref
    runtime::put_key("access_uref", access_uref.into());
    // Save contract_hash under the contract's dictionary to be accessed through the contract's endpoints.
    storage::dictionary_put(
        internal_seed_uref,
        "contract_hash",
        contract_hash,
    );
}

fn set_erc20_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("decimals", vec![], CLType::U8));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint(
        "transfer",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", CLType::Key)],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "allowance",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("spender", CLType::Key),
        ],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "approve",
        vec![
            Parameter::new("spender", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_from",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "mint",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "burn",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points
}

fn _transfer(sender: Key, recipient: Key, amount: U256) {
    let new_sender_balance: U256 = (get_key::<U256>("balances", &key_to_str(&sender)) - amount);
    set_key("balances", &key_to_str(&sender), new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>("balances", &key_to_str(&recipient)) + amount);
    set_key("balances", &key_to_str(&recipient), new_recipient_balance);
}

fn _transfer_from(owner: Key, recipient: Key, amount: U256) {
    let key = allowance_key(&owner, &get_caller());
    _transfer(owner, recipient, amount);
    _approve(
        owner,
        get_caller(),
        (get_key_runtime::<U256>(&key) - amount),
    );
}

fn _approve(owner: Key, spender: Key, amount: U256) {
    set_key_runtime(&allowance_key(&owner, &spender), amount);
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    }
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

fn get_key_runtime<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key_runtime<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn get_key<T: FromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> T {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key).unwrap_or_default().unwrap_or_default()
}

fn set_key<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) { 
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

fn allowance_key(owner: &Key, sender: &Key) -> String {
    format!("allowances_{}_{}", owner, sender)
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

fn _authorization_check_erc20() {
    if (
        get_caller() != 
        Key::Account(
            get_key::<AccountHash>(
            "external",
            "governance"
            )
        )
    ) {
        runtime::revert(Error::NoAccessRights);
    }
}

fn _authorization_check() {
    if (
        get_caller() != 
        Key::Account(
            get_key::<AccountHash>(
            "factory_external",
            "governance"
            )
        )
    ) {
        runtime::revert(Error::NoAccessRights);
    }
}
