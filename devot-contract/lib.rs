#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod voting {
    use ink::storage::Mapping;
    use ink::prelude::{
        vec::Vec,
        string::String,
    };

    // structure used in ink sorage
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Candidate {
        id: u32,
        name: String,
        vote_count: u32,
    }

    // event that is emitted after someone votes
    #[ink(event)]
    pub struct VoteCast {}

    // ink storage, which holds all necessary data
    #[ink(storage)]
    pub struct Voting {
        // map of candidates
        candidates: Mapping<u32, Candidate>,
        // map of anonymous votes, to determine, if a person has already voted
        votes: Mapping<AccountId, bool>,
        // number of candidates, since it is not possible to get number of items from Mapping
        total_candidates: u32,
    }

    impl Voting {
        // instantiate smart contract
        #[ink(constructor)]
        pub fn new(names: Vec<String>) -> Self {
            let mut candidates = Mapping::default();
            for (i, name) in names.iter().enumerate() {
                let i: u32 = i.try_into().expect("value doesn't fit into u32");
                candidates.insert(i, &Candidate {
                    id: i,
                    name: name.clone(),
                    vote_count: 0,
                });
            }
            let name_len: u32 = names.len().try_into().expect("value doesn't fit into u32");
            Self {
                candidates,
                votes: Mapping::default(),
                total_candidates: name_len,
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, candidate_id: u32) {
            let caller = self.env().caller();
            // check if the person has already voted
            assert!(!self.votes.get(caller).unwrap_or(false), "Already voted");
            // check if valid candidate was selected
            assert!(candidate_id < self.total_candidates, "Invalid candidate");

            let mut candidate = self.candidates.get(candidate_id).expect("Candidate not found");
            candidate.vote_count = candidate.vote_count.saturating_add(1); //safe increment
            self.candidates.insert(candidate_id, &candidate);
            self.votes.insert(caller, &true);

            self.env().emit_event(VoteCast {});
        }

        // get list of candidates
        #[ink(message)]
        pub fn get_candidates(&self) -> Vec<Candidate> {
            let mut list = Vec::new();
            for i in 0..self.total_candidates {
                if let Some(c) = self.candidates.get(i) {
                    list.push(c);
                }
            }
            list
        }

        // check if the person has already voted
        #[ink(message)]
        pub fn has_voted(&self, user: AccountId) -> bool {
            self.votes.get(user).unwrap_or(false)
        }
    }
}
