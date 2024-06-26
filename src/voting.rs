// Primary blueprint purpose
// Validate Proof of authorized caller ie founders_badge or operators_badge to create ballots &
// create vote_tokens with total supply of num_votes param or num_shares(total supply of member tokens)

// accept votes from callers with proof of member_tokens
// deposit vote_tokens into vault corresponding to vote_option @ vote_tokens * number_member_tokens held

use scrypto::prelude::*;

blueprint! {
 struct Voting {
    admin_badge_vault: Vault,
    // vaults collection for constructing ballot vaults
    vaults: HashMap<ResourceAddress, Vault>,
    vote_token_vault: Vault,
    total_shares: u128,
 }

 impl Voting {
     // TODO implement access rules to accept founders or operators badge for auth
   pub fn instantiate_voting(_auth_badge: Proof, total_shares: u128) -> ComponentAddress {

     let admin_badge: Bucket = ResourceBuilder::new_fungible()
     .divisibility(DIVISIBILITY_NONE)
     .metadata("name", "Admin Badge")
     .metadata("description", "An admin badge used for internal functionality of creating vote tokens & ballots.")
     .initial_supply(dec!("1"));

     let vote_token: Bucket = ResourceBuilder::new_fungible()
          .divisibility(DIVISIBILITY_NONE)
          .metadata("name", "Vote Token")
          .metadata("symbol", "Vote")
          .initial_supply(total_shares);


      Self {
     admin_badge_vault: Vault::with_bucket(admin_badge),
     vaults: HashMap::new(),
     vote_token_vault: Vault::with_bucket(vote_token),
     total_shares: total_shares,
     }
     .instantiate()
     .globalize()
   }

   pub fn create_ballot(&mut self, _ballot_options: HashMap<String, String>) {
    // Iterate over ballot_options and create a vault/token pair for each option
       // Iterate over ballot_options and create a vault/token pair for each option
    for (option_name, option_description) in _ballot_options {
        let ballot_resource = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", &option_name)
            .metadata("description", &option_description)
            .initial_supply(0); // Initialize with zero tokens
        
        let ballot_address = ballot_resource.resource_address(); // Get the resource address

        // Create the vault with the resource address
        let ballot_vault = Vault::new(ballot_address);

        // Insert the vault into the vaults HashMap
        self.vaults.insert(ballot_address, ballot_vault);
    }
}

// require proof of voters badge --> include num_member_tokens for weighted votes/delegate voters
   pub fn operators_vote(_ballot_name: String, _vote: String, _num_votes: u32) {
     // general purpose voting mechanism for internal initiatives voted on by operators only
    // collect votes
    // deposit signed ballot w/num_votes * vote_token into vault with corresponding ballot_name/vote

    // report results
   }

  //  Special mechanism for election of voter delegates and DAO operator positions
   pub fn create_election() {
    // construct ballots
    // create vault to collect votes
    }

   pub fn tally_votes(_ballot_id: ResourceAddress) {
     // get list of vaults assaciated with ballot_id
     // count num of tokens for each ballot vault
     // evaluate ballot vaults to determine ranked results
     // report results
   }

   pub fn simple_nft_vote() {
    // construct simple list ballot + config num of options that can be selected
    // mint nft with ballot selections and deposit into nft_proposals_vault
    // voter must present proof of required badge, ie. members badge, delegate badge or other acceptable badge.
   }

 }
}
