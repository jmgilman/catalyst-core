use chain_addr::Discrimination;
use chain_core::property::Block;
use chain_core::property::Fragment as _;
use chain_impl_mockchain::fragment::Fragment;
use chain_impl_mockchain::fragment::FragmentId;
use chain_impl_mockchain::ledger::{Error as LedgerError, Ledger};
use chain_impl_mockchain::testing::TestGen;
use chain_impl_mockchain::transaction::Transaction;
use chain_impl_mockchain::vote::VotePlanStatus;
use jormungandr_lib::interfaces::Block0Configuration;
use jormungandr_lib::interfaces::RejectedFragmentInfo;
use jormungandr_lib::interfaces::{BlockDate, SettingsDto};
use jormungandr_lib::interfaces::{FragmentLog, FragmentOrigin, FragmentStatus};
use jormungandr_lib::interfaces::{FragmentRejectionReason, FragmentsProcessingSummary};
use jormungandr_lib::time::SystemTime;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;
use thor::BlockDateGenerator;

#[derive(Copy, Clone)]
pub enum FragmentRecieveStrategy {
    Reject,
    Accept,
    Pending,
    None,
}

pub struct LedgerState {
    fragment_strategy: FragmentRecieveStrategy,
    fragment_logs: Vec<FragmentLog>,
    received_fragments: Vec<Fragment>,
    ledger: Ledger,
    block0_configuration: Block0Configuration,
    block0_bin: Vec<u8>,
}

impl LedgerState {
    pub fn new(
        block0_configuration: Block0Configuration,
        block0_path: PathBuf,
    ) -> Result<Self, Error> {
        let block = block0_configuration.to_block();

        Ok(Self {
            fragment_strategy: FragmentRecieveStrategy::None,
            fragment_logs: Vec::new(),
            received_fragments: Vec::new(),
            block0_configuration,
            block0_bin: jortestkit::file::get_file_as_byte_vec(&block0_path),
            ledger: Ledger::new(block.id(), block.fragments())?,
        })
    }

    pub fn message(&mut self, fragment: Fragment) -> FragmentId {
        self.received_fragments.push(fragment.clone());
        let fragment_id = fragment.id();
        let parameters = self.ledger.get_ledger_parameters();
        let date = self.current_blockchain_age();
        let result = self
            .ledger
            .apply_fragment(&parameters, &fragment, date.into());
        let mut fragment_log = FragmentLog::new(fragment.id(), FragmentOrigin::Rest);
        self.set_fragment_status(&mut fragment_log, self.fragment_strategy, result);
        self.fragment_logs.push(fragment_log);
        fragment_id
    }

    pub fn batch_message(
        &mut self,
        fragments: Vec<Fragment>,
        fail_fast: bool,
    ) -> FragmentsProcessingSummary {
        let mut filtered_fragments = Vec::new();
        let mut rejected = Vec::new();
        let mut fragments = fragments.into_iter();

        for fragment in fragments.by_ref() {
            let id = fragment.id();
            if self
                .fragment_logs
                .iter()
                .any(|x| *x.fragment_id() == id.into())
            {
                rejected.push(RejectedFragmentInfo {
                    id,
                    reason: FragmentRejectionReason::FragmentAlreadyInLog,
                });
                continue;
            }

            if !is_fragment_valid(&fragment) {
                rejected.push(RejectedFragmentInfo {
                    id,
                    reason: FragmentRejectionReason::FragmentInvalid,
                });

                if fail_fast {
                    break;
                }

                continue;
            }
            filtered_fragments.push(fragment);
        }

        if fail_fast {
            for fragment in fragments.by_ref() {
                let id = fragment.id();
                rejected.push(RejectedFragmentInfo {
                    id,
                    reason: FragmentRejectionReason::PreviousFragmentInvalid,
                })
            }
        }

        let mut accepted = HashSet::new();

        for fragment in filtered_fragments {
            let id = fragment.id();
            self.message(fragment);
            accepted.insert(id);
        }

        let accepted = accepted.into_iter().collect();
        FragmentsProcessingSummary { accepted, rejected }
    }

    pub fn statuses(&self, ids: Vec<FragmentId>) -> HashMap<String, FragmentStatus> {
        self.fragment_logs
            .iter()
            .filter(|x| ids.contains(&(*x.fragment_id()).into_hash()))
            .map(|x| (x.fragment_id().to_string(), x.status().clone()))
            .collect()
    }

    pub fn set_fragment_strategy(&mut self, fragment_strategy: FragmentRecieveStrategy) {
        self.fragment_strategy = fragment_strategy;
    }

    pub fn accounts(&self) -> &chain_impl_mockchain::account::Ledger {
        self.ledger.accounts()
    }

    pub fn active_vote_plans(&self) -> Vec<VotePlanStatus> {
        self.ledger.active_vote_plans()
    }

    #[allow(dead_code)]
    pub fn accept_last_fragment(&mut self) {
        self.set_status_for_recent_fragment(FragmentRecieveStrategy::Accept);
    }

    #[allow(dead_code)]
    pub fn reject_last_fragment(&mut self) {
        self.set_status_for_recent_fragment(FragmentRecieveStrategy::Reject);
    }

    pub fn set_status_for_recent_fragment(&mut self, fragment_strategy: FragmentRecieveStrategy) {
        let block_date = self.current_blockchain_age();
        let fragment_log = self.fragment_logs.last_mut().unwrap();
        override_fragment_status(block_date, fragment_log, fragment_strategy);
    }

    pub fn set_fragment_status(
        &mut self,
        fragment_log: &mut FragmentLog,
        fragment_strategy: FragmentRecieveStrategy,
        result: Result<Ledger, LedgerError>,
    ) {
        if let FragmentRecieveStrategy::None = fragment_strategy {
            match result {
                Ok(ledger) => {
                    self.ledger = ledger;
                    fragment_log.modify(FragmentStatus::InABlock {
                        date: self.current_blockchain_age(),
                        block: TestGen::hash().into(),
                    })
                }
                Err(error) => fragment_log.modify(FragmentStatus::Rejected {
                    reason: format!("{:?}", error),
                }),
            };
        } else {
            override_fragment_status(
                self.current_blockchain_age(),
                fragment_log,
                fragment_strategy,
            );
        }
    }

    pub fn fragment_logs(&self) -> Vec<FragmentLog> {
        self.fragment_logs.clone()
    }

    pub fn received_fragments(&self) -> Vec<Fragment> {
        self.received_fragments.clone()
    }

    pub fn curr_slot_start_time(&self) -> SystemTime {
        let blockchain_configuration = &self.block0_configuration.blockchain_configuration;

        let slot_duration: u8 = blockchain_configuration.slot_duration.into();
        let slots_per_epoch: u32 = blockchain_configuration.slots_per_epoch.into();
        let last_block_date = self.current_blockchain_age();
        let secs = last_block_date.epoch() * slot_duration as u32 * slots_per_epoch
            + slot_duration as u32 * last_block_date.slot();
        let block0_time: std::time::SystemTime =
            jormungandr_lib::time::SystemTime::from(blockchain_configuration.block0_date).into();
        block0_time
            .checked_add(std::time::Duration::from_secs(secs.into()))
            .unwrap()
            .into()
    }

    pub fn current_blockchain_age(&self) -> BlockDate {
        let blockchain_configuration = &self.block0_configuration.blockchain_configuration;

        let slot_duration: u8 = blockchain_configuration.slot_duration.into();
        let slots_per_epoch: u32 = blockchain_configuration.slots_per_epoch.into();
        BlockDateGenerator::current_blockchain_age(
            SystemTime::from(blockchain_configuration.block0_date),
            slots_per_epoch,
            slot_duration.into(),
        )
        .into()
    }

    pub fn absolute_slot_count(&self) -> u32 {
        let settings = self.settings();
        let block_date = self.current_blockchain_age();
        block_date.epoch() * settings.slots_per_epoch + block_date.slot()
    }

    pub fn settings(&self) -> SettingsDto {
        let params = self.ledger.get_ledger_parameters();
        let slot_duration: u8 = self
            .block0_configuration
            .blockchain_configuration
            .slot_duration
            .into();

        SettingsDto {
            block0_hash: self.block0_configuration.to_block().id().to_string(),
            block0_time: SystemTime::from_secs_since_epoch(
                self.block0_configuration
                    .blockchain_configuration
                    .block0_date
                    .to_secs(),
            ),
            discrimination: Discrimination::Production,
            curr_slot_start_time: Some(self.curr_slot_start_time()),
            consensus_version: self
                .block0_configuration
                .blockchain_configuration
                .block0_consensus
                .to_string(),
            fees: self
                .block0_configuration
                .blockchain_configuration
                .linear_fees,
            block_content_max_size: self
                .block0_configuration
                .blockchain_configuration
                .block_content_max_size
                .into(),
            epoch_stability_depth: self
                .block0_configuration
                .blockchain_configuration
                .epoch_stability_depth
                .into(),
            slot_duration: slot_duration as u64,
            slots_per_epoch: self
                .block0_configuration
                .blockchain_configuration
                .slots_per_epoch
                .into(),
            treasury_tax: params.treasury_tax,
            reward_params: params.reward_params,
            tx_max_expiry_epochs: self.ledger.settings().transaction_max_expiry_epochs,
        }
    }

    pub fn block0_bin(&self) -> Vec<u8> {
        self.block0_bin.clone()
    }
}

pub fn override_fragment_status(
    block_date: BlockDate,
    fragment_log: &mut FragmentLog,
    fragment_strategy: FragmentRecieveStrategy,
) {
    match fragment_strategy {
        FragmentRecieveStrategy::Pending => {
            fragment_log.modify(FragmentStatus::Pending);
        }
        FragmentRecieveStrategy::Accept => {
            fragment_log.modify(FragmentStatus::InABlock {
                date: block_date,
                block: TestGen::hash().into(),
            });
        }
        FragmentRecieveStrategy::Reject => {
            fragment_log.modify(FragmentStatus::Rejected {
                reason: "Force reject by mock".to_string(),
            });
        }
        FragmentRecieveStrategy::None => {}
    }
}

fn is_fragment_valid(fragment: &Fragment) -> bool {
    match fragment {
        // never valid in the pool, only acceptable in genesis
        Fragment::Initial(_) => false,
        Fragment::OldUtxoDeclaration(_) => false,
        // general transactions stuff
        Fragment::Evm(_) => false,
        Fragment::MintToken(ref tx) => is_transaction_valid(tx),
        Fragment::Transaction(ref tx) => is_transaction_valid(tx),
        Fragment::StakeDelegation(ref tx) => is_transaction_valid(tx),
        Fragment::OwnerStakeDelegation(ref tx) => is_transaction_valid(tx),
        Fragment::PoolRegistration(ref tx) => is_transaction_valid(tx),
        Fragment::PoolRetirement(ref tx) => is_transaction_valid(tx),
        Fragment::PoolUpdate(ref tx) => is_transaction_valid(tx),
        // vote stuff
        Fragment::UpdateProposal(_) => false, // TODO: enable when ready
        Fragment::UpdateVote(_) => false,     // TODO: enable when ready
        Fragment::VotePlan(ref tx) => is_transaction_valid(tx),
        Fragment::VoteCast(ref tx) => is_transaction_valid(tx),
        Fragment::VoteTally(ref tx) => is_transaction_valid(tx),
        Fragment::EncryptedVoteTally(ref tx) => is_transaction_valid(tx),
    }
}

fn is_transaction_valid<E>(tx: &Transaction<E>) -> bool {
    tx.verify_possibly_balanced().is_ok()
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("ledger error")]
    Ledger(#[from] chain_impl_mockchain::ledger::Error),
}
