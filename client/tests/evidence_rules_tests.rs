use client::core::GhostType;
use client::gameplay::evidence::{
    emf_five_candidate, emf_level, overlap_distance, spiritbox_is_evidence, spiritbox_reply,
    EvidenceTuning, SpiritboxReply,
};

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
fn spiritbox_replies_match_banshee_distance() {
    assert_eq!(
        spiritbox_reply(GhostType::Banshee, true),
        SpiritboxReply::Here
    );
    assert_eq!(
        spiritbox_reply(GhostType::Banshee, false),
        SpiritboxReply::Static
    );
}

#[test]
fn spiritbox_non_banshee_is_static_or_silence() {
    assert_eq!(
        spiritbox_reply(GhostType::Spirit, false),
        SpiritboxReply::Static
    );
    assert_eq!(
        spiritbox_reply(GhostType::Onryo, true),
        SpiritboxReply::Static
    );
}

#[test]
fn spiritbox_evidence_only_for_close_or_here() {
    assert!(spiritbox_is_evidence(SpiritboxReply::Here));
    assert!(!spiritbox_is_evidence(SpiritboxReply::Static));
}
