#![allow(non_snake_case)]
/*
centipede

Copyright 2018 by Kzen Networks

This file is part of centipede library
(https://github.com/KZen-networks/centipede)

Escrow-recovery is free software: you can redistribute
it and/or modify it under the terms of the GNU General Public
License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

@license GPL-3.0+ <https://github.com/KZen-networks/centipede/blob/master/LICENSE>
*/

use crate::curv::{FE,GE,BigInt};
use crate::curv::cryptographic_primitives::proofs::sigma_correct_homomorphic_elgamal_encryption_of_dlog::{HomoELGamalDlogProof,HomoElGamalDlogWitness,HomoElGamalDlogStatement};
use crate::curv::cryptographic_primitives::proofs::sigma_correct_homomorphic_elgamal_enc::{HomoELGamalProof,HomoElGamalWitness,HomoElGamalStatement};
use crate::curv::cryptographic_primitives::hashing::hash_sha512::HSha512;
use crate::curv::cryptographic_primitives::hashing::traits::*;
use crate::curv::arithmetic::traits::Converter;
use crate::bulletproofs::proofs::range_proof::{RangeProof,generate_random_point};
use super::segmentation::Msegmentation;
use crate::centipede::Errors::{self, ErrorProving};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Helgamal {
    pub D: GE,
    pub E: GE,
}

#[derive(Serialize, Deserialize)]
pub struct Helgamalsegmented {
    pub DE: Vec<Helgamal>,
}

#[derive(Serialize, Deserialize)]
pub struct Witness {
    pub x_vec: Vec<FE>,
    pub r_vec: Vec<FE>,
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub bulletproof: RangeProof,
    pub elgamal_enc: Vec<HomoELGamalProof>,
    pub elgamal_enc_dlog: HomoELGamalDlogProof,
}

impl Proof {
    pub fn prove(
        w: &Witness,
        c: &Helgamalsegmented,
        G: &GE,
        Y: &GE,
        segment_size: &usize,
    ) -> Proof {
        // bulletproofs:
        let num_segments = w.x_vec.len();
        // bit range
        let n = *segment_size;
        // batch size
        let m = num_segments;
        let nm = n * m;
        // some seed for generating g and h vectors
        let KZen: &[u8] = &[75, 90, 101, 110];
        let kzen_label = BigInt::from(KZen);

        let g_vec = (0..nm)
            .map(|i| {
                let kzen_label_i = BigInt::from(i as u32) + &kzen_label;
                let hash_i = HSha512::create_hash(&[&kzen_label_i]);
                generate_random_point(&Converter::to_vec(&hash_i))
            })
            .collect::<Vec<GE>>();

        // can run in parallel to g_vec:
        let h_vec = (0..nm)
            .map(|i| {
                let kzen_label_j = BigInt::from(n as u32) + BigInt::from(i as u32) + &kzen_label;
                let hash_j = HSha512::create_hash(&[&kzen_label_j]);
                generate_random_point(&Converter::to_vec(&hash_j))
            })
            .collect::<Vec<GE>>();

        let range_proof = RangeProof::prove(&g_vec, &h_vec, G, Y, w.x_vec.clone(), &w.r_vec, n);

        // proofs of correct elgamal:

        let elgamal_proofs = (0..num_segments)
            .map(|i| {
                let w = HomoElGamalWitness {
                    r: w.r_vec[i],
                    x: w.x_vec[i],
                };
                let delta = HomoElGamalStatement {
                    G: *G,
                    H: *G,
                    Y: *Y,
                    D: c.DE[i].D,
                    E: c.DE[i].E,
                };
                HomoELGamalProof::prove(&w, &delta)
            })
            .collect::<Vec<HomoELGamalProof>>();

        // proof of correct ElGamal DLog
        let D_vec: Vec<GE> = (0..num_segments).map(|i| c.DE[i].D).collect();
        let E_vec: Vec<GE> = (0..num_segments).map(|i| c.DE[i].E).collect();
        let sum_D = Msegmentation::assemble_ge(&D_vec, segment_size);
        let sum_E = Msegmentation::assemble_ge(&E_vec, segment_size);
        let sum_r = Msegmentation::assemble_fe(&w.r_vec, segment_size);
        let sum_x = Msegmentation::assemble_fe(&w.x_vec, segment_size);
        let Q = G * &sum_x;
        let delta = HomoElGamalDlogStatement {
            G: *G,
            Y: *Y,
            Q,
            D: sum_D,
            E: sum_E,
        };
        let w = HomoElGamalDlogWitness { r: sum_r, x: sum_x };
        let elgamal_dlog_proof = HomoELGamalDlogProof::prove(&w, &delta);

        Proof {
            bulletproof: range_proof,
            elgamal_enc: elgamal_proofs,
            elgamal_enc_dlog: elgamal_dlog_proof,
        }
    }

    pub fn verify(
        &self,
        c: &Helgamalsegmented,
        G: &GE,
        Y: &GE,
        Q: &GE,
        segment_size: &usize,
    ) -> Result<(), Errors> {
        // bulletproofs:
        let num_segments = self.elgamal_enc.len();
        // bit range
        let n = *segment_size;
        // batch size
        let m = num_segments;
        let nm = n * m;
        // some seed for generating g and h vectors
        let KZen: &[u8] = &[75, 90, 101, 110];
        let kzen_label = BigInt::from(KZen);

        let g_vec = (0..nm)
            .map(|i| {
                let kzen_label_i = BigInt::from(i as u32) + &kzen_label;
                let hash_i = HSha512::create_hash(&[&kzen_label_i]);
                generate_random_point(&Converter::to_vec(&hash_i))
            })
            .collect::<Vec<GE>>();

        // can run in parallel to g_vec:
        let h_vec = (0..nm)
            .map(|i| {
                let kzen_label_j = BigInt::from(n as u32) + BigInt::from(i as u32) + &kzen_label;
                let hash_j = HSha512::create_hash(&[&kzen_label_j]);
                generate_random_point(&Converter::to_vec(&hash_j))
            })
            .collect::<Vec<GE>>();

        let D_vec: Vec<GE> = (0..num_segments).map(|i| c.DE[i].D).collect();
        let bp_ver = self
            .bulletproof
            .verify(&g_vec, &h_vec, G, Y, &D_vec, *segment_size)
            .is_ok();

        let elgamal_proofs_ver = (0..num_segments)
            .map(|i| {
                let delta = HomoElGamalStatement {
                    G: *G,
                    H: *G,
                    Y: *Y,
                    D: c.DE[i].D,
                    E: c.DE[i].E,
                };
                self.elgamal_enc[i].verify(&delta).is_ok()
            })
            .collect::<Vec<bool>>();

        let E_vec: Vec<GE> = (0..num_segments).map(|i| c.DE[i].E).collect();
        let sum_D = Msegmentation::assemble_ge(&D_vec, segment_size);
        let sum_E = Msegmentation::assemble_ge(&E_vec, segment_size);

        let delta = HomoElGamalDlogStatement {
            G: *G,
            Y: *Y,
            Q: *Q,
            D: sum_D,
            E: sum_E,
        };

        let elgamal_dlog_proof_ver = self.elgamal_enc_dlog.verify(&delta).is_ok();
        if bp_ver && elgamal_dlog_proof_ver && elgamal_proofs_ver.iter().all(|&x| x) {
            Ok(())
        } else {
            Err(ErrorProving)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::centipede::juggling::proof_system::*;
    use crate::centipede::juggling::segmentation::Msegmentation;
    use crate::curv::elliptic::curves::traits::*;
    use crate::curv::{FE, GE};

    #[test]
    fn test_varifiable_encryption() {
        let segment_size = 8;
        let y: FE = ECScalar::new_random();
        let G: GE = ECPoint::generator();
        let Y = G * y;
        let x = FE::new_random();
        let Q = G * x;
        let (segments, encryptions) =
            Msegmentation::to_encrypted_segments(&x, &segment_size, 32, &Y, &G);
        let secret_new = Msegmentation::assemble_fe(&segments.x_vec, &segment_size);
        let secret_decrypted = Msegmentation::decrypt(&encryptions, &G, &y, &segment_size);

        assert_eq!(x, secret_new);
        assert_eq!(x, secret_decrypted.unwrap());

        let proof = Proof::prove(&segments, &encryptions, &G, &Y, &segment_size);
        let result = proof.verify(&encryptions, &G, &Y, &Q, &segment_size);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_varifiable_encryption_bad_Q() {
        let segment_size = 8;
        let y: FE = ECScalar::new_random();
        let G: GE = ECPoint::generator();
        let Y = G * y;
        let x = FE::new_random();
        let Q = G * x + G;
        let (segments, encryptions) =
            Msegmentation::to_encrypted_segments(&x, &segment_size, 32, &Y, &G);
        let secret_new = Msegmentation::assemble_fe(&segments.x_vec, &segment_size);
        let secret_decrypted = Msegmentation::decrypt(&encryptions, &G, &y, &segment_size);
        assert_eq!(x, secret_new);
        assert_eq!(x, secret_decrypted.unwrap());

        let proof = Proof::prove(&segments, &encryptions, &G, &Y, &segment_size);
        let result = proof.verify(&encryptions, &G, &Y, &Q, &segment_size);
        assert!(result.is_ok());
    }
}
