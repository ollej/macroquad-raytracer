use crate::{color::*, float::*, material::*, matrix::IDENTITY_MATRIX, object::*, tuple::*};

use std::collections::HashMap;

struct Face {
    vertice: Point,
    normal: Option<Vector>,
}

impl Face {
    fn new(vertice: Point) -> Self {
        Self {
            vertice,
            normal: None,
        }
    }

    fn with_normal(vertice: Point, normal: Vector) -> Self {
        Self {
            vertice,
            normal: Some(normal),
        }
    }

    fn has_normal(&self) -> bool {
        self.normal.is_some()
    }
}

pub struct ObjParser<'a> {
    content: &'a str,
    pub ignored: usize,
    pub vertices: Vec<Point>,
    pub groups: HashMap<&'a str, Object>,
    pub normals: Vec<Vector>,
    material: Material,
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
            normals: vec![Vector::empty_vector()],
            material: Material::default(),
            latest_group: Self::DEFAULT_GROUP,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn parse(&mut self) {
        self.content.lines().for_each(|line| self.parse_line(line));
    }

    pub fn default_group(&self) -> Option<&Object> {
        self.groups.get(Self::DEFAULT_GROUP)
    }

    pub fn obj_to_group(&self) -> Object {
        let mut object = Object::new_group(IDENTITY_MATRIX, Material::default());
        for (_name, group) in self.groups.iter() {
            let group = &mut group.clone();
            object.add_child(group);
        }

        object
    }

    fn parse_line(&mut self, line: &'a str) {
        let values = line.split_whitespace().collect::<Vec<&'a str>>();
        match values.split_first() {
            Some((&"v", arguments)) => self.parse_vertice(arguments),
            Some((&"f", arguments)) => self.parse_face(arguments),
            Some((&"g", arguments)) => self.create_group(arguments),
            Some((&"vn", arguments)) => self.parse_normal(arguments),
            _ => {
                //println!("Ignored line: {:?}", values);
                self.ignored += 1;
            }
        }
    }

    fn parse_vertice(&mut self, arguments: &[&'a str]) {
        //println!("vertice: {:?}", arguments);
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
        //println!("face: {:?}", arguments);
        if arguments.len() >= 3 {
            let mut vertices_and_normals = vec![];
            for arg in arguments.iter() {
                //println!("Arguments: {arg}");
                let s: Vec<Option<usize>> = arg
                    .split("/")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|f| f.parse::<usize>().ok())
                    .collect();
                match &s[..] {
                    // Matches faces in the format v/vt/vn
                    &[Some(vidx), _, Some(nidx)] => {
                        let v = self.vertices.get(vidx);
                        let n = self.normals.get(nidx);
                        if v.is_some() && n.is_some() {
                            //println!("Vertice with normal: {vidx} : {v:?} / {nidx} : {n:?}");
                            vertices_and_normals.push(Face::with_normal(*v.unwrap(), *n.unwrap()));
                        }
                    }
                    // Matches face with only v
                    &[Some(v)] => {
                        let face = self.vertices.get(v).map(|v| Face::new(*v));
                        if face.is_some() {
                            //println!("Vertice without normal: {v:?}");
                            vertices_and_normals.push(face.unwrap());
                        }
                    }
                    _ => (),
                };
            }

            let mut triangles = self.fan_triangulation(vertices_and_normals);
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

    fn parse_normal(&mut self, arguments: &[&'a str]) {
        //println!("vertice: {:?}", arguments);
        let numbers: Option<Vec<Float>> =
            arguments.iter().map(|f| f.parse::<Float>().ok()).collect();
        if let Some(p) = numbers {
            if p.len() == 3 {
                self.normals.push(Vector::vector(p[0], p[1], p[2]));
                return;
            }
        }
        self.ignored += 1;
    }

    fn add_face(&mut self, face: &mut Object) {
        if let Some(group) = self.groups.get_mut(self.latest_group) {
            group.add_child(face);
        }
    }

    fn fan_triangulation(&self, vertices: Vec<Face>) -> Vec<Object> {
        let mut triangles = vec![];
        for index in 1..vertices.len() - 1 {
            let tri = if vertices[0].has_normal()
                && vertices[index].has_normal()
                && vertices[index + 1].has_normal()
            {
                self.smooth_triangle(
                    vertices[0].vertice,
                    vertices[index].vertice,
                    vertices[index + 1].vertice,
                    vertices[0].normal.unwrap(),
                    vertices[index].normal.unwrap(),
                    vertices[index + 1].normal.unwrap(),
                )
            } else {
                self.triangle(
                    vertices[0].vertice,
                    vertices[index].vertice,
                    vertices[index + 1].vertice,
                )
            };
            triangles.push(tri);
        }
        triangles
    }

    fn triangle(&self, p1: Point, p2: Point, p3: Point) -> Object {
        Object::new_triangle(p1, p2, p3, IDENTITY_MATRIX, self.material)
    }

    fn smooth_triangle(
        &self,
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> Object {
        Object::new_smooth_triangle(p1, p2, p3, n1, n2, n3, IDENTITY_MATRIX, self.material)
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

    fn assert_triangle_with_normals(
        t: &Object,
        vertice1: Point,
        vertice2: Point,
        vertice3: Point,
        normal1: Vector,
        normal2: Vector,
        normal3: Vector,
    ) {
        match &t.shape {
            Shape::SmoothTriangle(triangle) => {
                assert_eq!(triangle.p1, vertice1);
                assert_eq!(triangle.p2, vertice2);
                assert_eq!(triangle.p3, vertice3);
                assert_eq!(triangle.n1, normal1);
                assert_eq!(triangle.n2, normal2);
                assert_eq!(triangle.n3, normal3);
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

    #[test]
    fn converting_an_OBJ_file_to_a_group() {
        let file = fs::read_to_string("triangles.obj").unwrap();
        let parser = parse_obj_file(file.as_str());
        let g = parser.obj_to_group();
        let children = &group_children(&g);
        let g1 = parser.groups.get("FirstGroup").unwrap();
        let g2 = parser.groups.get("SecondGroup").unwrap();
        assert_eq!(parser.ignored, 0);
        assert!(children.contains(g1));
        assert!(children.contains(g2));
    }

    #[test]
    fn vertex_normal_records() {
        let file = r##"vn 0 0 1
        vn 0.707 0 -0.707
        vn 1 2 3"##;
        let parser = parse_obj_file(file);
        assert_eq!(parser.ignored, 0);
        assert_eq!(parser.normals[1], vector(0.0, 0.0, 1.0));
        assert_eq!(parser.normals[2], vector(0.707, 0.0, -0.707));
        assert_eq!(parser.normals[3], vector(1.0, 2.0, 3.0));
    }

    #[test]
    fn faces_with_normals() {
        let file = r##"v 0 1 0
v -1 0 0
v 1 0 0
vn -1 0 0
vn 1 0 0
vn 0 1 0
f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2"##;
        let parser = parse_obj_file(file);
        let g = parser.default_group().unwrap();
        let children = group_children(g);
        let t1 = &children[0];
        let t2 = &children[1];
        assert_eq!(parser.ignored, 0);
        assert_triangle_with_normals(
            t1,
            parser.vertices[1],
            parser.vertices[2],
            parser.vertices[3],
            parser.normals[3],
            parser.normals[1],
            parser.normals[2],
        );
        assert_eq!(t2, t1);
    }
}
