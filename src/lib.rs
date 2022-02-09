use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,PanicOnDefault};
use near_sdk::json_types::U128;
use near_sdk::collections::{UnorderedMap};
use serde::{Deserialize, Serialize};


#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;


#[derive(BorshDeserialize, BorshSerialize,Deserialize, Serialize,PartialEq,Debug)]
pub struct Item{
    title: String,
    score:u16,
    content: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Content {
    Raw,
    IPFS,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract{
    pub item: UnorderedMap<u128, Item>
}
#[near_bindgen]
impl Contract{
    #[init]
    pub fn new() -> Self{
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            item: UnorderedMap::new(b"scv-chain".to_vec()),
        }
    }

    pub fn get_item_info(&self, id:U128) {
        let stored_item = self.get_item(id);
        match stored_item {
            Some(stored_item) => {
                let log_message = format!("\ntitle: {}\n score: {}\n content: {}"
                , stored_item.title
                ,stored_item.score
                ,stored_item.content);
                env::log(log_message.as_bytes());
            },
            None => {
                env::log("Item not found".as_bytes());
            }
        }
    }

    pub fn create_item(&mut self,id: U128,
                        title:String,
                        score:u16,
                        content:String,)
    {


        let existed_item: Option<Item> = self.item.get(&id.into());
        if existed_item.is_some() {
            env::panic(b"Sorry, already added this item.")
        }
        let item = Item{title: title, score:score, content:content};
        self.item.insert(&id.into(),&item);
    }

    pub fn revoke_item(&mut self, id: U128){
        let existed_item: Option<Item> = self.item.get(&id.into());
        if existed_item.is_none() {
            env::panic(b"Sorry, No item found")
        }

        self.item.remove(&id.into());
    }

    pub fn reset_all(&mut self){
        assert_eq!(env::current_account_id()
                    , env::predecessor_account_id()
                    , "To reset all the items, this method must be called by the contract owner.");
        self.item.clear();
        env::log(b"All the items have been revoked.");
    }
}


impl Contract{
    pub fn get_item(&self, id:U128) -> Option<Item> {
        self.item.get(&id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};
    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn create() {
        let context = get_context(vec![],false,"bob.testnet".to_string());
        testing_env!(context);
        let mut contract = Contract::new();
        let id_1 = U128(1001);
        let item_1 = Item{
                                            title:"Who am I".to_string(),
                                            score: 5,
                                            content: "link file cv".to_string(),
                                        };
        contract.create_item(id_1, "Who am I".to_string(), 5, "link file cv".to_string());
        assert_eq!(contract.get_item(id_1.into()).unwrap(),item_1);
    }

    #[test]
    fn revoke() {
        let context = get_context(vec![],false,"bob.testnet".to_string());
        testing_env!(context);
        let mut contract = Contract::new();
        let id = U128(1001);
        contract.create_item(id, "Who am I".to_string(), 5, "link file cv".to_string());
        contract.revoke_item(id);
        assert_eq!(contract.get_item(id), None);
    }
}
