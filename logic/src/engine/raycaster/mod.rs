mod polygon_raycast_target;
mod texture_raycast_target;

use crate::math::{compare, Vector2};

pub use texture_raycast_target::TextureRaycastTarget;

pub trait RaycastTarget {
    fn contains_point(&self, point: &Vector2<f32>) -> bool;
}

struct TargetEntry<TargetID> {
    pub z_index: f32,
    pub target: Box<dyn RaycastTarget>,
    pub id: TargetID,
}

pub struct Raycaster<TargetID> {
    targets: Vec<TargetEntry<TargetID>>,
    dirty: bool,
}

impl<TargetID> Raycaster<TargetID>
where
    TargetID: Clone,
{
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            dirty: false,
        }
    }

    pub fn add_target(&mut self, target: impl RaycastTarget + 'static, id: TargetID, z_index: f32) {
        self.targets.push(TargetEntry {
            z_index,
            target: Box::new(target),
            id,
        });

        self.dirty = true;
    }

    pub fn sort(&mut self) {
        self.targets.sort_by(|a, b| compare(a.z_index, b.z_index));

        self.dirty = false;
    }

    pub fn raycast(&self, point: &Vector2<f32>) -> Option<TargetID> {
        if self.dirty {
            panic!("sort() must be called between end of filling Raycaster with polygons and calling containing_polygon()");
        }

        for target in &self.targets {
            if target.target.contains_point(point) {
                return Some(target.id.clone());
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.targets.clear();
    }
}
