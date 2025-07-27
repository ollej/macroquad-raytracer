use crate::{float::*, material::*, matrix::IDENTITY_MATRIX, object::*, tuple::*};

use std::collections::HashMap;

pub struct ObjParser<'a> {
    content: &'a str,
    pub ignored: usize,
    pub vertices: Vec<Point>,
    pub groups: HashMap<&'a str, Object>,
    latest_group: &'a str,
}

impl<'a> ObjParser<'a> {
    const DEFAULT_GROUP: &'a str = "__DefaultGroup__";

    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            ignored: 0,
            vertices: vec![Point::empty_point()],
            groups: HashMap::from([(
                Self::DEFAULT_GROUP,
                Object::new_group(IDENTITY_MATRIX, Material::default()),
            )]),
            latest_group: Self::DEFAULT_GROUP,
        }
    }

    pub fn parse(&mut self) {
        self.content.lines().for_each(|line| self.parse_line(line));
    }

    pub fn default_group(&self) -> Option<&Object> {
        self.groups.get(Self::DEFAULT_GROUP)
    }

    fn parse_line(&mut self, line: &'a str) {
        let values = line.split(" ").collect::<Vec<&'a str>>();
        match values.split_first() {
            Some((&"v", arguments)) => self.parse_vertice(arguments),
            Some((&"f", arguments)) => self.parse_face(arguments),
            Some((&"g", arguments)) => self.create_group(arguments),
            _ => self.ignored += 1,
        }
    }

    fn parse_vertice(&mut self, arguments: &[&'a str]) {
        let numbers: Option<Vec<Float>> =
            arguments.iter().map(|f| f.parse::<Float>().ok()).collect();
        if let Some(p) = numbers {
            if p.len() == 3 {
                self.vertices.push(Point::point(p[0], p[1], p[2]));
                return;
            }
        }
        self.ignored += 1;
    }

    fn parse_face(&mut self, arguments: &[&'a str]) {
        let points: Vec<&Point> = arguments
            .iter()
            .flat_map(|f| f.parse::<usize>().ok())
            .flat_map(|index| self.vertices.get(index))
            .collect();
        if points.len() >= 3 {
            let mut triangles = self.fan_triangulation(points);
            for triangle in triangles.iter_mut() {
                self.add_face(triangle);
            }
        } else {
            self.ignored += 1;
        }
    }

    fn create_group(&mut self, arguments: &[&'a str]) {
        if let Some(group_name) = arguments.first() {
            if !self.groups.contains_key(group_name) {
                self.groups.insert(
                    group_name,
                    Object::new_group(IDENTITY_MATRIX, Material::default()),
                );
            }
            self.latest_group = group_name;
        } else {
            self.ignored += 1;
        }
    }

    fn add_face(&mut self, face: &mut Object) {
        if let Some(group) = self.groups.get_mut(self.latest_group) {
            group.add_child(face);
        }
    }

    fn fan_triangulation(&self, vertices: Vec<&Point>) -> Vec<Object> {
        let mut triangles = vec![];
        for index in 1..vertices.len() - 1 {
            let tri = self.triangle(vertices[0], vertices[index], vertices[index + 1]);
            triangles.push(tri);
        }
        triangles
    }

    fn triangle(&self, p1: &Point, p2: &Point, p3: &Point) -> Object {
        Object::new_triangle(
            p1.clone(),
            p2.clone(),
            p3.clone(),
            IDENTITY_MATRIX,
            Material::default(),
        )
    }
}

pub fn parse_obj_file<'a>(content: &'a str) -> ObjParser<'a> {
    let mut parser = ObjParser::new(content);
    parser.parse();
    parser
}

#[cfg(test)]
mod test_chapter_15_obj_parser {
    #![allow(non_snake_case)]

    use super::*;

    use crate::shape::*;

    use std::fs;

    fn assert_triangle(t: &Object, vertice1: Point, vertice2: Point, vertice3: Point) {
        match &t.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, vertice1);
                assert_eq!(triangle.p2, vertice2);
                assert_eq!(triangle.p3, vertice3);
            }
            _ => panic!("Object is not a Triangle!"),
        }
    }

    fn group_children(g: &Object) -> Vec<Object> {
        match &g.shape {
            Shape::Group(group) => group.children.clone(),
            _ => panic!("Object is not a group!"),
        }
    }

    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = r##"There was a young lady named Bright
        who traveled much faster than light.
        She set out one day
        in a relative way,
        and came back the previous night."##;
        let parser = parse_obj_file(gibberish);
        assert_eq!(parser.ignored, 5);
    }

    #[test]
    fn vertex_records() {
        let file = r##"v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0"##;
        let parser = parse_obj_file(file);
        assert_eq!(parser.ignored, 0);
        assert_eq!(parser.vertices[1], point(-1.0, 1.0, 0.0));
        assert_eq!(parser.vertices[2], point(-1.0, 0.5, 0.0));
        assert_eq!(parser.vertices[3], point(1.0, 0.0, 0.0));
        assert_eq!(parser.vertices[4], point(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_triangle_faces() {
        let file = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2 3
f 1 3 4"##;
        let parser = parse_obj_file(file);
        let g = parser.default_group().unwrap();
        let children = group_children(g);
        let t1 = &children[0];
        let t2 = &children[1];
        assert_eq!(parser.ignored, 0);
        assert_triangle(
            t1,
            parser.vertices[1],
            parser.vertices[2],
            parser.vertices[3],
        );
        assert_triangle(
            t2,
            parser.vertices[1],
            parser.vertices[3],
            parser.vertices[4],
        );
    }

    #[test]
    fn triangulating_polygons() {
        let file = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5"##;
        let parser = parse_obj_file(file);
        let g = parser.default_group().unwrap();
        let children = group_children(g);
        let t1 = &children[0];
        let t2 = &children[1];
        let t3 = &children[2];
        assert_eq!(parser.ignored, 0);
        assert_triangle(
            t1,
            parser.vertices[1],
            parser.vertices[2],
            parser.vertices[3],
        );
        assert_triangle(
            t2,
            parser.vertices[1],
            parser.vertices[3],
            parser.vertices[4],
        );
        assert_triangle(
            t3,
            parser.vertices[1],
            parser.vertices[4],
            parser.vertices[5],
        );
    }

    #[test]
    fn triangles_in_groups() {
        let file = fs::read_to_string("triangles.obj").unwrap();
        let parser = parse_obj_file(file.as_str());
        let g1 = parser.groups.get("FirstGroup").unwrap();
        let g2 = parser.groups.get("SecondGroup").unwrap();
        let t1 = &group_children(g1)[0];
        let t2 = &group_children(g2)[0];
        assert_triangle(
            t1,
            parser.vertices[1],
            parser.vertices[2],
            parser.vertices[3],
        );
        assert_triangle(
            t2,
            parser.vertices[1],
            parser.vertices[3],
            parser.vertices[4],
        );
    }
}
