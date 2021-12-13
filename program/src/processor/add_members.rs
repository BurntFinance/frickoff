use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    },
};

use crate::{
    processor::{CollectionData, CollectionError},
    utils::assert_owned_by,
};

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AddMembersArgs {}

pub fn add_members(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _: AddMembersArgs,
) -> ProgramResult {
    msg!("+ Processing AddMembers");
    let account_iter = &mut accounts.iter();

    // assert the function is called by the collection owner
    let collection_account = next_account_info(account_iter)?;
    assert_owned_by(collection_account, program_id)?;

    // iterate through remaining accounts for their pubkeys
    let mut new_members: Vec<Pubkey> = Vec::with_capacity(accounts.len() - 1);
    while let Ok(member) = next_account_info(account_iter) {
        new_members.push(*member.key);
    }

    // assert the collection can add members
    let mut collection = CollectionData::from_account_info(collection_account)?;

    if collection.expandable == false {
        return Err(CollectionError::NotExpandable.into());
    }

    if (collection.max_size as usize) < (collection.members.len() + new_members.len()) {
        return Err(CollectionError::CapacityExceeded.into());
    }

    // append the member to the collection
    collection.members.append(&mut new_members.clone());
    collection.serialize(&mut *collection_account.data.borrow_mut())?;

    Ok(())
}