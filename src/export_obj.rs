extern crate obj_exporter as obj;

use obj::{Geometry, ObjSet, Object, Primitive, Shape, TVertex, Vertex};

use crate::BCH;

pub fn bch_to_obj(bch: &BCH) -> ObjSet { //TODO: error handling
    let mut objects: Vec<Object> = Vec::new();
    for model in &bch.models {
        for object in &model.mesh {
            println!("len vertices: {}", object.vertices.len());
            let name = object.name.clone();
            println!("added {}", name);
            let mut vertices = Vec::new();
            for vertice in &object.vertices {
                vertices.push(Vertex { x: vertice.position[0] as f64, y: vertice.position[1] as f64, z: vertice.position[2] as f64})
            };
            let tex_vertices = Vec::new();
            let normals = Vec::new();
            let mut shapes = Vec::new();
            // vertices are paired by 3
            for entry_id in 0..object.vertices.len()/3 {
                shapes.push(Shape {
                    primitive: Primitive::Triangle (
                        (entry_id*3, Some(entry_id*3), Some(0)),
                        (entry_id*3 + 1, Some(entry_id*3 + 1), Some(0)),
                        (entry_id*3 + 2, Some(entry_id*3 + 2), Some(0))
                    ),
                    groups: vec![],
                    smoothing_groups: vec![],
                })
            };

            objects.push(Object {
                name,
                vertices,
                tex_vertices,
                normals,
                geometry: vec![Geometry {
                    material_name: None,
                    shapes
                }],
            })
        };
    };

    ObjSet {
        material_library: None,
        objects,
    }

    /*ObjSet {
      material_library: None,
      objects: vec![
        Object {
          name: "Square".to_owned(),
          vertices: vec![
            (-1.0, -1.0, 0.0),
            (1.0, -1.0, 0.0),
            (1.0, 1.0, 0.0),
            (-1.0, 1.0, 0.0),
          ].into_iter()
            .map(|(x, y, z)| Vertex { x, y, z })
            .collect(),
          tex_vertices: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .into_iter()
            .map(|(u, v)| TVertex { u, v, w: 0.0 })
            .collect(),
          normals: vec![
            Vertex {
              x: 0.0,
              y: 0.0,
              z: -1.0,
            },
          ],
          geometry: vec![
            Geometry {
              material_name: None,
              shapes: vec![(0, 1, 2), (0, 2, 3)]
                .into_iter()
                .map(|(x, y, z)| Shape {
                  primitive: Primitive::Triangle(
                    (x, Some(x), Some(0)),
                    (y, Some(y), Some(0)),
                    (z, Some(z), Some(0)),
                  ),
                  groups: vec![],
                  smoothing_groups: vec![],
                })
                .collect(),
            },
          ],
        },
      ],
  }*/
}
