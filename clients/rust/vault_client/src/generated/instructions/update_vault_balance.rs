//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UpdateVaultBalance {
    pub config: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vault_token_account: solana_program::pubkey::Pubkey,

    pub vrt_mint: solana_program::pubkey::Pubkey,

    pub vault_fee_token_account: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl UpdateVaultBalance {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vrt_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_fee_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = UpdateVaultBalanceInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UpdateVaultBalanceInstructionData {
    discriminator: u8,
}

impl UpdateVaultBalanceInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 25 }
    }
}

impl Default for UpdateVaultBalanceInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `UpdateVaultBalance`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` vault
///   2. `[]` vault_token_account
///   3. `[writable]` vrt_mint
///   4. `[writable]` vault_fee_token_account
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct UpdateVaultBalanceBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vault_token_account: Option<solana_program::pubkey::Pubkey>,
    vrt_mint: Option<solana_program::pubkey::Pubkey>,
    vault_fee_token_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateVaultBalanceBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vault_token_account(
        &mut self,
        vault_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_token_account = Some(vault_token_account);
        self
    }
    #[inline(always)]
    pub fn vrt_mint(&mut self, vrt_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vrt_mint = Some(vrt_mint);
        self
    }
    #[inline(always)]
    pub fn vault_fee_token_account(
        &mut self,
        vault_fee_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_fee_token_account = Some(vault_fee_token_account);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = UpdateVaultBalance {
            config: self.config.expect("config is not set"),
            vault: self.vault.expect("vault is not set"),
            vault_token_account: self
                .vault_token_account
                .expect("vault_token_account is not set"),
            vrt_mint: self.vrt_mint.expect("vrt_mint is not set"),
            vault_fee_token_account: self
                .vault_fee_token_account
                .expect("vault_fee_token_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `update_vault_balance` CPI accounts.
pub struct UpdateVaultBalanceCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_vault_balance` CPI instruction.
pub struct UpdateVaultBalanceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> UpdateVaultBalanceCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateVaultBalanceCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            vault: accounts.vault,
            vault_token_account: accounts.vault_token_account,
            vrt_mint: accounts.vrt_mint,
            vault_fee_token_account: accounts.vault_fee_token_account,
            token_program: accounts.token_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vrt_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_fee_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = UpdateVaultBalanceInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.vault_token_account.clone());
        account_infos.push(self.vrt_mint.clone());
        account_infos.push(self.vault_fee_token_account.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `UpdateVaultBalance` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` vault
///   2. `[]` vault_token_account
///   3. `[writable]` vrt_mint
///   4. `[writable]` vault_fee_token_account
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct UpdateVaultBalanceCpiBuilder<'a, 'b> {
    instruction: Box<UpdateVaultBalanceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateVaultBalanceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateVaultBalanceCpiBuilderInstruction {
            __program: program,
            config: None,
            vault: None,
            vault_token_account: None,
            vrt_mint: None,
            vault_fee_token_account: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vault_token_account(
        &mut self,
        vault_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_token_account = Some(vault_token_account);
        self
    }
    #[inline(always)]
    pub fn vrt_mint(
        &mut self,
        vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vrt_mint = Some(vrt_mint);
        self
    }
    #[inline(always)]
    pub fn vault_fee_token_account(
        &mut self,
        vault_fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_fee_token_account = Some(vault_fee_token_account);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = UpdateVaultBalanceCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vault_token_account: self
                .instruction
                .vault_token_account
                .expect("vault_token_account is not set"),

            vrt_mint: self.instruction.vrt_mint.expect("vrt_mint is not set"),

            vault_fee_token_account: self
                .instruction
                .vault_fee_token_account
                .expect("vault_fee_token_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateVaultBalanceCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vrt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_fee_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
