// For integration tests, please add your tests in /tests instead

#[cfg(test)]
mod tests {

    use std::fmt::format;

    use party_one::PaillierKeyPair;
    use serde::de;

    use crate::curv::arithmetic::traits::Samplable;
    use crate::curv::elliptic::curves::traits::*;
    use crate::curv::BigInt;
    use crate::*;
    use sha3::{Digest, Sha3_256};

    #[test]
    fn test_d_log_proof_party_two_party_one() {
        let (party_one_first_message, comm_witness, _ec_key_pair_party1) =
            party_one::KeyGenFirstMsg::create_commitments();
        let (party_two_first_message, _ec_key_pair_party2) = party_two::KeyGenFirstMsg::create();
        let party_one_second_message = party_one::KeyGenSecondMsg::verify_and_decommit(
            comm_witness,
            &party_two_first_message.d_log_proof,
        )
        .expect("failed to verify and decommit");

        let _party_two_second_message =
            party_two::KeyGenSecondMsg::verify_commitments_and_dlog_proof(
                &party_one_first_message,
                &party_one_second_message,
            )
            .expect("failed to verify commitments and DLog proof");
    }

    #[test]

    fn test_full_key_gen() {
        let (party_one_first_message, comm_witness, ec_key_pair_party1) =
            party_one::KeyGenFirstMsg::create_commitments_with_fixed_secret_share(ECScalar::from(
                &BigInt::sample(253),
            ));
        let (party_two_first_message, _ec_key_pair_party2) =
            party_two::KeyGenFirstMsg::create_with_fixed_secret_share(ECScalar::from(
                &BigInt::from(10_i32),
            ));
        let party_one_second_message = party_one::KeyGenSecondMsg::verify_and_decommit(
            comm_witness,
            &party_two_first_message.d_log_proof,
        )
        .expect("failed to verify and decommit");

        let _party_two_second_message =
            party_two::KeyGenSecondMsg::verify_commitments_and_dlog_proof(
                &party_one_first_message,
                &party_one_second_message,
            )
            .expect("failed to verify commitments and DLog proof");

        // init paillier keypair:
        let paillier_key_pair =
            party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(&ec_key_pair_party1);

        let party_one_private =
            party_one::Party1Private::set_private_key(&ec_key_pair_party1, &paillier_key_pair);

        let party_two_paillier = party_two::PaillierPublic {
            ek: paillier_key_pair.ek.clone(),
            encrypted_secret_share: paillier_key_pair.encrypted_share.clone(),
        };

        let correct_key_proof =
            party_one::PaillierKeyPair::generate_ni_proof_correct_key(&paillier_key_pair);

        correct_key_proof
            .verify(&party_two_paillier.ek)
            .expect("bad paillier key");

        // zk proof of correct paillier key

        // zk range proof
        let range_proof = party_one::PaillierKeyPair::generate_range_proof(
            &paillier_key_pair,
            &party_one_private,
        );
        party_two::PaillierPublic::verify_range_proof(&party_two_paillier, &range_proof)
            .expect("range proof error");
    }

    #[test]
    fn test_two_party_sign() {
        // assume party1 and party2 engaged with KeyGen in the past resulting in
        // party1 owning private share and paillier key-pair
        // party2 owning private share and paillier encryption of party1 share
        let (_party_one_private_share_gen, _comm_witness, mut ec_key_pair_party1) =
            party_one::KeyGenFirstMsg::create_commitments();
        let (party_two_private_share_gen, ec_key_pair_party2) = party_two::KeyGenFirstMsg::create();
        ec_key_pair_party1.add_scalar(&Secp256k1Scalar::new_random());
        let keypair =
            party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(&ec_key_pair_party1);

        // creating the ephemeral private shares:

        let (eph_party_two_first_message, eph_comm_witness, eph_ec_key_pair_party2) =
            party_two::EphKeyGenFirstMsg::create_commitments();
        let (eph_party_one_first_message, eph_ec_key_pair_party1) =
            party_one::EphKeyGenFirstMsg::create();
        let eph_party_two_second_message = party_two::EphKeyGenSecondMsg::verify_and_decommit(
            eph_comm_witness,
            &eph_party_one_first_message,
        )
        .expect("party1 DLog proof failed");

        let _eph_party_one_second_message: party_one::EphKeyGenSecondMsg =
            party_one::EphKeyGenSecondMsg::verify_commitments_and_dlog_proof(
                &eph_party_two_first_message,
                &eph_party_two_second_message,
            )
            .expect("failed to verify commitments and DLog proof");
        let party2_private = party_two::Party2Private::set_private_key(&ec_key_pair_party2);
        let message = BigInt::from(1234);
        let partial_sig = party_two::PartialSig::compute(
            &keypair.ek,
            &keypair.encrypted_share,
            &party2_private,
            &eph_ec_key_pair_party2,
            &eph_party_one_first_message.public_share,
            &message,
        );
        let party1_private =
            party_one::Party1Private::set_private_key(&ec_key_pair_party1, &keypair);

        let signature = party_one::Signature::compute(
            &party1_private,
            &partial_sig.c3,
            &eph_ec_key_pair_party1,
            &eph_party_two_second_message.comm_witness.public_share,
        );

        let pubkey =
            party_one::compute_pubkey(&party1_private, &party_two_private_share_gen.public_share);
        party_one::verify(&signature, &pubkey, &message).expect("Invalid signature")
    }

    fn test_scalar_sign() {
        // assume party1 and party2 engaged with KeyGen in the past resulting in
        // party1 owning private share and paillier key-pair
        // party2 owning private share and paillier encryption of party1 share
        let (_party_one_private_share_gen, _comm_witness, mut ec_key_pair_party1) =
            party_one::KeyGenFirstMsg::create_commitments();
        let (party_two_private_share_gen, ec_key_pair_party2) = party_two::KeyGenFirstMsg::create();
        let ec_key_pair_party1 = ec_key_pair_party1.add_scalar(&Secp256k1Scalar::new_random());
        let keypair =
            party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(&ec_key_pair_party1);

        // creating the ephemeral private shares:

        let (eph_party_two_first_message, eph_comm_witness, eph_ec_key_pair_party2) =
            party_two::EphKeyGenFirstMsg::create_commitments();
        let (eph_party_one_first_message, eph_ec_key_pair_party1) =
            party_one::EphKeyGenFirstMsg::create();
        let eph_party_two_second_message = party_two::EphKeyGenSecondMsg::verify_and_decommit(
            eph_comm_witness,
            &eph_party_one_first_message,
        )
        .expect("party1 DLog proof failed");

        let _eph_party_one_second_message: party_one::EphKeyGenSecondMsg =
            party_one::EphKeyGenSecondMsg::verify_commitments_and_dlog_proof(
                &eph_party_two_first_message,
                &eph_party_two_second_message,
            )
            .expect("failed to verify commitments and DLog proof");
        let party2_private = party_two::Party2Private::set_private_key(&ec_key_pair_party2);
        let message = BigInt::from(1234);
        let partial_sig = party_two::PartialSig::compute(
            &keypair.ek,
            &keypair.encrypted_share,
            &party2_private,
            &eph_ec_key_pair_party2,
            &eph_party_one_first_message.public_share,
            &message,
        );
        let party1_private =
            party_one::Party1Private::set_private_key(&ec_key_pair_party1, &keypair);

        let signature = party_one::Signature::compute(
            &party1_private,
            &partial_sig.c3,
            &eph_ec_key_pair_party1,
            &eph_party_two_second_message.comm_witness.public_share,
        );

        let pubkey =
            party_one::compute_pubkey(&party1_private, &party_two_private_share_gen.public_share);
        party_one::verify(&signature, &pubkey, &message).expect("Invalid signature")
    }

    fn test_derivation() {

        let (_party_one_private_share_gen, _comm_witness, mut ec_key_pair_party1) =
            party_one::KeyGenFirstMsg::create_commitments();
        let keypair = party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(
            &ec_key_pair_party1
        );
        let party1_private =
            party_one::Party1Private::set_private_key(&ec_key_pair_party1, &keypair);
        let (party_two_private_share_gen, ec_key_pair_party2) = party_two::KeyGenFirstMsg::create();
        // root pubkey 
        let pubkey =
            party_one::compute_pubkey(&party1_private, &party_two_private_share_gen.public_share);
        eprintln!("pubkey: {:?}", pubkey);  
        
        let aptos_pk = "0x8eeb041dc6382c044a14e1d1c1556a32cb665fe22a157c5739648d212e412b4e";
        let derivation_path = "ethereum-1";
        let derivation_string = format!(
            "aptos-mpc-recovery v0.1.0 epsilon derivation:{},{}",
            aptos_pk, derivation_path
        );
        let mut hasher = Sha3_256::new();
        hasher.update(derivation_string);
        let derivation_scalar_hex = hasher.finalize();
        // let bn_derivative_scalar_hex = BigInt::from_hex(derivation_scalar_hex);
        println!("derivation_scalar_hex: {:?}", derivation_scalar_hex);
    }
}
