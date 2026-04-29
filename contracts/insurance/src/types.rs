// Data types for the insurance contract (Issue #101 - extracted from lib.rs)

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PolicyStatus {
    Active,
    Expired,
    Cancelled,
    Claimed,
    Suspended,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CoverageType {
    Fire,
    Flood,
    Earthquake,
    Theft,
    LiabilityDamage,
    NaturalDisaster,
    Comprehensive,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ClaimStatus {
    Pending,
    UnderReview,
    OracleVerifying,
    Approved,
    Rejected,
    Paid,
    Disputed,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InsurancePolicy {
    pub policy_id: u64,
    pub property_id: u64,
    pub policyholder: AccountId,
    pub coverage_type: CoverageType,
    pub coverage_amount: u128,
    pub premium_amount: u128,
    pub deductible: u128,
    pub start_time: u64,
    pub end_time: u64,
    pub status: PolicyStatus,
    pub risk_level: RiskLevel,
    pub pool_id: u64,
    pub claims_count: u32,
    pub total_claimed: u128,
    pub metadata_url: String,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InsuranceClaim {
    pub claim_id: u64,
    pub policy_id: u64,
    pub claimant: AccountId,
    pub claim_amount: u128,
    pub description: String,
    pub evidence_url: String,
    pub oracle_report_url: String,
    pub status: ClaimStatus,
    pub submitted_at: u64,
    pub processed_at: Option<u64>,
    pub payout_amount: u128,
    pub assessor: Option<AccountId>,
    pub rejection_reason: String,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RiskPool {
    pub pool_id: u64,
    pub name: String,
    pub coverage_type: CoverageType,
    pub total_capital: u128,
    pub available_capital: u128,
    pub total_premiums_collected: u128,
    pub total_claims_paid: u128,
    pub active_policies: u64,
    pub max_coverage_ratio: u32,
    pub reinsurance_threshold: u128,
    pub created_at: u64,
    pub is_active: bool,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RiskAssessment {
    pub property_id: u64,
    pub location_risk_score: u32,
    pub construction_risk_score: u32,
    pub age_risk_score: u32,
    pub claims_history_score: u32,
    pub overall_risk_score: u32,
    pub risk_level: RiskLevel,
    pub assessed_at: u64,
    pub valid_until: u64,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PremiumCalculation {
    pub base_rate: u32,
    pub risk_multiplier: u32,
    pub coverage_multiplier: u32,
    pub annual_premium: u128,
    pub monthly_premium: u128,
    pub deductible: u128,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ReinsuranceAgreement {
    pub agreement_id: u64,
    pub reinsurer: AccountId,
    pub coverage_limit: u128,
    pub retention_limit: u128,
    pub premium_ceded_rate: u32,
    pub coverage_types: Vec<CoverageType>,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
    pub total_ceded_premiums: u128,
    pub total_recoveries: u128,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InsuranceToken {
    pub token_id: u64,
    pub policy_id: u64,
    pub owner: AccountId,
    pub face_value: u128,
    pub is_tradeable: bool,
    pub created_at: u64,
    pub listed_price: Option<u128>,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ActuarialModel {
    pub model_id: u64,
    pub coverage_type: CoverageType,
    pub loss_frequency: u32,
    pub average_loss_severity: u128,
    pub expected_loss_ratio: u32,
    pub confidence_level: u32,
    pub last_updated: u64,
    pub data_points: u32,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct UnderwritingCriteria {
    pub max_property_age_years: u32,
    pub min_property_value: u128,
    pub max_property_value: u128,
    pub excluded_locations: Vec<String>,
    pub required_safety_features: bool,
    pub max_previous_claims: u32,
    pub min_risk_score: u32,
}

#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PoolLiquidityProvider {
    pub provider: AccountId,
    pub pool_id: u64,
    pub deposited_amount: u128,
    pub share_percentage: u32,
    pub deposited_at: u64,
    pub last_reward_claim: u64,
    pub accumulated_rewards: u128,
}

// =========================================================================
// RISK ASSESSMENT MODEL TYPES (Task #254)
// =========================================================================

/// Property risk factors for comprehensive risk assessment
#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PropertyRiskFactors {
    pub property_id: u64,
    pub property_age_years: u32,
    pub property_value: u128,
    pub location_code: String,
    pub construction_type: String,
    pub has_security_system: bool,
    pub has_fire_extinguisher: bool,
    pub has_alarm_system: bool,
    pub owner_age_years: u32,
    pub years_as_owner: u32,
    pub assessed_at: u64,
}

/// Comprehensive risk assessment model with detailed scoring
#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PropertyRiskModel {
    pub risk_id: u64,
    pub property_id: u64,
    pub property_factors: PropertyRiskFactors,
    pub historical_claims_count: u32,
    pub historical_claims_amount: u128,
    pub location_risk_score: u32,     // 0-1000
    pub construction_risk_score: u32, // 0-1000
    pub age_risk_score: u32,          // 0-1000
    pub ownership_risk_score: u32,    // 0-1000
    pub claims_history_score: u32,    // 0-1000
    pub safety_features_score: u32,   // 0-1000 (higher is safer)
    pub overall_risk_score: u32,      // 0-1000 (weighted average)
    pub final_risk_level: RiskLevel,
    pub premium_multiplier: u32,      // 10000 = 1.0x
    pub assessed_at: u64,
    pub valid_until: u64,
    pub model_version: u32,
}

// =========================================================================
// FRAUD DETECTION TYPES (Task #258)
// =========================================================================

/// Types of fraud indicators detected in claims
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FraudIndicator {
    MultipleClaimsShortPeriod,   // Multiple claims within days
    AnomalousClaimAmount,         // Claim amount far above normal
    SuspiciousTimingPattern,      // Claims on weekends/holidays
    ExcessiveCoverageRatio,       // Claim close to max coverage
    HistoricalFraudPattern,       // Policyholder with history
    Misrepresentation,            // Inconsistent claim details
    KnownFraudNetwork,            // Associated with fraudulent accounts
    DuplicateClaimPatterns,       // Similar to previous fraud claims
}

/// Fraud risk assessment for a claim
#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FraudRiskAssessment {
    pub assessment_id: u64,
    pub claim_id: u64,
    pub policy_id: u64,
    pub policyholder: AccountId,
    pub fraud_score: u32,              // 0-1000 (higher = more fraud risk)
    pub fraud_level: RiskLevel,        // Fraud risk level
    pub detected_indicators: Vec<FraudIndicator>,
    pub claim_amount: u128,
    pub expected_amount_range: (u128, u128), // (min, max) expected
    pub time_since_last_claim: Option<u64>,  // seconds
    pub similar_claims_count: u32,     // Similar historical claims
    pub policyholder_claims_count: u32,
    pub assessor_notes: String,
    pub assessment_timestamp: u64,
    pub requires_manual_review: bool,
}

/// Historical fraud pattern for detection
#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FraudPattern {
    pub pattern_id: u64,
    pub pattern_type: FraudIndicator,
    pub description: String,
    pub severity_weight: u32, // Weight in fraud scoring (0-1000)
    pub triggered_count: u32, // How many times this pattern triggered
    pub last_triggered: u64,
    pub is_active: bool,
}

/// Statistics for fraud detection and prevention
#[derive(
    Debug, Clone, PartialEq, scale::Encode, scale::Decode, ink::storage::traits::StorageLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FraudDetectionStats {
    pub total_assessments: u32,
    pub high_risk_claims: u32,
    pub rejected_fraud_claims: u32,
    pub patterns_detected: u32,
    pub false_positive_count: u32,
    pub average_fraud_score: u32,
    pub last_update: u64,
}
