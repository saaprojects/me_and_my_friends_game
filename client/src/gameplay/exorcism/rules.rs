use super::ExorcismState;

pub fn spirit_progress(
    mut progress: f32,
    all_watched: bool,
    dt: f32,
    rate_up: f32,
    rate_down: f32,
) -> f32 {
    if all_watched {
        progress += rate_up * dt;
    } else {
        progress -= rate_down * dt;
    }
    progress.clamp(0.0, 1.0)
}

pub fn banshee_advance(
    stage: u8,
    sequence_len: u8,
    timing_ok: bool,
    order_ok: bool,
) -> ExorcismState {
    if !order_ok || !timing_ok {
        return ExorcismState::Failed;
    }

    let next = stage + 1;
    if next >= sequence_len {
        ExorcismState::Complete
    } else {
        ExorcismState::Stage(next)
    }
}

pub fn onryo_stack_tick(
    stacks: f32,
    dt: f32,
    carrying: bool,
    max_stacks: f32,
    stack_rate: f32,
) -> (f32, bool) {
    if !carrying {
        return (stacks, false);
    }
    let new_stacks = stacks + dt * stack_rate;
    (new_stacks, new_stacks >= max_stacks)
}
