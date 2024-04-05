use crate::error::Error;
use crate::state::{ExchangeContract, Token};
use crate::utils::{check_account_owner, perform_math_operation};
use anchor_lang::prelude::*;
use solana_program::program::{invoke};
use spl_token::instruction::transfer as spl_token_transfer;

pub fn process_exchange(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_a: Pubkey,
    token_b: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    // 获取交换合约账户
    let exchange_contract_account = next_account_info(account_iter)?;
    if exchange_contract_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 验证Token账户所有权
    let token_a_account = next_account_info(account_iter)?;
    if !check_account_owner(token_a_account, program_id)? {
        return Err(Error::AccountOwnershipMismatch.into());
    }

    let token_b_account = next_account_info(account_iter)?;
    if !check_account_owner(token_b_account, program_id)? {
        return Err(Error::AccountOwnershipMismatch.into());
    }

    // 解析交换合约数据
    let mut exchange_contract_data = ExchangeContract::try_from_slice(&exchange_contract_account.data.borrow())?;
    
    // 计算交易后的数量，考虑费用
    let token_a_amount = perform_math_operation(amount, exchange_contract_data.fee, true)?;
    let token_b_amount = perform_math_operation(amount, exchange_contract_data.fee, false)?;

    // 执行代币交换
    perform_token_transfer(
        program_id,
        token_a_account,
        token_b_account,
        token_a_amount,
    )?;
    perform_token_transfer(
        program_id,
        token_b_account,
        token_a_account,
        token_b_amount,
    )?;

    // 更新交换合约数据
    exchange_contract_data.exchange(token_a, token_b, amount)?;

    // 保存更新后的交换合约数据
    exchange_contract_account.data.borrow_mut().copy_from_slice(&exchange_contract_data.try_to_vec()?);

    Ok(())
}

// 实现代币转移逻辑
fn perform_token_transfer(
    token_program_id: &Pubkey,
    source_account: &AccountInfo,
    destination_account: &AccountInfo,
    amount: u64,
) -> ProgramResult {
    let transfer_instruction = spl_token_transfer(
        token_program_id,
        source_account.key,
        destination_account.key,
        amount,
        &[&source_account.key],
    )?;
    invoke(
        &transfer_instruction,
        &[
            source_account.clone(),
            destination_account.clone(),
        ],
    )
}