use crate::{
    engine::{
        game_object::GameObject,
        raycaster::{RaycastTargetID, Raycaster},
        webgl_renderer::Color,
        WebGlRenderer,
    },
    math,
};

use super::polygon::Polygon;

pub struct ClickablePolygon<TargetID>
where
    TargetID: RaycastTargetID,
{
    polygon: Polygon,
    target_id: TargetID,
    raycaster: Raycaster<TargetID>,
}

impl<TargetID> ClickablePolygon<TargetID>
where
    TargetID: RaycastTargetID,
{
    pub fn new(
        polygon: impl Into<math::Polygon>,
        target_id: TargetID,
        raycaster: Raycaster<TargetID>,
        z_index: f32,
        color: Color,
    ) -> Self {
        let result = Self {
            polygon: Polygon::new(polygon, z_index, color),
            target_id: target_id.clone(),
            raycaster: raycaster.clone(),
        };

        raycaster.add_target(result.polygon.polygon().clone(), target_id, z_index);

        result
    }
}

impl<TargetID> GameObject for ClickablePolygon<TargetID>
where
    TargetID: RaycastTargetID,
{
    fn render(&mut self, renderer: &mut WebGlRenderer) {
        self.polygon.render(renderer);
    }
}

impl<TargetID> Drop for ClickablePolygon<TargetID>
where
    TargetID: RaycastTargetID,
{
    fn drop(&mut self) {
        self.raycaster.remove_target(&self.target_id);
    }
}
