use crate::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Resource)]
pub struct CollisionWorld {
    pub bounds: Bounds,
    pub obstacles: Vec<Obstacle>,
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Obstacle {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct RoomZone {
    pub id: u8,
    pub name: &'static str,
    pub bounds: Bounds,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct WallVisual {
    pub size: Vec3,
    pub translation: Vec3,
    pub color: [f32; 3],
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExorcismLayout {
    pub spirit_anchors: Vec<Vec3>,
    pub banshee_anchors: Vec<Vec3>,
    pub onryo_cursed_positions: Vec<Vec3>,
    pub onryo_ritual_positions: Vec<Vec3>,
}

#[allow(dead_code)]
#[derive(Resource, Clone, Debug)]
pub struct HouseLayout {
    pub bounds: Bounds,
    pub obstacles: Vec<Obstacle>,
    pub rooms: Vec<RoomZone>,
    pub walls: Vec<WallVisual>,
    pub exorcism: ExorcismLayout,
    pub investigator_spawn: Vec3,
    pub ghost_spawns: Vec<Vec3>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseLayoutKind {
    TwoRoom,
    ThreeRoom,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct HouseLayoutSelection {
    pub selected_kind: HouseLayoutKind,
    pub active_kind: HouseLayoutKind,
}

impl Default for HouseLayoutSelection {
    fn default() -> Self {
        Self {
            selected_kind: HouseLayoutKind::TwoRoom,
            active_kind: HouseLayoutKind::TwoRoom,
        }
    }
}

impl HouseLayout {
    pub fn for_kind(kind: HouseLayoutKind) -> Self {
        match kind {
            HouseLayoutKind::TwoRoom => Self::two_room(),
            HouseLayoutKind::ThreeRoom => Self::three_room(),
        }
    }

    pub fn two_room() -> Self {
        let bounds = Bounds {
            min_x: -9.4,
            max_x: 9.4,
            min_z: -9.4,
            max_z: 9.4,
        };

        let divider_segments = [
            Obstacle {
                min_x: 1.8,
                max_x: 2.2,
                min_z: -9.4,
                max_z: -1.2,
            },
            Obstacle {
                min_x: 1.8,
                max_x: 2.2,
                min_z: 1.2,
                max_z: 9.4,
            },
        ];

        let props = [
            Obstacle {
                min_x: -4.6,
                max_x: -2.4,
                min_z: -1.5,
                max_z: -0.5,
            },
            Obstacle {
                min_x: 3.3,
                max_x: 4.7,
                min_z: 2.3,
                max_z: 3.7,
            },
        ];

        let mut obstacles = Vec::with_capacity(divider_segments.len() + props.len());
        obstacles.extend(divider_segments);
        obstacles.extend(props);

        Self {
            bounds,
            obstacles,
            rooms: vec![
                RoomZone {
                    id: 0,
                    name: "Main Room",
                    bounds: Bounds {
                        min_x: -9.4,
                        max_x: 1.8,
                        min_z: -9.4,
                        max_z: 9.4,
                    },
                },
                RoomZone {
                    id: 1,
                    name: "Side Room",
                    bounds: Bounds {
                        min_x: 2.2,
                        max_x: 9.4,
                        min_z: -9.4,
                        max_z: 9.4,
                    },
                },
            ],
            walls: vec![
                WallVisual {
                    size: Vec3::new(20.0, 4.0, 0.4),
                    translation: Vec3::new(0.0, 2.0, -10.0),
                    color: [0.08, 0.10, 0.15],
                },
                WallVisual {
                    size: Vec3::new(20.0, 4.0, 0.4),
                    translation: Vec3::new(0.0, 2.0, 10.0),
                    color: [0.06, 0.08, 0.13],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 20.0),
                    translation: Vec3::new(-10.0, 2.0, 0.0),
                    color: [0.06, 0.08, 0.13],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 20.0),
                    translation: Vec3::new(10.0, 2.0, 0.0),
                    color: [0.08, 0.10, 0.15],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 8.6),
                    translation: Vec3::new(2.0, 2.0, -5.5),
                    color: [0.09, 0.12, 0.18],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 8.6),
                    translation: Vec3::new(2.0, 2.0, 5.5),
                    color: [0.09, 0.12, 0.18],
                },
            ],
            exorcism: ExorcismLayout {
                spirit_anchors: vec![
                    Vec3::new(-6.0, 0.7, -6.0),
                    Vec3::new(6.0, 0.7, -5.5),
                    Vec3::new(-5.5, 0.7, 6.0),
                ],
                banshee_anchors: vec![
                    Vec3::new(-4.0, 0.5, -2.0),
                    Vec3::new(4.5, 0.5, -1.5),
                    Vec3::new(0.0, 0.5, 5.0),
                ],
                onryo_cursed_positions: vec![
                    Vec3::new(-6.5, 0.4, 0.0),
                    Vec3::new(6.5, 0.4, 0.0),
                    Vec3::new(0.0, 0.4, -6.5),
                ],
                onryo_ritual_positions: vec![
                    Vec3::new(-2.5, 0.1, 2.5),
                    Vec3::new(2.5, 0.1, 2.5),
                    Vec3::new(0.0, 0.1, 6.5),
                ],
            },
            investigator_spawn: Vec3::new(-6.0, 0.9, 0.0),
            ghost_spawns: vec![Vec3::new(-5.0, 1.6, 4.5), Vec3::new(5.6, 1.6, 0.0)],
        }
    }

    pub fn three_room() -> Self {
        let bounds = Bounds {
            min_x: -9.4,
            max_x: 9.4,
            min_z: -9.4,
            max_z: 9.4,
        };

        let divider_segments = [
            Obstacle {
                min_x: 1.8,
                max_x: 2.2,
                min_z: -9.4,
                max_z: -1.2,
            },
            Obstacle {
                min_x: 1.8,
                max_x: 2.2,
                min_z: 1.2,
                max_z: 9.4,
            },
            Obstacle {
                min_x: -9.4,
                max_x: -1.8,
                min_z: -2.2,
                max_z: -1.8,
            },
            Obstacle {
                min_x: 0.2,
                max_x: 1.8,
                min_z: -2.2,
                max_z: -1.8,
            },
        ];

        let props = [
            Obstacle {
                min_x: -4.6,
                max_x: -2.4,
                min_z: -1.5,
                max_z: -0.5,
            },
            Obstacle {
                min_x: 3.3,
                max_x: 4.7,
                min_z: 2.3,
                max_z: 3.7,
            },
        ];

        let mut obstacles = Vec::with_capacity(divider_segments.len() + props.len());
        obstacles.extend(divider_segments);
        obstacles.extend(props);

        Self {
            bounds,
            obstacles,
            rooms: vec![
                RoomZone {
                    id: 0,
                    name: "Main Room",
                    bounds: Bounds {
                        min_x: -9.4,
                        max_x: 1.8,
                        min_z: -9.4,
                        max_z: -2.2,
                    },
                },
                RoomZone {
                    id: 1,
                    name: "Upper Room",
                    bounds: Bounds {
                        min_x: -9.4,
                        max_x: 1.8,
                        min_z: -1.8,
                        max_z: 9.4,
                    },
                },
                RoomZone {
                    id: 2,
                    name: "Side Room",
                    bounds: Bounds {
                        min_x: 2.2,
                        max_x: 9.4,
                        min_z: -9.4,
                        max_z: 9.4,
                    },
                },
            ],
            walls: vec![
                WallVisual {
                    size: Vec3::new(20.0, 4.0, 0.4),
                    translation: Vec3::new(0.0, 2.0, -10.0),
                    color: [0.08, 0.10, 0.15],
                },
                WallVisual {
                    size: Vec3::new(20.0, 4.0, 0.4),
                    translation: Vec3::new(0.0, 2.0, 10.0),
                    color: [0.06, 0.08, 0.13],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 20.0),
                    translation: Vec3::new(-10.0, 2.0, 0.0),
                    color: [0.06, 0.08, 0.13],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 20.0),
                    translation: Vec3::new(10.0, 2.0, 0.0),
                    color: [0.08, 0.10, 0.15],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 8.6),
                    translation: Vec3::new(2.0, 2.0, -5.5),
                    color: [0.09, 0.12, 0.18],
                },
                WallVisual {
                    size: Vec3::new(0.4, 4.0, 8.6),
                    translation: Vec3::new(2.0, 2.0, 5.5),
                    color: [0.09, 0.12, 0.18],
                },
                WallVisual {
                    size: Vec3::new(8.0, 4.0, 0.4),
                    translation: Vec3::new(-5.8, 2.0, -2.0),
                    color: [0.09, 0.12, 0.18],
                },
                WallVisual {
                    size: Vec3::new(1.8, 4.0, 0.4),
                    translation: Vec3::new(1.1, 2.0, -2.0),
                    color: [0.09, 0.12, 0.18],
                },
            ],
            exorcism: ExorcismLayout {
                spirit_anchors: vec![
                    Vec3::new(-6.0, 0.7, -6.0),
                    Vec3::new(-5.5, 0.7, 6.0),
                    Vec3::new(6.0, 0.7, -5.5),
                ],
                banshee_anchors: vec![
                    Vec3::new(-4.0, 0.5, -4.5),
                    Vec3::new(-1.0, 0.5, 5.0),
                    Vec3::new(4.5, 0.5, -1.5),
                ],
                onryo_cursed_positions: vec![
                    Vec3::new(-6.5, 0.4, -6.0),
                    Vec3::new(-6.5, 0.4, 4.5),
                    Vec3::new(6.5, 0.4, 0.0),
                ],
                onryo_ritual_positions: vec![
                    Vec3::new(-2.5, 0.1, -5.5),
                    Vec3::new(-2.5, 0.1, 3.5),
                    Vec3::new(2.5, 0.1, 6.5),
                ],
            },
            investigator_spawn: Vec3::new(-6.0, 0.9, -5.5),
            ghost_spawns: vec![
                Vec3::new(-5.0, 1.6, -6.0),
                Vec3::new(-5.0, 1.6, 5.0),
                Vec3::new(5.6, 1.6, 0.0),
            ],
        }
    }

    pub fn collision_world(&self) -> CollisionWorld {
        CollisionWorld {
            bounds: self.bounds,
            obstacles: self.obstacles.clone(),
        }
    }

    pub fn random_ghost_spawn(&self) -> Vec3 {
        if self.ghost_spawns.is_empty() {
            return Vec3::new(0.0, 1.6, 0.0);
        }
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        self.ghost_spawns[(seed % self.ghost_spawns.len() as u128) as usize]
    }
}

impl Bounds {
    #[allow(dead_code)]
    pub fn contains_xz(&self, pos: Vec3) -> bool {
        pos.x >= self.min_x && pos.x <= self.max_x && pos.z >= self.min_z && pos.z <= self.max_z
    }
}
