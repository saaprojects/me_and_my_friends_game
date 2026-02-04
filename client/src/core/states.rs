use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Ghost,
    Investigator,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Equipment {
    Emf,
    Spiritbox,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GhostType {
    Spirit,
    Banshee,
    Onryo,
}

#[derive(Resource)]
pub struct RoleState {
    pub current: Role,
}

#[derive(Resource)]
pub struct MenuState {
    pub open: bool,
    pub selected_role: Role,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuScreen {
    Start,
    RoleSelect,
    GhostDetails,
    InvestigatorDetails,
}

#[derive(Resource)]
pub struct MenuFlowState {
    pub screen: MenuScreen,
}

#[derive(Resource)]
pub struct JournalState {
    pub open: bool,
}

#[derive(Resource)]
pub struct RoleYaw {
    pub ghost: f32,
    pub investigator: f32,
}

#[derive(Resource)]
pub struct CameraControl {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Resource)]
pub struct GhostTypeState {
    pub selected: GhostType,
    pub active: GhostType,
}

#[derive(Resource)]
pub struct SessionState {
    pub started: bool,
}

pub const DEFAULT_GHOST_YAW: f32 = 0.0;
pub const DEFAULT_GHOST_PITCH: f32 = 0.12;
pub const DEFAULT_INVESTIGATOR_YAW: f32 = 0.0;
pub const DEFAULT_INVESTIGATOR_PITCH: f32 = 0.2;

pub fn set_default_camera(role: Role, control: &mut CameraControl, role_yaw: &mut RoleYaw) {
    match role {
        Role::Ghost => {
            control.yaw = DEFAULT_GHOST_YAW;
            control.pitch = DEFAULT_GHOST_PITCH;
            role_yaw.ghost = DEFAULT_GHOST_YAW;
        }
        Role::Investigator => {
            control.yaw = DEFAULT_INVESTIGATOR_YAW;
            control.pitch = DEFAULT_INVESTIGATOR_PITCH;
            role_yaw.investigator = DEFAULT_INVESTIGATOR_YAW;
        }
    }
}
