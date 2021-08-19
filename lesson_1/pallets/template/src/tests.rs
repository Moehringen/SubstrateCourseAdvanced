use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

//---------------------------------------- Question 1 ---------------------------------- 
//----CLAIM TESTING------

#[test]
fn create_claim_works(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(1, frame_system::Pallet::<Test>::block_number()
		));
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
        let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			TemplateModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	})
}


//----REVOKE TESTING------
#[test]
fn revoke_claim_work(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
        let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(
			TemplateModule::revoke_claim(Origin::signed(1),claim.clone())
		);
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(0,0)
		);
	})
}


#[test]
fn revoke_claim_failed_when_claim_does_not_exist(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
		assert_noop!(
			TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NoSuchProof
		);
	})
}


#[test]
fn revoke_claim_failed_when_not_the_owner(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
		let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			TemplateModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotProofOwner
		);
	})
}


//----TRANSFER TESTING------
#[test]
fn transfer_claim_work(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
        let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(
			TemplateModule::transfer_claim(Origin::signed(1),2,claim.clone())
		);
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(2, frame_system::Pallet::<Test>::block_number()
		));
		assert_ne!(
			Proofs::<Test>::get(&claim),
			(1, frame_system::Pallet::<Test>::block_number()
		));
	})
}


#[test]
fn transfer_failed_when_not_proof_onwer(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
		let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			TemplateModule::transfer_claim(Origin::signed(2), 1,claim.clone()),
			Error::<Test>::NotProofOwner
		);
	})
}

#[test]
fn transfer_failed_when_proof_not_exist(){
	new_test_ext().execute_with( || { 
		let claim: Vec<u8> = vec![0,1];
		let _= TemplateModule::create_claim(Origin::signed(1), claim.clone());
		let _= TemplateModule::revoke_claim(Origin::signed(1),claim.clone());
		assert_noop!(
			TemplateModule::transfer_claim(Origin::signed(1), 2,claim.clone()),
			Error::<Test>::NoSuchProof
		);
	})
}

//---------------------------------------- Question 2 ---------------------------------- 

#[test]
fn creart_claim_fail_when_claim_too_large(){
	new_test_ext().execute_with( || { 
		let claim = vec![0,1,2,3,4,5,6,7,8,9,10];
		assert_noop!(
			TemplateModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimLengthToolarge
		);
	})
}