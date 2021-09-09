use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{AsymmetricType, CLTyped, ContractHash, PublicKey, RuntimeArgs, U256, U512, account::AccountHash, bytesrepr::FromBytes, runtime_args};

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.

pub struct Sender(pub AccountHash);

pub struct Factory {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Factory {
    pub fn deployed() -> Factory {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();
        
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("factory.wasm");
        let session_args = runtime_args! {
            "governance" => ali.to_account_hash()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);
        Factory {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    pub fn contract_hash(&self) -> Hash {
        self.context
            .query(self.ali, &[format!("{}_hash", "Factory")])
            .unwrap_or_else(|_| panic!("{} contract not found", "Factory"))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", "Factory"))
    }

    /// query a contract's dictionary's key.
    fn query_contract_dictionary<T: CLTyped + FromBytes>(
        &self,
        key: AccountHash,
        context: &TestContext,
        dictionary_name: String,
        name: String,
    ) -> Option<T> {
        match context.query_dictionary_item(key.into(), Some(dictionary_name), name.clone()) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not the expected type.", name));
                Some(value)
            }
        }
    }

    /// call a contract's specific entry point.
    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn get_erc20_hash(&self, token_name: String) -> ContractHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "tokens".to_string(),
            token_name
        ).unwrap_or_default()
    }

    pub fn create_erc20(
        &mut self,
        token_name: String,
        token_symbol: String,
        token_decimals: u8,
        token_total_supply: U256,
        governance: AccountHash,
        sender: Sender,
    ) {
        self.call(
            sender,
            "create_erc20",
            runtime_args! {
                "token_name" => token_name,
                "token_symbol" => token_symbol,
                "token_decimals" => token_decimals,
                "token_total_supply" => token_total_supply,
                "governance" => governance
            }
        )
    }
}