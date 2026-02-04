use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crossbeam_channel::{self, Receiver};
use reqwest::blocking::get;
use shared::prelude::Health;
use std::thread;
use std::time::Duration;

use crate::core::RoleState;

#[derive(Resource, Clone)]
pub struct HealthState {
    pub status: String,
}

#[derive(Resource)]
pub struct HealthChannel {
    pub rx: Receiver<HealthState>,
}

pub fn spawn_health_thread() -> Receiver<HealthState> {
    let (tx, rx) = crossbeam_channel::unbounded::<HealthState>();
    thread::spawn(move || loop {
        let state = fetch_health();
        let _ = tx.send(state);
        thread::sleep(Duration::from_secs(2));
    });
    rx
}

pub fn update_health(mut health: ResMut<HealthState>, channel: Res<HealthChannel>) {
    while let Ok(new_state) = channel.rx.try_recv() {
        *health = new_state;
    }
}

pub fn update_window_title(
    role: Res<RoleState>,
    health: Res<HealthState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let role_label = match role.current {
        crate::core::Role::Ghost => "Ghost",
        crate::core::Role::Investigator => "Investigator",
    };
    let mut window = windows.single_mut();
    window.title = format!("Me & My Friends - {} - {}", role_label, health.status);
}

fn fetch_health() -> HealthState {
    match get("http://localhost:8000/health") {
        Ok(resp) => {
            if !resp.status().is_success() {
                return HealthState {
                    status: "error".into(),
                };
            }

            match resp.json::<Health>() {
                Ok(payload) => HealthState {
                    status: payload.status.clone(),
                },
                Err(_) => HealthState {
                    status: "error".into(),
                },
            }
        }
        Err(_) => HealthState {
            status: "error".into(),
        },
    }
}
