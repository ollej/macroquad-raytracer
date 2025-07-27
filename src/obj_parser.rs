use crate::{float::*, material::*, matrix::IDENTITY_MATRIX, object::*, shape::*, tuple::*};

pub struct ObjParser {
    content: &'static str,
    pub ignored: usize,
    pub vertices: Vec<Point>,
    pub default_group: Object,
}

impl ObjParser {
    pub fn new(content: &'static str) -> Self {
        Self {
            content,
            ignored: 0,
            vertices: vec![Point::empty_point()],
            default_group: Object::new_group(IDENTITY_MATRIX, Material::default()),
        }
    }

    pub fn parse(&mut self) {
        self.content.lines().for_each(|line| self.parse_line(line));
    }

    fn parse_line(&mut self, line: &'static str) {
        let values = line.split(" ").collect::<Vec<&'static str>>();
        match values.split_first() {
            Some((&"v", arguments)) => self.parse_vertice(arguments),
            Some((&"f", arguments)) => self.parse_face(arguments),
            _ => self.ignored += 1,
        }
    }

    fn parse_vertice(&mut self, arguments: &[&'static str]) {
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

    fn parse_face(&mut self, arguments: &[&'static str]) {
        let points: Vec<&Point> = arguments
            .iter()
            .flat_map(|f| f.parse::<usize>().ok())
            .flat_map(|index| self.vertices.get(index))
            .collect();
        if points.len() >= 3 {
            let mut triangles = self.fan_triangulation(points);
            for triangle in triangles.iter_mut() {
                self.default_group.add_child(triangle);
            }
        } else {
            self.ignored += 1;
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

pub fn parse_obj_file(content: &'static str) -> ObjParser {
    let mut parser = ObjParser::new(content);
    parser.parse();
    parser
}

#[cfg(test)]
mod test_chapter_15_obj_parser {
    #![allow(non_snake_case)]

    use super::*;

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
        let g = parser.default_group;
        let children = match g.shape {
            Shape::Group(group) => group.children,
            _ => panic!("Object is not a group!"),
        };
        let t1 = children.get(0).unwrap();
        let t2 = children.get(1).unwrap();
        assert_eq!(parser.ignored, 0);
        match &t1.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, parser.vertices[1]);
                assert_eq!(triangle.p2, parser.vertices[2]);
                assert_eq!(triangle.p3, parser.vertices[3]);
            }
            _ => panic!("Object is not a Triangle!"),
        }
        match &t2.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, parser.vertices[1]);
                assert_eq!(triangle.p2, parser.vertices[3]);
                assert_eq!(triangle.p3, parser.vertices[4]);
            }
            _ => panic!("Object is not a Triangle!"),
        }
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
        let g = parser.default_group;
        let children = match g.shape {
            Shape::Group(group) => group.children,
            _ => panic!("Object is not a group!"),
        };
        let t1 = children.get(0).unwrap();
        let t2 = children.get(1).unwrap();
        let t3 = children.get(2).unwrap();
        assert_eq!(parser.ignored, 0);
        match &t1.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, parser.vertices[1]);
                assert_eq!(triangle.p2, parser.vertices[2]);
                assert_eq!(triangle.p3, parser.vertices[3]);
            }
            _ => panic!("Object is not a Triangle!"),
        }
        match &t2.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, parser.vertices[1]);
                assert_eq!(triangle.p2, parser.vertices[3]);
                assert_eq!(triangle.p3, parser.vertices[4]);
            }
            _ => panic!("Object is not a Triangle!"),
        }
        match &t3.shape {
            Shape::Triangle(triangle) => {
                assert_eq!(triangle.p1, parser.vertices[1]);
                assert_eq!(triangle.p2, parser.vertices[4]);
                assert_eq!(triangle.p3, parser.vertices[5]);
            }
            _ => panic!("Object is not a Triangle!"),
        }
    }
}
