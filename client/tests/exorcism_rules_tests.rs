use client::gameplay::exorcism::rules::{banshee_advance, onryo_stack_tick, spirit_progress};
use client::gameplay::exorcism::tables::ExorcismTables;
use client::gameplay::exorcism::ExorcismState;

#[test]
fn spirit_progress_increases_when_watched() {
    let tables = ExorcismTables::default();
    let next = spirit_progress(
        0.2,
        true,
        1.0,
        tables.spirit.rate_up,
        tables.spirit.rate_down,
    );
    assert!(next > 0.2);
}

#[test]
fn spirit_progress_decays_when_unwatched() {
    let tables = ExorcismTables::default();
    let next = spirit_progress(
        0.2,
        false,
        1.0,
        tables.spirit.rate_up,
        tables.spirit.rate_down,
    );
    assert!(next < 0.2);
}

#[test]
fn banshee_advance_fails_on_wrong_order() {
    let state = banshee_advance(1, 3, true, false);
    assert!(matches!(state, ExorcismState::Failed));
}

#[test]
fn banshee_advance_completes_on_last_step() {
    let state = banshee_advance(2, 3, true, true);
    assert!(matches!(state, ExorcismState::Complete));
}

#[test]
fn onryo_stack_fails_at_max() {
    let tables = ExorcismTables::default();
    let (stacks, failed) = onryo_stack_tick(
        4.9,
        1.0,
        true,
        tables.onryo.max_stacks,
        tables.onryo.stack_rate,
    );
    assert!(stacks >= tables.onryo.max_stacks);
    assert!(failed);
}
