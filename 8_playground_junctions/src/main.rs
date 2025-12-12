use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(i64, i64, i64);

impl Point {
    fn new(line: &str) -> Self {
        let mut coords = line.trim().split(',').map(|s| s.parse().unwrap());

        Self(
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        )
    }

    fn distance(&self, other: Self) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

fn closest_point_index_and_distance(point: Point, list: &[Point]) -> (usize, f64) {
    list.into_iter()
        .enumerate()
        .filter(|(_, p)| **p != point)
        .map(|(i, p)| (i, p.distance(point)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
}

fn get_sorted_point_list(list: &str) -> Vec<Point> {
    let points: Vec<Point> = list.trim().split('\n').map(Point::new).collect();

    let mut points_and_gaps: Vec<(Point, f64)> = points
        .iter()
        .map(|p| (*p, closest_point_index_and_distance(*p, &points).1))
        .collect();

    points_and_gaps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    points_and_gaps.into_iter().map(|(p, _)| p).collect()
}

fn circuit_group_sizes(connections: &[Option<usize>]) -> Vec<usize> {
    // let mut grouped = HashSet::new();

    let mut group_sizes = Vec::new();

    for index in connections
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|_| i))
    {
        // let mut
    }

    group_sizes
}

fn main() {
    let input = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    let points = get_sorted_point_list(input);

    let mut connections: Vec<Option<usize>> = vec![None; points.len()];

    let mut connections_made = 0;
    let mut i = 0;

    while connections_made < 10 && i < points.len() {
        let closest = closest_point_index_and_distance(points[i], &points).0;

        connections[i] = Some(closest);

        if connections[closest] != Some(i) {
            connections_made += 1;
        }

        i += 1;
    }

    for (p, con) in points
        .iter()
        .zip(connections.into_iter())
        .filter(|pair| pair.1.is_some())
    {
        println!("{:?} -> {:?}", p, points[con.unwrap()]);
    }
}
