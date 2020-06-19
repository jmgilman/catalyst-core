use crate::{ledger::governance::GovernanceAcceptanceCriteria, value::Value};
use chain_core::mempack::{ReadBuf, ReadError, Readable};
use imhamt::Hamt;
use std::collections::hash_map::DefaultHasher;
use typed_bytes::ByteBuilder;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TreasuryGovernanceAction {
    NoOp,
    TransferToRewards { value: Value },
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TreasuryGovernanceActionType {
    NoOp,
    TransferToRewards,
}

#[derive(Default, Clone, Eq, PartialEq)]
pub struct TreasuryGovernance {
    acceptance_criteria_per_action:
        Hamt<DefaultHasher, TreasuryGovernanceActionType, GovernanceAcceptanceCriteria>,

    default_acceptance_criteria: GovernanceAcceptanceCriteria,
}

impl TreasuryGovernanceAction {
    pub fn to_type(&self) -> TreasuryGovernanceActionType {
        match self {
            Self::NoOp => TreasuryGovernanceActionType::NoOp,
            Self::TransferToRewards { .. } => TreasuryGovernanceActionType::TransferToRewards,
        }
    }

    pub(crate) fn serialize_in(&self, bb: ByteBuilder<Self>) -> ByteBuilder<Self> {
        match self {
            Self::NoOp => bb.u8(0),
            Self::TransferToRewards { value } => bb.u8(1).u64(value.0),
        }
    }
}

impl TreasuryGovernance {
    pub fn new() -> Self {
        Self::default()
    }

    /// set the new default acceptance criteria
    ///
    /// this function does not do any allocation/drop and returns the previous
    /// default value.
    pub fn set_default_acceptance_criteria(
        &mut self,
        new: GovernanceAcceptanceCriteria,
    ) -> GovernanceAcceptanceCriteria {
        std::mem::replace(&mut self.default_acceptance_criteria, new)
    }

    /// get the default acceptance criteria
    ///
    /// This is the default criteria that will be used for any
    /// treasury governance action if a specific one is not set
    /// for that given governance action.
    pub fn default_acceptance_criteria(&self) -> &GovernanceAcceptanceCriteria {
        &self.default_acceptance_criteria
    }

    pub fn set_acceptance_criteria(
        &mut self,
        action: TreasuryGovernanceActionType,
        criteria: GovernanceAcceptanceCriteria,
    ) {
        self.acceptance_criteria_per_action = self
            .acceptance_criteria_per_action
            .insert_or_update_simple(action, criteria.clone(), |_| Some(criteria));
    }

    pub fn acceptance_criteria_for(
        &self,
        action: TreasuryGovernanceActionType,
    ) -> &GovernanceAcceptanceCriteria {
        self.acceptance_criteria_per_action
            .lookup(&action)
            .unwrap_or_else(|| self.default_acceptance_criteria())
    }
}

/* Ser/De ******************************************************************* */

impl Readable for TreasuryGovernanceAction {
    fn read<'a>(buf: &mut ReadBuf<'a>) -> Result<Self, ReadError> {
        match buf.get_u8()? {
            0 => Ok(Self::NoOp),
            1 => {
                let value = Value::read(buf)?;
                Ok(Self::TransferToRewards { value })
            }
            t => Err(ReadError::UnknownTag(t as u32)),
        }
    }
}
