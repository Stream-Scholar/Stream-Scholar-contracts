// Comprehensive tests for ScholarError enum variants
// These tests demonstrate the new structured error handling

#[cfg(test)]
mod scholar_error_tests {
    use super::*;
    use crate::ScholarError;

    #[test]
    fn test_scholarship_paused_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Create a scholarship for the student
        client.deposit_scholarship(&admin, &student, &token_address.address(), &1000);

        // Pause the scholarship
        client.pause_scholarship(&admin, &student);

        // Try to withdraw - should return ScholarshipPaused error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "withdraw_scholarship"),
            (student, 100),
        );
        assert_eq!(result.result, Err(ScholarError::ScholarshipPaused));
    }

    #[test]
    fn test_insufficient_balance_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Create a scholarship with only 100 tokens
        client.deposit_scholarship(&admin, &student, &token_address.address(), &100);

        // Try to withdraw more than available - should return InsufficientBalance error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "withdraw_scholarship"),
            (student, 200),
        );
        assert_eq!(result.result, Err(ScholarError::InsufficientBalance));
    }

    #[test]
    fn test_amount_exceeds_unlocked_balance_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Create a scholarship with locked funds
        client.deposit_scholarship(&admin, &student, &token_address.address(), &1000);

        // Try to withdraw more than unlocked balance - should return AmountExceedsAvailableBalance error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "withdraw_scholarship"),
            (student, 1000),
        );
        assert_eq!(result.result, Err(ScholarError::AmountExceedsAvailableBalance));
    }

    #[test]
    fn test_university_security_hold_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let university = Address::generate(&env);
        let university_admin = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Register university admin
        client.register_university_admin(&admin, &university, &university_admin);

        // Associate student with university
        client.associate_student_with_university(&university_admin, &university, &student);

        // Create scholarship
        client.deposit_scholarship(&admin, &student, &token_address.address(), &1000);

        // Trigger security hold
        client.trigger_security_hold(&university_admin, &university, &Symbol::new(&env, "review"));

        // Try to withdraw - should return UniversitySecurityHoldActive error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "withdraw_scholarship"),
            (student, 100),
        );
        assert_eq!(result.result, Err(ScholarError::UniversitySecurityHoldActive));
    }

    #[test]
    fn test_final_release_locked_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Create a scholarship and use most of it (leaving less than 10%)
        client.deposit_scholarship(&admin, &student, &token_address.address(), &1000);
        
        // Withdraw most of the funds to trigger final release lock
        client.withdraw_scholarship(&student, &900);

        // Try to withdraw remaining funds - should return FinalReleaseLocked error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "withdraw_scholarship"),
            (student, 50),
        );
        assert_eq!(result.result, Err(ScholarError::FinalReleaseLocked));
    }

    #[test]
    fn test_vote_already_initiated_error() {
        let env = Env::default();
        env.mock_all_auths();

        let student = Address::generate(&env);
        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_address = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::StellarAssetClient::new(&env, &token_address.address());
        token_client.mint(&admin, &10000);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Create a scholarship and use most of it
        client.deposit_scholarship(&admin, &student, &token_address.address(), &1000);
        client.withdraw_scholarship(&student, &900);

        // Initiate final release vote
        client.initiate_final_release_vote(&student);

        // Try to initiate again - should return VoteAlreadyInitiated error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "initiate_final_release_vote"),
            student,
        );
        assert_eq!(result.result, Err(ScholarError::VoteAlreadyInitiated));
    }

    #[test]
    fn test_not_platform_admin_error() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let unauthorized = Address::generate(&env);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Try to set tax rate with unauthorized user - should return NotPlatformAdmin error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "set_tax_rate"),
            (unauthorized, 500u32),
        );
        assert_eq!(result.result, Err(ScholarError::NotPlatformAdmin));
    }

    #[test]
    fn test_tax_rate_too_high_error() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);

        let contract_id = env.register(ScholarContract, ());
        let client = ScholarContractClient::new(&env, &contract_id);

        client.init(&10, &3600, &10, &100, &60);
        client.set_admin(&admin);

        // Try to set tax rate above 100% - should return TaxRateTooHigh error
        let result = env.try_invoke_contract::<_, ScholarError>(
            &contract_id,
            &Symbol::new(&env, "set_tax_rate"),
            (admin, 15000u32), // 150% tax rate
        );
        assert_eq!(result.result, Err(ScholarError::TaxRateTooHigh));
    }

    #[test]
    fn test_error_code_ranges() {
        // Verify that error codes are in the correct ranges
        assert!(ScholarError::Unauthorized as u32 >= 0x1000 && ScholarError::Unauthorized as u32 < 0x1100);
        assert!(ScholarError::ScholarshipPaused as u32 >= 0x1100 && ScholarError::ScholarshipPaused as u32 < 0x1200);
        assert!(ScholarError::InsufficientBalance as u32 >= 0x1200 && ScholarError::InsufficientBalance as u32 < 0x1300);
        assert!(ScholarError::UniversitySecurityHoldActive as u32 >= 0x1300 && ScholarError::UniversitySecurityHoldActive as u32 < 0x1400);
        assert!(ScholarError::VoteAlreadyInitiated as u32 >= 0x1400 && ScholarError::VoteAlreadyInitiated as u32 < 0x1500);
        assert!(ScholarError::MilestoneNotReady as u32 >= 0x1500 && ScholarError::MilestoneNotReady as u32 < 0x1600);
        assert!(ScholarError::InvalidConfiguration as u32 >= 0x1600 && ScholarError::InvalidConfiguration as u32 < 0x1700);
        assert!(ScholarError::ExportBlockedPendingDiscipline as u32 >= 0x1700 && ScholarError::ExportBlockedPendingDiscipline as u32 < 0x1800);
        assert!(ScholarError::NoActiveCommitteeSession as u32 >= 0x1800 && ScholarError::NoActiveCommitteeSession as u32 < 0x1900);
        assert!(ScholarError::OracleDataStale as u32 >= 0x1900 && ScholarError::OracleDataStale as u32 < 0x1A00);
        assert!(ScholarError::LeaderboardEmpty as u32 >= 0x1A00 && ScholarError::LeaderboardEmpty as u32 < 0x1B00);
    }

    #[test]
    fn test_error_codes_are_unique() {
        use std::collections::HashSet;
        
        let mut codes = HashSet::new();
        let errors = vec![
            ScholarError::Unauthorized,
            ScholarError::NotPlatformAdmin,
            ScholarError::NotUniversityAdmin,
            ScholarError::NotSecurityCouncil,
            ScholarError::NotCommitteeMember,
            ScholarError::Sep12VerificationRequired,
            ScholarError::ScholarshipPaused,
            ScholarError::ScholarshipDisputed,
            ScholarError::ScholarshipNotFound,
            ScholarError::FinalReleaseAlreadyClaimed,
            ScholarError::FinalReleaseNotLocked,
            ScholarError::FinalReleaseLocked,
            ScholarError::InsufficientBalance,
            ScholarError::AmountExceedsAvailableBalance,
            ScholarError::AmountExceedsUnlockedBalance,
            ScholarError::InvalidAmount,
            ScholarError::ProtocolFeeOverflow,
            ScholarError::TaxRateTooHigh,
            ScholarError::NoBalanceToDistribute,
            ScholarError::NativeXLMReserveViolation,
            ScholarError::UniversitySecurityHoldActive,
            ScholarError::SecurityHoldAlreadyInactive,
            ScholarError::SecurityHoldNotFound,
            ScholarError::VoteAlreadyInitiated,
            ScholarError::VoteAlreadyPassed,
            ScholarError::VoterAlreadyVoted,
            ScholarError::VoteNotPassed,
            ScholarError::VoteNotFound,
            ScholarError::MilestoneNotReady,
            ScholarError::MilestoneAlreadyClaimed,
            ScholarError::GPABelowThreshold,
            ScholarError::GPANotSubmitted,
            ScholarError::TranscriptNotVerified,
            ScholarError::InvalidConfiguration,
            ScholarError::ClawbackPercentageTooHigh,
            ScholarError::CommitteeFull,
            ScholarError::InvalidSlot,
            ScholarError::BadMilestoneCount,
            ScholarError::BadMaskLength,
            ScholarError::MilestoneDependencyCycle,
            ScholarError::ExportBlockedPendingDiscipline,
            ScholarError::ReputationExportReplay,
            ScholarError::CrossChainDedupCollision,
            ScholarError::NoActiveCommitteeSession,
            ScholarError::CommitteeStillInWindow,
            ScholarError::AlreadyFinalized,
            ScholarError::CommitteeSessionNotFound,
            ScholarError::OracleDataStale,
            ScholarError::InvalidOracleSignature,
            ScholarError::TimelockNotExpired,
            ScholarError::ReplayAttack,
            ScholarError::LeaderboardEmpty,
            ScholarError::ResearchBonusNotInitialized,
            ScholarError::InvalidTimestamp,
            ScholarError::ExecutionDelayNotMet,
        ];

        for error in errors {
            let code = error as u32;
            assert!(!codes.contains(&code), "Duplicate error code found: 0x{:04X}", code);
            codes.insert(code);
        }
    }
}
