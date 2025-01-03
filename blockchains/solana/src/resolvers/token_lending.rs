use crate::error::{Result, SolanaError};
use crate::resolvers::template_instruction;
use crate::solana_lib::solana_program::pubkey::Pubkey;
use crate::solana_lib::spl::token_lending::instruction::LendingInstruction;
use crate::solana_lib::spl::token_lending::state::ReserveConfig;
use serde_json::{json, Value};
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
static PROGRAM_NAME: &str = "TokenLending";

pub fn resolve(instruction: LendingInstruction, accounts: Vec<String>) -> Result<Value> {
    match instruction {
        LendingInstruction::InitLendingMarket {
            owner,
            quote_currency,
        } => init_lending_market(accounts, owner, quote_currency),
        LendingInstruction::SetLendingMarketOwner { new_owner } => {
            set_lending_market_owner(accounts, new_owner)
        }
        LendingInstruction::InitReserve {
            liquidity_amount,
            config,
        } => init_reserve(accounts, liquidity_amount, config),
        LendingInstruction::RefreshReserve => refresh_reserve(accounts),
        LendingInstruction::DepositReserveLiquidity { liquidity_amount } => {
            deposit_reserve_liquidity(accounts, liquidity_amount)
        }
        LendingInstruction::RedeemReserveCollateral { collateral_amount } => {
            redeem_reserve_collateral(accounts, collateral_amount)
        }
        LendingInstruction::InitObligation => init_obligation(accounts),
        LendingInstruction::RefreshObligation => refresh_obligation(accounts),
        LendingInstruction::DepositObligationCollateral { collateral_amount } => {
            deposit_obligation_collateral(accounts, collateral_amount)
        }
        LendingInstruction::WithdrawObligationCollateral { collateral_amount } => {
            withdraw_obligation_collateral(accounts, collateral_amount)
        }
        LendingInstruction::BorrowObligationLiquidity { liquidity_amount } => {
            borrow_obligation_liquidity(accounts, liquidity_amount)
        }
        LendingInstruction::RepayObligationLiquidity { liquidity_amount } => {
            repay_obligation_liquidity(accounts, liquidity_amount)
        }
        LendingInstruction::LiquidateObligation { liquidity_amount } => {
            liquidate_obligation(accounts, liquidity_amount)
        }
        LendingInstruction::FlashLoan { amount } => flash_loan(accounts, amount),
    }
}

fn init_lending_market(
    accounts: Vec<String>,
    owner: Pubkey,
    quote_currency: [u8; 32],
) -> Result<Value> {
    let method_name = "InitLendingMarket";
    let lending_market_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name,
    )))?;
    let sysvar_rent = accounts.get(1).ok_or(SolanaError::AccountNotFound(format!(
        "{}.account",
        method_name
    )))?;
    let token_program_id = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;
    let oracle_program_id = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.oracle_program_id",
        method_name
    )))?;
    let owner = owner.to_string();
    let quote_currency = core::str::from_utf8(&quote_currency)
        .map_err(|_| SolanaError::InvalidData(format!("{}.quote_currency", method_name)))?;
    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "lending_market_account": lending_market_account,
            "sysvar_rent": sysvar_rent,
            "token_program_id": token_program_id,
            "oracle_program_id": oracle_program_id,
            "owner": owner,
            "quote_currency": quote_currency,
        }),
        json!({
            "lending_market_account": lending_market_account,
            "quote_currency": quote_currency,
        }),
    ))
}

fn set_lending_market_owner(accounts: Vec<String>, new_owner: Pubkey) -> Result<Value> {
    let method_name = "SetLendingMarketOwner";
    let lending_market_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name,
    )))?;
    let current_owner = accounts.get(1).ok_or(SolanaError::AccountNotFound(format!(
        "{}.current_owner",
        method_name
    )))?;
    let new_owner = new_owner.to_string();
    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "lending_market_account": lending_market_account,
            "current_owner": current_owner,
            "new_owner": new_owner,
        }),
        json!({
            "lending_market_account": lending_market_account,
            "current_owner": current_owner,
            "new_owner": new_owner,
        }),
    ))
}

fn init_reserve(
    accounts: Vec<String>,
    liquidity_amount: u64,
    config: ReserveConfig,
) -> Result<Value> {
    let method_name = "SetLendingMarketOwner";
    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_collateral_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_collateral_account", method_name),
    ))?;
    let reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_account",
        method_name
    )))?;
    let reserve_liquidity_mint = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_liquidity_mint",
        method_name
    )))?;
    let reserve_liquidity_supply_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_supply_account", method_name),
    ))?;
    let reserve_liquidity_fee_receiver = accounts.get(5).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_fee_receiver", method_name),
    ))?;
    let reserve_collateral_mint = accounts.get(6).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_collateral_mint",
        method_name
    )))?;
    let reserve_collateral_supply_pubkey = accounts.get(7).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_collateral_supply_pubkey", method_name),
    ))?;
    let pyth_product_account = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.pyth_product_account",
        method_name
    )))?;
    let pyth_price_account = accounts.get(9).ok_or(SolanaError::AccountNotFound(format!(
        "{}.pyth_price_account",
        method_name
    )))?;
    let lending_market_account = accounts
        .get(10)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.lending_market_account",
            method_name
        )))?;
    let lending_market_authority_pubkey =
        accounts
            .get(11)
            .ok_or(SolanaError::AccountNotFound(format!(
                "{}.lending_market_authority_pubkey",
                method_name
            )))?;
    let lending_market_owner = accounts
        .get(12)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.lending_market_owner",
            method_name
        )))?;
    let user_transfer_authority_pubkey =
        accounts
            .get(13)
            .ok_or(SolanaError::AccountNotFound(format!(
                "{}.user_transfer_authority_pubkey",
                method_name
            )))?;
    let sysvar_clock = accounts
        .get(14)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.sysvar_clock",
            method_name
        )))?;
    let sysvar_rent = accounts
        .get(15)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.sysvar_rent",
            method_name
        )))?;
    let token_program_id = accounts
        .get(16)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.token_program_id",
            method_name
        )))?;
    let liquidity_amount = liquidity_amount.to_string();
    let reserve_config = json!({
        "optimal_utilization_rate": config.optimal_utilization_rate,
        "loan_to_value_ratio": config.loan_to_value_ratio,
        "liquidation_bonus": config.liquidation_bonus,
        "liquidation_threshold": config.liquidation_threshold,
        "min_borrow_rate": config.min_borrow_rate,
        "optimal_borrow_rate": config.optimal_borrow_rate,
        "max_borrow_rate": config.max_borrow_rate,
        "fees": {
            "borrow_fee_wad": config.fees.borrow_fee_wad.to_string(),
            "flash_loan_fee_wad": config.fees.flash_loan_fee_wad.to_string(),
            "host_fee_percentage": config.fees.host_fee_percentage.to_string(),
        },
    });
    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_collateral_account": destination_collateral_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_mint": reserve_liquidity_mint,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_liquidity_fee_receiver": reserve_liquidity_fee_receiver,
            "reserve_collateral_mint": reserve_collateral_mint,
            "reserve_collateral_supply_pubkey": reserve_collateral_supply_pubkey,
            "pyth_product_account": pyth_product_account,
            "pyth_price_account": pyth_price_account,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "lending_market_owner": lending_market_owner,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "sysvar_rent": sysvar_rent,
            "token_program_id": token_program_id,
            "liquidity_amount": liquidity_amount,
            "reserve_config": reserve_config,
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_collateral_account": destination_collateral_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_mint": reserve_liquidity_mint,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_liquidity_fee_receiver": reserve_liquidity_fee_receiver,
            "reserve_collateral_mint": reserve_collateral_mint,
            "reserve_collateral_supply_pubkey": reserve_collateral_supply_pubkey,
            "pyth_product_account": pyth_product_account,
            "pyth_price_account": pyth_price_account,
            "lending_market_account": lending_market_account,
            "lending_market_owner": lending_market_owner,
            "liquidity_amount": liquidity_amount,
            "reserve_config": reserve_config,
        }),
    ))
}

fn refresh_reserve(accounts: Vec<String>) -> Result<Value> {
    let method_name = "RefreshReserve";
    let reserve_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_account",
        method_name,
    )))?;
    let reserve_liquidity_oracle_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_oracle_account", method_name),
    ))?;
    let sysvar_clock = accounts.get(1).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "reserve_account": reserve_account,
            "reserve_liquidity_oracle_account": reserve_liquidity_oracle_account,
            "sysvar_clock": sysvar_clock,
        }),
        json!({
            "reserve_account": reserve_account,
            "reserve_liquidity_oracle_account": reserve_liquidity_oracle_account,
        }),
    ))
}

fn deposit_reserve_liquidity(accounts: Vec<String>, liquidity_amount: u64) -> Result<Value> {
    let method_name = "DepositReserveLiquidity";
    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_collateral_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_collateral_account", method_name),
    ))?;
    let reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_account",
        method_name
    )))?;
    let reserve_liquidity_supply_account = accounts.get(3).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_supply_account", method_name),
    ))?;
    let reserve_collateral_mint = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_collateral_mint",
        method_name
    )))?;
    let lending_market_account = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(6).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let user_transfer_authority_pubkey = accounts.get(7).ok_or(SolanaError::AccountNotFound(
        format!("{}.user_transfer_authority_pubkey", method_name),
    ))?;
    let sysvar_clock = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(9).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;
    let liquidity_amount = liquidity_amount.to_string();
    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_collateral_account": destination_collateral_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_collateral_mint": reserve_collateral_mint,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "liquidity_amount": liquidity_amount,
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_collateral_account": destination_collateral_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_collateral_mint": reserve_collateral_mint,
            "lending_market_account": lending_market_account,
            "liquidity_amount": liquidity_amount,
        }),
    ))
}

fn redeem_reserve_collateral(accounts: Vec<String>, collateral_amount: u64) -> Result<Value> {
    let method_name = "RedeemReserveCollateral";
    let source_collateral_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(
        format!("{}.source_collateral_account", method_name,),
    ))?;
    let destination_liquidity_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_liquidity_account", method_name),
    ))?;
    let reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_account",
        method_name
    )))?;
    let reserve_collateral_mint = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_collateral_mint",
        method_name
    )))?;
    let reserve_liquidity_supply_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_supply_account", method_name),
    ))?;
    let lending_market_account = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(6).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let user_transfer_authority_pubkey = accounts.get(7).ok_or(SolanaError::AccountNotFound(
        format!("{}.user_transfer_authority_pubkey", method_name),
    ))?;
    let sysvar_clock = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(9).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_liquidity_account": destination_liquidity_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_collateral_mint": reserve_collateral_mint,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "collateral_amount": collateral_amount.to_string(),
        }),
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_liquidity_account": destination_liquidity_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_supply_account": reserve_liquidity_supply_account,
            "reserve_collateral_mint": reserve_collateral_mint,
            "lending_market_account": lending_market_account,
            "collateral_amount": collateral_amount.to_string(),
        }),
    ))
}

fn init_obligation(accounts: Vec<String>) -> Result<Value> {
    let method_name = "InitObligation";
    let obligation_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name,
    )))?;
    let lending_market_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let obligation_owner = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_owner",
        method_name
    )))?;
    let sysvar_clock = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let sysvar_rent = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_rent",
        method_name
    )))?;
    let token_program_id = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
            "sysvar_clock": sysvar_clock,
            "sysvar_rent": sysvar_rent,
            "token_program_id": token_program_id,
        }),
        json!({
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
        }),
    ))
}

fn refresh_obligation(accounts: Vec<String>) -> Result<Value> {
    let method_name = "RefreshObligation";
    let obligation_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name,
    )))?;
    let sysvar_clock = accounts.get(1).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let keys = &accounts[2..];

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "obligation_account": obligation_account,
            "sysvar_clock": sysvar_clock,
            "keys": keys,
        }),
        json!({
            "obligation_account": obligation_account,
        }),
    ))
}

fn deposit_obligation_collateral(accounts: Vec<String>, collateral_amount: u64) -> Result<Value> {
    let method_name = "DepositObligationCollateral";
    let source_collateral_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(
        format!("{}.source_collateral_account", method_name,),
    ))?;
    let destination_collateral_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_collateral_account", method_name),
    ))?;
    let deposit_reserve_pubkey = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.deposit_reserve_pubkey",
        method_name
    )))?;
    let obligation_account = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name
    )))?;
    let lending_market_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let obligation_owner = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_owner",
        method_name
    )))?;
    let user_transfer_authority_pubkey = accounts.get(6).ok_or(SolanaError::AccountNotFound(
        format!("{}.user_transfer_authority_pubkey", method_name),
    ))?;
    let sysvar_clock = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_collateral_account": destination_collateral_account,
            "deposit_reserve_pubkey": deposit_reserve_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "collateral_amount": collateral_amount.to_string(),
        }),
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_collateral_account": destination_collateral_account,
            "deposit_reserve_pubkey": deposit_reserve_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
            "collateral_amount": collateral_amount.to_string(),
        }),
    ))
}

fn withdraw_obligation_collateral(accounts: Vec<String>, collateral_amount: u64) -> Result<Value> {
    let method_name = "WithdrawObligationCollateral";
    let source_collateral_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(
        format!("{}.source_collateral_account", method_name,),
    ))?;
    let destination_collateral_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_collateral_account", method_name),
    ))?;
    let withdraw_reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.deposit_reserve_pubkey",
        method_name
    )))?;
    let obligation_account = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name
    )))?;
    let lending_market_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(5).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let obligation_owner = accounts.get(6).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_owner",
        method_name
    )))?;
    let sysvar_clock = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_collateral_account": destination_collateral_account,
            "withdraw_reserve_account": withdraw_reserve_account,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "obligation_owner": obligation_owner,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "collateral_amount": collateral_amount.to_string(),
        }),
        json!({
            "source_collateral_account": source_collateral_account,
            "destination_collateral_account": destination_collateral_account,
            "withdraw_reserve_account": withdraw_reserve_account,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
            "collateral_amount": collateral_amount.to_string(),
        }),
    ))
}

fn borrow_obligation_liquidity(accounts: Vec<String>, liquidity_amount: u64) -> Result<Value> {
    let method_name = "BorrowObligationLiquidity";
    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_liquidity_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_liquidity_account", method_name),
    ))?;
    let borrow_reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.borrow_reserve_account",
        method_name
    )))?;
    let borrow_reserve_liquidity_fee_receiver_pubkey =
        accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
            "{}.borrow_reserve_liquidity_fee_receiver_pubkey",
            method_name
        )))?;
    let obligation_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name
    )))?;
    let lending_market_account = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(6).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let obligation_owner = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_owner",
        method_name
    )))?;
    let sysvar_clock = accounts.get(9).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(8).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;
    let host_fee_receiver = accounts.get(10);

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "borrow_reserve_account": borrow_reserve_account,
            "borrow_reserve_liquidity_fee_receiver_pubkey": borrow_reserve_liquidity_fee_receiver_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "obligation_owner": obligation_owner,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "host_fee_receiver": host_fee_receiver,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "borrow_reserve_account": borrow_reserve_account,
            "borrow_reserve_liquidity_fee_receiver_pubkey": borrow_reserve_liquidity_fee_receiver_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "obligation_owner": obligation_owner,
            "host_fee_receiver": host_fee_receiver,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
    ))
}

fn repay_obligation_liquidity(accounts: Vec<String>, liquidity_amount: u64) -> Result<Value> {
    let method_name = "RepayObligationLiquidity";

    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_liquidity_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_liquidity_account", method_name),
    ))?;
    let repay_reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.repay_reserve_account",
        method_name
    )))?;
    let obligation_account = accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name
    )))?;
    let lending_market_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let user_transfer_authority_pubkey = accounts.get(5).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let sysvar_clock = accounts.get(6).ok_or(SolanaError::AccountNotFound(format!(
        "{}.sysvar_clock",
        method_name
    )))?;
    let token_program_id = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "repay_reserve_account": repay_reserve_account,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "repay_reserve_account": repay_reserve_account,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
    ))
}

fn liquidate_obligation(accounts: Vec<String>, liquidity_amount: u64) -> Result<Value> {
    let method_name = "LiquidateObligation";
    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_collateral_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_collateral_account", method_name),
    ))?;
    let repay_reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.repay_reserve_account",
        method_name
    )))?;
    let repay_reserve_liquidity_supply_pubkey =
        accounts.get(3).ok_or(SolanaError::AccountNotFound(format!(
            "{}.repay_reserve_liquidity_supply_pubkey",
            method_name
        )))?;
    let withdraw_reserve_account = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.withdraw_reserve_account",
        method_name
    )))?;
    let withdraw_reserve_collateral_supply_pubkey =
        accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
            "{}.withdraw_reserve_collateral_supply_pubkey",
            method_name
        )))?;
    let obligation_account = accounts.get(6).ok_or(SolanaError::AccountNotFound(format!(
        "{}.obligation_account",
        method_name
    )))?;
    let lending_market_account = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(8).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let user_transfer_authority_pubkey = accounts.get(9).ok_or(SolanaError::AccountNotFound(
        format!("{}.user_transfer_authority_pubkey", method_name),
    ))?;
    let sysvar_clock = accounts
        .get(10)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.sysvar_clock",
            method_name
        )))?;
    let token_program_id = accounts
        .get(11)
        .ok_or(SolanaError::AccountNotFound(format!(
            "{}.token_program_id",
            method_name
        )))?;

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_collateral_account,
            "repay_reserve_account": repay_reserve_account,
            "repay_reserve_liquidity_supply_pubkey": repay_reserve_liquidity_supply_pubkey,
            "withdraw_reserve_account": withdraw_reserve_account,
            "withdraw_reserve_collateral_supply_pubkey": withdraw_reserve_collateral_supply_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "user_transfer_authority_pubkey": user_transfer_authority_pubkey,
            "sysvar_clock": sysvar_clock,
            "token_program_id": token_program_id,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_collateral_account,
            "repay_reserve_account": repay_reserve_account,
            "repay_reserve_liquidity_supply_pubkey": repay_reserve_liquidity_supply_pubkey,
            "withdraw_reserve_account": withdraw_reserve_account,
            "withdraw_reserve_collateral_supply_pubkey": withdraw_reserve_collateral_supply_pubkey,
            "obligation_account": obligation_account,
            "lending_market_account": lending_market_account,
            "liquidity_amount": liquidity_amount.to_string(),
        }),
    ))
}

fn flash_loan(accounts: Vec<String>, amount: u64) -> Result<Value> {
    let method_name = "FlashLoan";
    let source_liquidity_account = accounts.get(0).ok_or(SolanaError::AccountNotFound(format!(
        "{}.source_liquidity_account",
        method_name,
    )))?;
    let destination_liquidity_account = accounts.get(1).ok_or(SolanaError::AccountNotFound(
        format!("{}.destination_liquidity_account", method_name),
    ))?;
    let reserve_account = accounts.get(2).ok_or(SolanaError::AccountNotFound(format!(
        "{}.reserve_account",
        method_name
    )))?;
    let reserve_liquidity_fee_receiver = accounts.get(3).ok_or(SolanaError::AccountNotFound(
        format!("{}.reserve_liquidity_fee_receiver", method_name),
    ))?;
    let host_fee_receiver = accounts.get(4).ok_or(SolanaError::AccountNotFound(format!(
        "{}.host_fee_receiver",
        method_name
    )))?;
    let lending_market_account = accounts.get(5).ok_or(SolanaError::AccountNotFound(format!(
        "{}.lending_market_account",
        method_name
    )))?;
    let lending_market_authority_pubkey = accounts.get(6).ok_or(SolanaError::AccountNotFound(
        format!("{}.lending_market_authority_pubkey", method_name),
    ))?;
    let token_program_id = accounts.get(7).ok_or(SolanaError::AccountNotFound(format!(
        "{}.token_program_id",
        method_name
    )))?;
    let flash_loan_receiver_program_id = accounts.get(8).ok_or(SolanaError::AccountNotFound(
        format!("{}.flash_loan_receiver_program_id", method_name),
    ))?;
    let flash_loan_receiver_program_accounts = &accounts[9..];

    Ok(template_instruction(
        PROGRAM_NAME,
        method_name,
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_fee_receiver": reserve_liquidity_fee_receiver,
            "host_fee_receiver": host_fee_receiver,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "token_program_id": token_program_id,
            "flash_loan_receiver_program_id": flash_loan_receiver_program_id,
            "flash_loan_receiver_program_accounts": flash_loan_receiver_program_accounts,
            "amount": amount.to_string(),
        }),
        json!({
            "source_liquidity_account": source_liquidity_account,
            "destination_liquidity_account": destination_liquidity_account,
            "reserve_account": reserve_account,
            "reserve_liquidity_fee_receiver": reserve_liquidity_fee_receiver,
            "host_fee_receiver": host_fee_receiver,
            "lending_market_account": lending_market_account,
            "lending_market_authority_pubkey": lending_market_authority_pubkey,
            "token_program_id": token_program_id,
            "flash_loan_receiver_program_id": flash_loan_receiver_program_id,
            "flash_loan_receiver_program_accounts": flash_loan_receiver_program_accounts,
            "amount": amount.to_string(),
        }),
    ))
}
