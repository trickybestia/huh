mod polygon_raycast_target;
mod texture_raycast_target;

use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::math::{compare, Vector2};

pub use texture_raycast_target::TextureRaycastTarget;

pub trait RaycastTarget {
    fn contains_point(&self, point: &Vector2<f32>) -> bool;
}

pub trait RaycastTargetID: Eq + Hash + Clone {}

impl<T> RaycastTargetID for T where T: Eq + Hash + Clone {}

struct TargetEntry {
    pub z_index: f32,
    pub target: Rc<dyn RaycastTarget>,
}

struct RaycasterInternal<TargetID>
where
    TargetID: RaycastTargetID,
{
    targets: HashMap<TargetID, TargetEntry>,
}

impl<TargetID> RaycasterInternal<TargetID>
where
    TargetID: RaycastTargetID,
{
    pub fn new() -> Self {
        Self {
            targets: HashMap::new(),
        }
    }

    pub fn add_target(&mut self, target: Rc<dyn RaycastTarget>, id: TargetID, z_index: f32) {
        self.targets.insert(id, TargetEntry { z_index, target });
    }

    pub fn remove_target(&mut self, id: &TargetID) {
        self.targets.remove(id);
    }

    pub fn raycast(&self, point: &Vector2<f32>) -> Option<TargetID> {
        let mut target = None;

        for (id, entry) in &self.targets {
            if entry.target.contains_point(point) {
                if let Some((_, previous_z_index)) = &target {
                    if compare(entry.z_index, *previous_z_index).is_gt() {
                        target = Some((id, entry.z_index));
                    }
                } else {
                    target = Some((id, entry.z_index));
                }
            }
        }

        target.map(|(id, _)| id.clone())
    }

    pub fn clear(&mut self) {
        self.targets.clear();
    }
}

#[derive(Clone)]
pub struct Raycaster<TargetID>(Rc<RefCell<RaycasterInternal<TargetID>>>)
where
    TargetID: RaycastTargetID;

impl<TargetID> Raycaster<TargetID>
where
    TargetID: RaycastTargetID,
{
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(RaycasterInternal::new())))
    }

    pub fn add_target(&self, target: Rc<dyn RaycastTarget>, id: TargetID, z_index: f32) {
        self.0.borrow_mut().add_target(target, id, z_index);
    }

    pub fn remove_target(&self, id: &TargetID) {
        self.0.borrow_mut().remove_target(id);
    }

    pub fn raycast(&self, point: &Vector2<f32>) -> Option<TargetID> {
        self.0.borrow().raycast(point)
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }
}
