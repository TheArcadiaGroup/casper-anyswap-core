use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{AsymmetricType, PublicKey, RuntimeArgs, U512, URef, account::AccountHash, runtime_args};

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.

pub struct Sender(pub AccountHash);

pub struct CsprHolder {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl CsprHolder {
    pub fn deployed() -> CsprHolder {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();
        
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("cspr_holder.wasm");
        let session_args = runtime_args! {
            "governance" => ali.to_account_hash()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);
        CsprHolder {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    pub fn contract_hash(&self) -> Hash {
        self.context
            .query(self.ali, &[format!("{}_hash", "CSPR_Holder")])
            .unwrap_or_else(|_| panic!("{} contract not found", "CSPR_Holder"))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", "CSPR_Holder"))
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

    pub fn lock(
        &mut self,
        src_purse: URef,
        amount: U512,
        sender: Sender,
    ) {
        self.call(
            sender,
            "lock",
            runtime_args! {
                "src_purse" => src_purse,
                "amount" => amount
            }
        )
    }

    pub fn unlock(
        &mut self,
        target_pubkey: PublicKey,
        amount: U512,
        sender: Sender,
    ) {
        self.call(
            sender,
            "unlock",
            runtime_args! {
                "target_pubkey" => target_pubkey,
                "amount" => amount
            }
        )
    }
}