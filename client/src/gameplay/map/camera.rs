use bevy::prelude::Vec3;

use super::collision::collides;
use super::components::{Bounds, Obstacle};

pub fn clamp_camera_distance(base: Vec3, dir: Vec3, desired: f32, bounds: Bounds) -> f32 {
    let margin = 0.6;
    let mut max_t = desired;

    if dir.x.abs() > f32::EPSILON {
        let bound_x = if dir.x > 0.0 {
            bounds.max_x - margin
        } else {
            bounds.min_x + margin
        };
        let t = (bound_x - base.x) / dir.x;
        if t.is_finite() && t > 0.0 {
            max_t = max_t.min(t);
        }
    }

    if dir.z.abs() > f32::EPSILON {
        let bound_z = if dir.z > 0.0 {
            bounds.max_z - margin
        } else {
            bounds.min_z + margin
        };
        let t = (bound_z - base.z) / dir.z;
        if t.is_finite() && t > 0.0 {
            max_t = max_t.min(t);
        }
    }

    max_t.clamp(1.2, desired)
}

pub fn avoid_camera_obstacles(
    base: Vec3,
    dir: Vec3,
    mut distance: f32,
    radius: f32,
    obstacles: &[Obstacle],
) -> f32 {
    let step = 0.2;
    while distance > 1.2 {
        let candidate = base + dir * distance;
        if !collides(candidate, radius, obstacles)
            && !segment_collides(base, candidate, radius, obstacles)
        {
            break;
        }
        distance -= step;
        if distance <= 1.2 {
            return 1.2;
        }
    }
    distance
}

fn segment_collides(start: Vec3, end: Vec3, radius: f32, obstacles: &[Obstacle]) -> bool {
    let delta = end - start;
    let len = delta.length();
    if len <= f32::EPSILON {
        return collides(start, radius, obstacles);
    }

    let dir = delta / len;
    let step = (radius * 0.5).max(0.05);
    let mut t = step;
    while t < len {
        let point = start + dir * t;
        if collides(point, radius, obstacles) {
            return true;
        }
        t += step;
    }
    false
}
