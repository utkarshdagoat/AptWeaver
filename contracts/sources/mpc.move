module mpc_contract::mpc_contract {
    use std::vector;
    use std::simple_map::SimpleMap;
    use std::string::String;
    struct InitializingContractState has store {
        threshold: u32
    }

    struct Candidates has store {
        pubkey: address,
        cipher_key: vector<u8>,
        sign_pk: vector<u8>,
        ip_address: String
    }

    struct RunningContractState has store {
        epoch: u32,
        threshold: u32,
        root_pubkey: vector<u8>,
        candidates: SimpleMap<u32,Candidates>
    }

    enum ProtocolStatus has store {
        NotInitialized,
        IntializingState(InitializingContractState),
        Ready(),
        Resharing()
    }

    struct MpcContractState has key {
        protocol_status: ProtocolStatus,
        pending_requests: SimpleMap<address, u32>,
        request_counter: u32
    }
}
