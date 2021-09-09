
use casper_engine_test_support::AccountHash;
use casper_types::{AccessRights, AsymmetricType, ContractHash, Key, PublicKey, U256, U512, URef};

use crate::erc20::{token_cfg, Sender, Token};
use crate::cspr_holder::{Sender as CSPR_Sender, CsprHolder};
use crate::factory::{Sender as F_Sender, Factory};

// ------------ START - ERC20 Tests ------------

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

#[test]
fn test_erc20_deploy() {
    let t = Token::deployed("ERC20", "ERC");
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.bob)), 0.into());
    assert_eq!(t.ali, AccountHash::from_formatted_str("account-hash-fb4215156ad2505de4b230bd8de087cc0443025cd1ad2b468846571d443196ac").unwrap());
}

#[test]
fn test_erc20_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(to_key(t.bob), amount, Sender(t.ali));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply() - amount);
    assert_eq!(t.balance_of(to_key(t.bob)), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let amount = 1.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(to_key(t.ali), amount, Sender(t.bob));
}

#[test]
fn test_erc20_approve() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(to_key(t.bob), amount, Sender(t.ali));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), amount);
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(to_key(t.bob), allowance, Sender(t.ali));
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), allowance);
    t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply() - amount);
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.balance_of(to_key(t.joe)), amount);
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
}

#[test]
fn test_mint() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    let balance = t.balance_of(to_key(t.ali));
    let total_supply = t.total_supply();
    t.mint(
        to_key(t.ali),
        amount,
        Sender(t.ali)
    );
    assert_eq!(t.balance_of(to_key(t.ali)), balance + amount);
    assert_eq!(t.total_supply(), total_supply + amount);
}

#[test]
fn test_mint_to_new_user() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    let total_supply = t.total_supply();
    t.mint(
        to_key(t.bob),
        amount,
        Sender(t.ali)
    );
    assert_eq!(t.balance_of(to_key(t.bob)), amount);
    assert_eq!(t.total_supply(), total_supply + amount);
}

#[test]
#[should_panic]
fn test_mint_to_zero_hash() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    t.mint(
        Key::Hash([0u8; 32]),
        amount,
        Sender(t.ali)
    );
}

#[test]
#[should_panic]
fn test_mint_unauthorized() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    t.mint(
        Key::Hash([0u8; 32]),
        amount,
        Sender(t.bob)
    );
}

#[test]
fn test_burn() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    let balance = t.balance_of(to_key(t.ali));
    let total_supply = t.total_supply();
    t.burn(
        to_key(t.ali),
        amount,
        Sender(t.ali)
    );
    assert_eq!(t.balance_of(to_key(t.ali)), balance - amount);
    assert_eq!(t.total_supply(), total_supply - amount);
}

#[test]
#[should_panic]
fn test_burn_too_much() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    let total_supply = t.total_supply();
    t.mint(
        to_key(t.bob),
        amount,
        Sender(t.ali)
    );
    assert_eq!(t.balance_of(to_key(t.bob)), amount);
    assert_eq!(t.total_supply(), total_supply + amount);
    t.burn(
        to_key(t.bob),
        amount + U256::from(1),
        Sender(t.ali)
    );
}

#[test]
#[should_panic]
fn test_burn_from_zero_hash() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    t.burn(
        Key::Hash([0u8; 32]),
        amount,
        Sender(t.ali)
    );
}

#[test]
#[should_panic]
fn test_burn_unauthorized() {
    let mut t = Token::deployed("ERC20", "ERC");
    let amount = 10.into();
    t.burn(
        Key::Hash([0u8; 32]),
        amount,
        Sender(t.bob)
    );
}

// ------------ START - CsprHolder Tests ------------
#[test]
fn test_cspr_holder_deploy() {
    let b = CsprHolder::deployed();
    assert_eq!(b.ali, AccountHash::from_formatted_str("account-hash-fb4215156ad2505de4b230bd8de087cc0443025cd1ad2b468846571d443196ac").unwrap());
}

#[test]
#[should_panic]
fn test_lock_zero() {
    let mut b = CsprHolder::deployed();
    let amount: U512 = 0.into();
    let main_purse = URef::new(b.ali.value(), AccessRights::READ_ADD_WRITE);
    b.lock(
        main_purse,
        amount,
        CSPR_Sender(b.ali)
    );
}

#[test]
#[should_panic]
fn test_unlock_too_much() {
    let mut b = CsprHolder::deployed();
    let amount: U512 = 10.into();
    let main_purse = URef::new(b.ali.value(), AccessRights::READ_ADD_WRITE);
    b.unlock(
        main_purse,
        PublicKey::ed25519_from_bytes([3u8; 32]).unwrap(),
        amount,
        CSPR_Sender(b.ali)
    );
}

#[test]
#[should_panic]
fn test_unlock_not_owner() {
    let mut b = CsprHolder::deployed();
    let amount: U512 = 10.into();
    let main_purse = URef::new(b.ali.value(), AccessRights::READ_ADD_WRITE);
    b.unlock(
        main_purse,
        PublicKey::ed25519_from_bytes([6u8; 32]).unwrap(),
        amount,
        CSPR_Sender(b.bob)
    );
}

// ------------ START - Factory Tests ------------
#[test]
fn test_factory_deploy() {
    let f = Factory::deployed();
    assert_eq!(f.ali, AccountHash::from_formatted_str("account-hash-fb4215156ad2505de4b230bd8de087cc0443025cd1ad2b468846571d443196ac").unwrap());
}

#[test]
fn test_factory_create_erc20() {
    let mut f = Factory::deployed();
    let contract_hash = f.get_erc20_hash(
        token_cfg::NAME.to_string()
    );
    assert_eq!(contract_hash, ContractHash::default());
    f.create_erc20(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
        f.ali,
        F_Sender(f.ali)
    );
    let contract_hash = f.get_erc20_hash(
        token_cfg::NAME.to_string()
    );
    assert_ne!(contract_hash, ContractHash::default());
}

#[test]
#[should_panic]
fn test_factory_create_unauthorized() {
    let mut f = Factory::deployed();
    f.create_erc20(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
        f.ali,
        F_Sender(f.bob)
    );
}

#[test]
#[should_panic]
fn test_factory_create_redundant() {
    let mut f = Factory::deployed();
    f.create_erc20(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
        f.ali,
        F_Sender(f.ali)
    );
    f.create_erc20(
        token_cfg::NAME.to_string(),
        token_cfg::SYMBOL.to_string(),
        token_cfg::DECIMALS,
        token_cfg::total_supply(),
        f.ali,
        F_Sender(f.ali)
    );
}