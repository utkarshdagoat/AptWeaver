use std::fmt::format;

use two_party_ecdsa::{
    curv::elliptic::curves::traits::ECScalar,
    party_one::{self, Converter},
    party_two, BigInt, ECPoint, Secp256k1Point, Secp256k1Scalar,
};

use sha3::{Digest, Keccak256, Sha3_256};
mod aptos_client;
mod node;
mod party_1;
mod party_2;
fn main() {
    let (_party_one_private_share_gen, _comm_witness, mut ec_key_pair_party1) =
        party_one::KeyGenFirstMsg::create_commitments();
    let keypair =
        party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(&ec_key_pair_party1);
    let party1_private = party_one::Party1Private::set_private_key(&ec_key_pair_party1, &keypair);
    let (party_two_private_share_gen, ec_key_pair_party2) = party_two::KeyGenFirstMsg::create();

    // root pubkey
    let pubkey =
        party_one::compute_pubkey(&party1_private, &party_two_private_share_gen.public_share);

    println!("pubkey - x: {:?}", pubkey.x_coor().unwrap().to_hex());
    println!("pubkey - y: {:?}", pubkey.y_coor().unwrap().to_hex());

    let aptos_pk = "0x8eeb041dc6382c044a14e1d1c1556a32cb665fe22a157c5739648d212e412b4e";
    let derivation_path = "ethereum-1";
    let derivation_string = format!(
        "aptos-mpc-recovery v0.1.0 epsilon derivation:{},{}",
        aptos_pk, derivation_path
    );
    let mut hasher = Sha3_256::new();
    hasher.update(derivation_string);
    let derivation_scalar_hex = &hasher.finalize()[..];
    // let bn_derivative_scalar_hex = BigInt::from_hex(derivation_scalar_hex);
    let a = hex::encode(derivation_scalar_hex);
    println!("derivation_scalar_hex: {:?}", a);
    let bn_a = BigInt::from_hex(&a);
    let scaler: Secp256k1Scalar = ECScalar::from(&bn_a);
    let g = Secp256k1Point::generator();
    let scaler_g = g.scalar_mul(&scaler.get_element());
    let new_pk = pubkey.add_point(&scaler_g.get_element());
    let x_new_pk = new_pk.x_coor().unwrap();
    let y_new_pk = new_pk.y_coor().unwrap();
    let x_hex = format!("{:0>64}", x_new_pk.to_hex());
    let y_hex = format!("{:0>64}", y_new_pk.to_hex());

    let uncompress_pk = format!("04{}{}", x_hex, y_hex);
    println!("uncompress_pk: {:?}", uncompress_pk);
    let mut keccack = Keccak256::new();
    keccack.update(format!("0x{}", &uncompress_pk[2..]));
    let address_hash = &keccack.finalize()[..];
    let address_hex = hex::encode(address_hash);
    println!("address_hex: {:?}", address_hex);
    let etherum_addr = &address_hex[(address_hex.len() - 40)..];
    let address = format!("0x{}", etherum_addr);
    println!("address: {:?}", address);

    let (ec_key_pair_party1,new_sk) = ec_key_pair_party1.add_scalar(&scaler);

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
    let party1_private = party_one::Party1Private::set_private_key(&ec_key_pair_party1, &keypair);

    let signature = party_one::Signature::compute(
        &party1_private,
        &partial_sig.c3,
        &eph_ec_key_pair_party1,
        &eph_party_two_second_message.comm_witness.public_share,
    );
    println!("signature: r {:?}", signature.r.to_hex());
    println!("signature: s {:?}", signature.s.to_hex());

    let pubkey =
        party_one::compute_pubkey(&party1_private, &party_two_private_share_gen.public_share);
    party_one::verify(&signature, &pubkey, &message).expect("Invalid signature")
}
