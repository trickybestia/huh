use crate::math::{compare, Polygon, Vector2};

struct PolygonEntry<PolygonID> {
    pub z_index: f32,
    pub polygon: Polygon,
    pub id: PolygonID,
}

pub struct Raycaster<PolygonID> {
    polygons: Vec<PolygonEntry<PolygonID>>,
    dirty: bool,
}

impl<PolygonID> Raycaster<PolygonID>
where
    PolygonID: Clone,
{
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            dirty: false,
        }
    }

    pub fn add_polygon(&mut self, polygon: impl Into<Polygon>, id: PolygonID, z_index: f32) {
        self.polygons.push(PolygonEntry {
            z_index,
            polygon: polygon.into(),
            id,
        });

        self.dirty = true;
    }

    pub fn sort(&mut self) {
        self.polygons.sort_by(|a, b| compare(a.z_index, b.z_index));

        self.dirty = false;
    }

    pub fn raycast(&self, point: &Vector2<f32>) -> Option<PolygonID> {
        if self.dirty {
            panic!("sort() must be called between end of filling Raycaster with polygons and calling containing_polygon()");
        }

        for polygon in &self.polygons {
            if polygon.polygon.contains_point(point) {
                return Some(polygon.id.clone());
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.polygons.clear();
    }
}
