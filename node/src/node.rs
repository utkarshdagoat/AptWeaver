use crate::aptos_client::NODE_URL;
use aptos_sdk::rest_client::Client;
use queues::*;
use serde::{Deserialize, Serialize};
use two_party_ecdsa::party_one::EphEcKeyPair;
use std::collections::BTreeMap;
use std::error::Error;
use two_party_ecdsa::party_two::{
    EphKeyGenFirstMsg as SecondEphKeyGenFirstMsg, EphKeyGenSecondMsg as SecondEphKeyGenSecondMsg,
};
use two_party_ecdsa::{
    curv::{elliptic::curves::traits::ECScalar, BigInt},
    party_one::Converter,
};
use two_party_ecdsa::{
    party_one::{
       EcKeyPair,  EphKeyGenFirstMsg, EphKeyGenSecondMsg,
        KeyGenFirstMsg, PaillierKeyPair, Party1Private,
        Signature
    },
    party_two::PartialSig,
};

#[derive(Serialize, Deserialize)]
pub struct PartyOneState {
    ec_key_pair: EcKeyPair,
    eph_key_pair_first: EphEcKeyPair,
    paillier_keypair: PaillierKeyPair,
    ephemeral_keygen_first_msg: EphKeyGenFirstMsg,
    private_key: Party1Private,
    commitments_verified: bool,
    message: BigInt,
}

pub struct Party1Node {
    aptos_client: Client,
    client_state: BTreeMap<u32, PartyOneState>,
    last_completed: u32,
    latest_request_index: u32,
    pending_requests: Queue<u32>,
    ec_keypair: EcKeyPair,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyCommitmentsData {
    request_id: u32,
    eph_keygen_first_msg: SecondEphKeyGenFirstMsg,
    eph_keygen_second_msg: SecondEphKeyGenSecondMsg,
}

#[derive(Serialize, Deserialize)]
pub struct SignatureGenData {
    request_id: u32,
    partial_sig: PartialSig,
    eph_keygen_second_msg: SecondEphKeyGenSecondMsg,
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    VerifyCommitments(VerifyCommitmentsData),
    SignatureGen(SignatureGenData),
}

impl Party1Node {
    pub fn new() -> Party1Node {
        let aptos_client = Client::new(NODE_URL.clone());
        let client_state = BTreeMap::new();
        let last_completed = 0;
        let latest_request_index = 0;
        let pending_requests = queue![];
        let (_, _, ec_keypair) = KeyGenFirstMsg::create_commitments();
        Party1Node {
            aptos_client,
            client_state,
            last_completed,
            latest_request_index,
            pending_requests,
            ec_keypair,
        }
    }

    async fn recieve() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn add_request(&mut self) {
        let request = self.latest_request_index + 1;
        self.pending_requests
            .add(request)
            .expect("Failed to add request");
        self.latest_request_index = request;
    }
    // pub fn start_new_request(&mut self, derivation_path: &str, message: &str) {
    //     let new_request_id = self
    //         .pending_requests
    //         .remove()
    //         .expect("Failed to remove request");
    //     let derivation_path_scalar = ECScalar::from(&BigInt::from_hex(derivation_path));
    //     let msg = BigInt::from_hex(message);
    //     let new_ec_keypair = self.ec_keypair.add_scalar(&derivation_path_scalar);
    //     let keypair = PaillierKeyPair::generate_keypair_and_encrypted_share(&new_ec_keypair);
    //     let (eph_party_one_first_message, eph_ec_key_pair_party1) = EphKeyGenFirstMsg::create();
    //     let party1_private = Party1Private::set_private_key(&new_ec_keypair, &keypair);
    //     let party1_state = PartyOneState {
    //         ec_key_pair: new_ec_keypair,
    //         paillier_keypair: keypair,
    //         ephemeral_keygen_first_msg: eph_party_one_first_message,
    //         private_key: party1_private,
    //         commitments_verified: false,
    //         message: msg,
    //         eph_key_pair_first:eph_ec_key_pair_party1
    //     };
    //     self.client_state.insert(new_request_id, party1_state);
    // }

    fn verify_commitments(&mut self, data: VerifyCommitmentsData) {
        let party1_state = self
            .client_state
            .get_mut(&data.request_id)
            .expect("Failed to get party1 state");
        match EphKeyGenSecondMsg::verify_commitments_and_dlog_proof(
            &data.eph_keygen_first_msg,
            &data.eph_keygen_second_msg,
        ) {
            Ok(_) => party1_state.commitments_verified = true,
            Err(_) => party1_state.commitments_verified = false,
        }
    }

    fn compute_signature(&mut self, data: SignatureGenData) -> Signature {
        let party1_state = self
            .client_state
            .get_mut(&data.request_id)
            .expect("Failed to get party1 state");
        Signature::compute(
            &party1_state.private_key,
            &data.partial_sig.c3,
            &party1_state.eph_key_pair_first,
            &data.eph_keygen_second_msg.comm_witness.public_share,
        )
    }
}
