module mpc_contract::mpc_contract {
    use std::simple_map::{Self,SimpleMap};
    use std::signer;
    use std::vector;
    use std::string::{Self,String};

    struct Request has store {
        owner: address,
        path: String, // Derivation path (related to node request)
    }

    struct Node has store,drop {
        id: u64,
        addr: address,
    }

    struct SignatureResponse has store {
        r: vector<u8>,
        s: vector<u8>,
        v: u8,
    }

    struct Proposal has store {
        id: u64,
        proposer: address,
        target_node: address, 
        proposal_type: u8,    
        vote_count: u64,      
    }

    struct Vote has store {
        proposal_id: u64,
        voter: address,    
        decision: bool,    
    }

    struct Mpc has key , store {
        root_pk: String, 
        latest_proposal_id: u64, 
        nodes: SimpleMap<address, Node>, 
        proposals: SimpleMap<u64, Proposal>, 
        votes: SimpleMap<u64, vector<Vote>>, 
        latest_request_id: u64, 
        requests: SimpleMap<u64, Request>, 
        responses: SimpleMap<u64, SignatureResponse>, 
        keygen_threshold: u64,
        vote_threshold: u64,
    }
    const ENOT_DEPLOYER: u64 = 0;
    const EPROPOSAL_NOT_FOUND: u64 = 1;
    const EVOTE_ALREADY_CAST: u64 = 2;
    const ENODE_NOT_FOUND: u64 = 3;
    const ENODE_ALREADY_EXISTS: u64 = 4;

    fun init_module(sender: &signer){
        assert!(signer::address_of(sender) == @mpc_contract,ENOT_DEPLOYER );
        move_to(
            sender,
            Mpc {
                root_pk:string::utf8(b"0"),
                latest_proposal_id: 0,
                nodes: simple_map::new(),
                proposals: simple_map::new(),
                votes: simple_map::new(),
                latest_request_id: 0,
                requests: simple_map::new(),
                responses: simple_map::new(),
                keygen_threshold: 0,
                vote_threshold: 0,
            }
        )
    }

    public entry fun request_join(
        node: &signer,
        path: String,
    ) {
        let proposer_addr = signer::address_of(node);
        assert!(exists<Mpc>(@mpc_contract), 10);

        let contract = borrow_global_mut<Mpc>(@mpc_contract);
        let existing_node = simple_map::contains_key(&contract.nodes, &proposer_addr);
        assert!(!existing_node, ENODE_ALREADY_EXISTS);

        let proposal_id = contract.latest_proposal_id+1;
        let new_proposal = Proposal {
            id: proposal_id,
            proposer: proposer_addr,
            target_node: proposer_addr, 
            proposal_type: 0,           // Type 0 = join proposal
            vote_count: 0,
        };

        simple_map::add(&mut contract.proposals, proposal_id, new_proposal);
        contract.latest_proposal_id = proposal_id;
    }

    /// Existing nodes vote on a proposal (either join or remove)
    public entry fun vote_on_proposal(
        account: &signer,
        proposal_id: u64,
        approve: bool,
    ) {
        let voter = signer::address_of(account);
        let contract = borrow_global_mut<Mpc>(@mpc_contract);

        // Ensure the proposal exists
        let proposal = simple_map::borrow_mut(&mut contract.proposals, &proposal_id);
        assert!(proposal.id == proposal_id, EPROPOSAL_NOT_FOUND);

        let is_node = simple_map::contains_key(&contract.nodes, &voter);
        assert!(is_node, ENODE_NOT_FOUND);

        // Prevent double voting
                // Register the vote
        let new_vote = Vote {
            proposal_id,
            voter,
            decision: approve,
        };

        let existing_votes = simple_map::borrow(&contract.votes, &proposal_id);
        let (already_voted,_) = vector::index_of(existing_votes, &new_vote);
        assert!(already_voted, EVOTE_ALREADY_CAST);


        let vote_list = simple_map::borrow_mut(&mut contract.votes, &proposal_id);
        vector::push_back(vote_list, new_vote);

        // Tally votes
        if (approve) {
            proposal.vote_count = proposal.vote_count + 1;
        };

        // If vote count exceeds threshold, finalize the proposal
        if (proposal.vote_count >= contract.vote_threshold) {
            finalize_proposal(proposal_id, contract);
        };
    }

    /// Finalize the outcome of a proposal
    fun finalize_proposal(proposal_id: u64, contract: &mut Mpc) {
        let proposal = simple_map::borrow_mut(&mut contract.proposals, &proposal_id);
        if (proposal.proposal_type == 0) {
            let new_node = Node {
                id: proposal.id,
                addr: proposal.target_node,
            };
            simple_map::add(&mut contract.nodes, proposal.target_node, new_node);
        } else if (proposal.proposal_type == 1) {
            // Handle node removal
            simple_map::remove(&mut contract.nodes, &proposal.target_node);
        };

    }

    /// Deregister a node by making a removal proposal
    public entry fun propose_remove_node(
        account: &signer,
        target_node: address
    ) {
        let proposer_addr = signer::address_of(account);
        let contract = borrow_global_mut<Mpc>(@mpc_contract);
        // Ensure the proposer is an existing node
        let is_node = simple_map::contains_key(&contract.nodes, &proposer_addr);
        assert!(is_node, ENODE_NOT_FOUND);

        // Ensure the target node exists
        let target_exists = simple_map::contains_key(&contract.nodes, &target_node);
        assert!(target_exists, ENODE_NOT_FOUND);

        let proposal_id = contract.latest_proposal_id;
        let remove_proposal = Proposal {
            id: proposal_id,
            proposer: proposer_addr,
            target_node: target_node,
            proposal_type: 1, // Type 1 = remove proposal
            vote_count: 0,
        };

        simple_map::add(&mut contract.proposals, proposal_id, remove_proposal);
        contract.latest_proposal_id = proposal_id + 1;
    }


}
