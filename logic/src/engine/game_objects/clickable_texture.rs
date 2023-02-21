use std::rc::Rc;

use crate::{
    engine::{
        game_object::GameObject,
        raycaster::{RaycastTargetID, Raycaster, TextureRaycastTarget},
        webgl_renderer,
    },
    math::Rectangle,
};

use super::texture::Texture;

pub struct ClickableTexture<TargetID>
where
    TargetID: RaycastTargetID,
{
    texture: Texture,
    raycaster: Raycaster<TargetID>,
    target_id: TargetID,
}

impl<TargetID> ClickableTexture<TargetID>
where
    TargetID: RaycastTargetID,
{
    pub fn new(
        bounding_rectangle: Rectangle<f32>,
        target_id: TargetID,
        raycaster: Raycaster<TargetID>,
        texture: &'static webgl_renderer::Texture,
        z_coordinate: f32,
    ) -> Self {
        let result = Self {
            texture: Texture::new(bounding_rectangle.clone(), texture, z_coordinate),
            raycaster: raycaster.clone(),
            target_id: target_id.clone(),
        };

        raycaster.add_target(
            Rc::new(TextureRaycastTarget::new(bounding_rectangle, texture)),
            target_id,
            z_coordinate,
        );

        result
    }
}

impl<TargetID> GameObject for ClickableTexture<TargetID>
where
    TargetID: RaycastTargetID,
{
    fn render(&mut self, renderer: &mut webgl_renderer::WebGlRenderer) {
        self.texture.render(renderer);
    }
}

impl<TargetID> Drop for ClickableTexture<TargetID>
where
    TargetID: RaycastTargetID,
{
    fn drop(&mut self) {
        self.raycaster.remove_target(&self.target_id);
    }
}
