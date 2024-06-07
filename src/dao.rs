use scrypto::prelude::*;

// Define Non-Fungible Token (NFT) data for the DAO
#[derive(NonFungibleData)]
pub struct DAO {
    pub name: String,
    pub desc: String,
    pub num_shares: u32,
}

blueprint! {
    struct DAO {
        dao_token_vault: Vault,
        member_token_vault: Vault, // Vault to hold member tokens
        member_minter_badge: Vault, // Badge to mint member tokens
    }

    impl DAO {
        pub fn instantiate_dao(dao_name: String, description: String, num_shares: u32) -> ComponentAddress {
            // Mint a single fungible token representing the DAO
            let total_shares = num_shares.to_string();
            let dao_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("dao_name", &dao_name)
                .metadata("description", &description)
                .metadata("num_shares", &total_shares)
                .initial_supply(1);

            // Create a badge that allows minting member tokens
            let minter_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Member Minter Badge")
                .initial_supply(1);

            // Create member tokens (NFTs)
            let member_token: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "DAO Member Token")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)
                .no_initial_supply();

            Self {
                dao_token_vault: Vault::with_bucket(dao_bucket),
                member_token_vault: Vault::new(member_token),
                member_minter_badge: Vault::with_bucket(minter_badge),
            }
            .instantiate()
            .globalize()
        }

        // Function to mint member tokens
        pub fn mint_member_token(&mut self, member_id: u64, member_name: String) -> Bucket {
            // Create a unique identifier for the member token
            let member_nft_data = Member {
                id: member_id,
                name: member_name,
                wallet_address: "".to_string(),
            };

            // Mint the member token
            let member_token_bucket = self.member_minter_badge.authorize(|| {
                borrow_resource_manager!(self.member_token_vault.resource_address())
                    .mint_non_fungible(&NonFungibleId::random(), member_nft_data)
            });

            member_token_bucket
        }

        // Function to get DAO information
        pub fn get_dao_info(&self) -> (String, String, u32) {
            let dao_info = borrow_resource_manager!(self.dao_token_vault.resource_address()).metadata();
            let dao_name = dao_info.get("dao_name").unwrap().clone();
            let description = dao_info.get("description").unwrap().clone();
            let num_shares: u32 = dao_info.get("num_shares").unwrap().parse().unwrap();
            (dao_name, description, num_shares)
        }
    }
}

// Define Non-Fungible Token (NFT) data for the Member
#[derive(NonFungibleData)]
pub struct Member {
    pub id: u64,
    pub name: String,
    pub wallet_address: String, // Add wallet address for Radix
}
