use crate::core::GhostType;
use crate::gameplay::evidence::{
    emf_five_candidate, emf_level, overlap_distance, spiritbox_bearing, spiritbox_is_evidence,
    spiritbox_reply, EvidenceTuning, SpiritboxBearing, SpiritboxReply,
};
use bevy::prelude::Vec3;

#[test]
fn spirit_emf_levels_ramp_with_distance() {
    let same_room = true;
    let tuning = EvidenceTuning::default();
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_4 - 0.1, same_room, &tuning),
        4
    );
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_4, same_room, &tuning),
        4
    );
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_4 + 0.2, same_room, &tuning),
        3
    );
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_3 + 0.2, same_room, &tuning),
        2
    );
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_2 + 0.6, same_room, &tuning),
        1
    );
}

#[test]
fn spirit_emf_is_one_at_far_distance() {
    let tuning = EvidenceTuning::default();
    assert_eq!(
        emf_level(GhostType::Spirit, tuning.emf_range_2 + 0.6, false, &tuning),
        1
    );
}

#[test]
fn non_spirit_emf_is_one() {
    let tuning = EvidenceTuning::default();
    assert_eq!(emf_level(GhostType::Banshee, 0.5, true, &tuning), 1);
    assert_eq!(emf_level(GhostType::Banshee, 0.5, false, &tuning), 1);
    assert_eq!(emf_level(GhostType::Onryo, 9.0, true, &tuning), 1);
}

#[test]
fn emf_five_candidate_requires_intersection() {
    let tuning = EvidenceTuning::default();
    let overlap = overlap_distance(&tuning);
    assert!(emf_five_candidate(GhostType::Spirit, overlap - 0.1, &tuning));
    assert!(emf_five_candidate(GhostType::Spirit, overlap, &tuning));
    assert!(!emf_five_candidate(GhostType::Spirit, overlap + 0.1, &tuning));
    assert!(!emf_five_candidate(GhostType::Banshee, overlap - 0.1, &tuning));
}

#[test]
fn spiritbox_replies_directionally_for_banshee_contact() {
    let tuning = EvidenceTuning::default();
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Ahead
        ),
        SpiritboxReply::Here
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Left
        ),
        SpiritboxReply::Left
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Right
        ),
        SpiritboxReply::Right
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Behind
        ),
        SpiritboxReply::Behind
    );
}

#[test]
fn spiritbox_requires_banshee_same_room_and_range() {
    let tuning = EvidenceTuning::default();
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            false,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Ahead
        ),
        SpiritboxReply::Static
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Banshee,
            true,
            tuning.spiritbox_range + 0.1,
            &tuning,
            SpiritboxBearing::Ahead
        ),
        SpiritboxReply::Static
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Spirit,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Ahead
        ),
        SpiritboxReply::Static
    );
    assert_eq!(
        spiritbox_reply(
            GhostType::Onryo,
            true,
            tuning.spiritbox_range - 0.1,
            &tuning,
            SpiritboxBearing::Ahead
        ),
        SpiritboxReply::Static
    );
}

#[test]
fn spiritbox_bearing_tracks_relative_direction() {
    let player_pos = Vec3::ZERO;
    let forward = Vec3::Z;

    assert_eq!(
        spiritbox_bearing(forward, player_pos, Vec3::new(0.0, 0.0, 4.0)),
        SpiritboxBearing::Ahead
    );
    assert_eq!(
        spiritbox_bearing(forward, player_pos, Vec3::new(-4.0, 0.0, 0.0)),
        SpiritboxBearing::Left
    );
    assert_eq!(
        spiritbox_bearing(forward, player_pos, Vec3::new(4.0, 0.0, 0.0)),
        SpiritboxBearing::Right
    );
    assert_eq!(
        spiritbox_bearing(forward, player_pos, Vec3::new(0.0, 0.0, -4.0)),
        SpiritboxBearing::Behind
    );
}

#[test]
fn spiritbox_evidence_only_for_actual_replies() {
    assert!(spiritbox_is_evidence(SpiritboxReply::Here));
    assert!(spiritbox_is_evidence(SpiritboxReply::Left));
    assert!(spiritbox_is_evidence(SpiritboxReply::Right));
    assert!(spiritbox_is_evidence(SpiritboxReply::Behind));
    assert!(!spiritbox_is_evidence(SpiritboxReply::Static));
}
