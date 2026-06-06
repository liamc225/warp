use warp_graphql::workspace::TelemetryEnablementSetting as GqlTelemetryEnablementSetting;

use super::organization_telemetry_policy;
use crate::features::FeatureFlag;
use crate::workspaces::workspace::{
    OrganizationTelemetryPolicy, TelemetryEnablementSetting as NativeTelemetryEnablementSetting,
};

#[test]
fn enforced_setting_wins_over_legacy_force_enabled() {
    let _flag = FeatureFlag::EnterpriseTelemetryPolicy.override_enabled(true);
    assert_eq!(
        organization_telemetry_policy(Some(GqlTelemetryEnablementSetting::Disable), true),
        OrganizationTelemetryPolicy::Enforced(NativeTelemetryEnablementSetting::Disabled)
    );
    assert_eq!(
        organization_telemetry_policy(Some(GqlTelemetryEnablementSetting::Enable), false),
        OrganizationTelemetryPolicy::Enforced(NativeTelemetryEnablementSetting::Enabled)
    );
}

#[test]
fn legacy_force_enabled_fallback_preserves_backwards_compatibility() {
    let _flag = FeatureFlag::EnterpriseTelemetryPolicy.override_enabled(true);
    assert_eq!(
        organization_telemetry_policy(None, true),
        OrganizationTelemetryPolicy::Enforced(NativeTelemetryEnablementSetting::Enabled)
    );
    assert_eq!(
        organization_telemetry_policy(None, false),
        OrganizationTelemetryPolicy::Unmanaged
    );
}

#[test]
fn rollout_off_ignores_enforced_setting_and_preserves_legacy_force_enabled() {
    let _flag = FeatureFlag::EnterpriseTelemetryPolicy.override_enabled(false);
    assert_eq!(
        organization_telemetry_policy(Some(GqlTelemetryEnablementSetting::Disable), true),
        OrganizationTelemetryPolicy::Enforced(NativeTelemetryEnablementSetting::Enabled)
    );
    assert_eq!(
        organization_telemetry_policy(Some(GqlTelemetryEnablementSetting::Disable), false),
        OrganizationTelemetryPolicy::Unmanaged
    );
}
