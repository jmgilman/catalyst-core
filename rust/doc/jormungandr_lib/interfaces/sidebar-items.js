window.SIDEBAR_ITEMS = {"constant":[["CERTIFICATE_HRP",""],["DEFAULT_ACTIVE_SLOT_COEFFICIENT","default active slot coefficient in milli `0.100`"],["DEFAULT_BLOCK_CONTENT_MAX_SIZE","the default value for block content max size"],["DEFAULT_EPOCH_STABILITY_DEPTH","the default value for epoch stability depth"],["DEFAULT_KES_SPEED_UPDATE","default KES Update speed (in seconds): 12hours"],["DEFAULT_NUMBER_OF_SLOTS_PER_EPOCH","default number of slots per epoch"],["DEFAULT_PROPOSAL_EXPIRATION","default proposal expiration in epochs"],["DEFAULT_SLOT_DURATION","default slot duration in seconds"],["MAXIMUM_ACTIVE_SLOT_COEFFICIENT","maximum active slot coefficient in milli `1.000`"],["MAXIMUM_KES_SPEED_UPDATE_IN_SECONDS","maximum KES Update speed (in seconds): about one year"],["MAXIMUM_NUMBER_OF_SLOTS_PER_EPOCH","maximum number of slots per epoch"],["MAXIMUM_SLOT_DURATION","maximum slot duration in seconds (here is it max of u8: 255)"],["MINIMUM_ACTIVE_SLOT_COEFFICIENT","minimum active slot coefficient in milli `0.001`"],["MINIMUM_KES_SPEED_UPDATE_IN_SECONDS","minimum KES Update speed (in seconds): 1minute"],["MINIMUM_NUMBER_OF_SLOTS_PER_EPOCH","minimum number of slots per epoch"],["MINIMUM_SLOT_DURATION","minimum slot duration in seconds"],["SIGNED_CERTIFICATE_HRP",""]],"enum":[["Block0ConfigurationError",""],["CertificateFromBech32Error",""],["CertificateFromStrError",""],["CertificateToBech32Error",""],["ConfigParam",""],["ConsensusVersionDef",""],["DiscriminationDef",""],["FeesGoTo","the settings for the fees to be redistributed to"],["FragmentOrigin","identify the source of a fragment"],["FragmentRejectionReason","This error is reserved for fragments that were rejected by the mempool at the time of sending them to mempool. If a fragment ended up being included to mempool, it will be listed in fragment logs and all further errors would be listed in fragment logs as well."],["FragmentStatus","status of the fragment within the blockchain or the pool"],["FromConfigParamError",""],["Initial",""],["LeadershipLogStatus","the status of a leadership log"],["LogOutput",""],["NodeState",""],["ParseRatioError",""],["PrivateTallyState",""],["RewardParams",""],["Tally",""],["TransactionInputType",""],["TryFromActiveSlotCoefficientError",""],["TryFromKesUpdateSpeedError",""],["TryFromNumberOfSlotsPerEpochError",""],["TryFromSlotDurationError",""],["VotePayload",""]],"fn":[["block0_configuration_documented_example",""],["config_params_documented_example",""],["load_persistent_fragments_logs_from_folder_path",""],["read_persistent_fragment_logs_from_file_path",""],["try_initial_fragment_from_message",""]],"mod":[["serde_base64_bytes",""],["serde_choices",""],["serde_committee_member_public_keys",""],["serde_external_proposal_id",""],["serde_proposals",""]],"struct":[["AccountIdentifier","An account identifier for the different kind of accounts (single or multi)."],["AccountState","represent the current state of an account in the ledger"],["AccountVotes",""],["ActiveSlotCoefficient",""],["Address","Address with the appropriate implementation for Serde API and Display/FromStr interfaces."],["Bft","hold the node’s bft secret setting"],["Block0Configuration",""],["Block0DateDef",""],["BlockContentMaxSize","the block content max size"],["BlockDate",""],["BlockchainConfiguration","Initial blockchain configuration for block0"],["Certificate",""],["CommitteeIdDef","remove serde encoding for the CommitteeId"],["ConfigParams",""],["ConsensusLeaderId",""],["Cors",""],["CorsOrigin",""],["Destination",""],["EpochRewardsInfo",""],["EpochStabilityDepth","epoch stability depth"],["EvmTransaction",""],["FileFragments",""],["FragmentDef",""],["FragmentLog","the log associated to a given fragment"],["FragmentLogDeserializeError",""],["FragmentsBatch","Submission of a batch of fragments to the node."],["FragmentsProcessingSummary","The summary of an attempt to add transactions to mempool for further processing."],["GenesisPraos","the genesis praos setting"],["InitialToken",""],["InitialUTxO",""],["JRpc",""],["KesUpdateSpeed",""],["LayersConfig",""],["LeadershipLog","provides information regarding events in the leadership schedule"],["LeadershipLogId","log identifier in the leadership log. Can be used to update back some."],["LegacyUTxO",""],["LinearFeeDef",""],["Log",""],["LogEntry",""],["LogMaxEntries",""],["Mempool",""],["NodeConfig",""],["NodeSecret",""],["NodeStats",""],["NodeStatsDto",""],["NumberOfSlotsPerEpoch",""],["OldAddress","OldAddress with the appropriate implementation for Serde API and Display/FromStr interfaces."],["P2p",""],["ParametersDef",""],["PeerRecord",""],["PeerStats",""],["PerCertificateFeeDef",""],["PerVoteCertificateFeeDef",""],["PersistentFragmentLog","Represents a persistent fragments log entry."],["PersistentLog",""],["Policy",""],["PoolMaxEntries",""],["PoolParticipationCapping",""],["PreferredListConfig",""],["ProposalExpiration",""],["Ratio","Ratio in the blockchain."],["RatioDef",""],["RejectedFragmentInfo","Information about a fragment rejected by the mempool. This is different from being rejected by the ledger during an attempt to apply this fragment."],["Rest",""],["RewardConstraints",""],["Rewards",""],["SettingsDto",""],["SignedCertificate",""],["SlotDuration",""],["Stake","Stake in the blockchain, always printed as absolute Lovelace"],["StakeDef",""],["StakeDistribution",""],["StakeDistributionDto",""],["StakePoolStats",""],["Subscription",""],["TallyResult",""],["TaxType",""],["TaxTypeDef",""],["TaxTypeSerde",""],["TimeEraDef",""],["Tls",""],["TokenIdentifier",""],["TopicsOfInterest",""],["TransactionInput",""],["TransactionOutput",""],["TransactionWitness","a transaction witness"],["TrustedPeer","Configuration item for a trusted peer."],["TryFromFeesGoToError",""],["UTxOInfo","the Unspent Transaction Output information."],["UTxOOutputInfo","The UTxO data about output without its location in blockchain"],["UpdateProposalDef",""],["UpdateProposalStateDef",""],["Value","Value in the blockchain, always printed as absolute Lovelace"],["ValueDef",""],["VotePlan",""],["VotePlanStatus",""],["VotePrivacy","Serializable wrapper for the payload type enum."],["VoteProposalStatus",""]],"type":[["NodeId","Identifier of a peer node."],["VotePlanId",""]]};