use crate::HexOrientation;

pub fn offset_to_cubic(node_coordinates:(i32,i32),orientation:&HexOrientation) -> (i32,i32,i32) {
    match orientation {
        HexOrientation::FlatTopOddUp => {
            let x:i32 = node_coordinates.0;
            let z:i32 = node_coordinates.1 - (node_coordinates.0 - (node_coordinates.0&1))/2;
            let y:i32 = -x - z;
            (x,y,z)
        }
        HexOrientation::FlatTopOddDown => {
            let x:i32 = node_coordinates.0;
            let z:i32 = node_coordinates.1 - (node_coordinates.0 + (node_coordinates.0&1))/2;
            let y:i32 = -x - z;
            (x,y,z)
        }
        HexOrientation::PointyTopOddRight => {
            let x:i32 = node_coordinates.0 - (node_coordinates.1 - (node_coordinates.1&1))/2;
            let z:i32 = node_coordinates.1;
            let y:i32 = -x - z;
            (x,y,z)
        }
        HexOrientation::PointyTopOddLeft => {
            let x:i32 = node_coordinates.0 - (node_coordinates.1 + (node_coordinates.1&1))/2;
            let z:i32 = node_coordinates.1;
            let y:i32 = -x - z;
            (x,y,z)
        }
    }
}

pub fn axial_to_cubic(node_coordinates:(i32,i32)) -> (i32,i32,i32) {
    let x = node_coordinates.0;
    let z = node_coordinates.1;
    let y:i32 = -x - z;
    (x,y,z)
}

pub fn axial_to_offset(node_coordinates:(i32,i32),orientation:&HexOrientation) -> (i32,i32) {
    match orientation {
        HexOrientation::FlatTopOddUp => {
            let x:i32 = node_coordinates.0;
            let y:i32 = node_coordinates.1 + (node_coordinates.0 - (node_coordinates.0&1))/2;
            (x,y)
        }
        HexOrientation::FlatTopOddDown => {
            let x:i32 = node_coordinates.0;
            let y:i32 = node_coordinates.1 + (node_coordinates.0 + (node_coordinates.0&1))/2;
            (x,y)
        }
        HexOrientation::PointyTopOddRight => {
            let x:i32 = node_coordinates.0 + (node_coordinates.0 - (node_coordinates.1&1))/2;
            let y:i32 = node_coordinates.1;
            (x,y)
        }
        HexOrientation::PointyTopOddLeft => {
            let x:i32 = node_coordinates.0 + (node_coordinates.0 + (node_coordinates.1&1))/2;
            let y:i32 = node_coordinates.1;
            (x,y)
        }
    }
}

pub fn cubic_to_axial(node_coordinates:(i32,i32,i32)) -> (i32,i32) {
    let q = node_coordinates.0;
    let r = node_coordinates.2;
    (q,r)
}

pub fn cubic_to_offset(node_coordinates:(i32,i32,i32),orientation:&HexOrientation) -> (i32,i32) {
    match orientation {
        HexOrientation::FlatTopOddUp => {
            let q:i32 = node_coordinates.0;
            let r:i32 = node_coordinates.2 + (node_coordinates.0 - (node_coordinates.0&1))/2;
            (q,r)
        }
        HexOrientation::FlatTopOddDown => {
            let q:i32 = node_coordinates.0;
            let r:i32 = node_coordinates.2 + (node_coordinates.0 + (node_coordinates.0&1))/2;
            (q,r)
        }
        HexOrientation::PointyTopOddRight => {
            let q:i32 = node_coordinates.0 + (node_coordinates.2 - (node_coordinates.2&1))/2;
            let r:i32 = node_coordinates.2;
            (q,r)
        }
        HexOrientation::PointyTopOddLeft => {
            let q:i32 = node_coordinates.0 + (node_coordinates.2 + (node_coordinates.2&1))/2;
            let r:i32 = node_coordinates.2;
            (q,r)
        }
    }
}


pub fn node_neighbours_offset(
    source:(i32,i32),
    orientation:&HexOrientation,
    min_column:i32,
    max_column:i32,
    min_row:i32,
    max_row:i32
) -> Vec<(i32,i32)> {
    let mut neighbours = Vec::new();
    match orientation {
        HexOrientation::FlatTopOddUp => {
            if source.0 & 1 == 0 {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1));
                };
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1));
                };
                if source.0 + 1 < max_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 + 1, source.1 - 1));
                };
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1));
                };
                if source.0 - 1 > min_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 - 1, source.1 - 1));
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1));
                }
            } else {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1));
                }
                if source.0 + 1 < max_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 + 1, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1));
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1));
                }
                if source.0 - 1 < max_column {
                    neighbours.push((source.0 - 1, source.1));
                }
                if source.0 - 1 < max_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 - 1, source.1 + 1))
                }
            }
        }
        HexOrientation::FlatTopOddDown => {
            if source.0 & 1 == 0 {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1));
                }
                if source.0 + 1 < max_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 + 1, source.1 + 1));
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1));
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1));
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1));
                }
                if source.0 - 1 > min_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 - 1, source.1 + 1));
                }
            } else {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1))
                }
                if source.0 + 1 < max_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 + 1, source.1 - 1))
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1))
                }
                if source.0 - 1 > min_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 - 1, source.1 - 1))
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1))
                }
            }
        }
        HexOrientation::PointyTopOddRight => {
            if source.1 & 1 == 0 {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1))
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1))
                }
                if source.0 - 1 > min_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 - 1, source.1 - 1))
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1))
                }
                if source.0 - 1 > min_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 - 1, source.1 + 1))
                }
            } else {
                if source.0 + 1 < max_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 + 1, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1))
                }
                if source.0 + 1 < max_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 + 1, source.1 - 1))
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1))
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1))
                }
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1))
                }
            }
        }
        HexOrientation::PointyTopOddLeft => {
            if source.1 & 1 == 0 {
                if source.0 + 1 < max_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 + 1, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1))
                }
                if source.0 + 1 < max_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 + 1, source.1 - 1))
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1))
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1))
                }
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1))
                }
            } else {
                if source.1 + 1 < max_row {
                    neighbours.push((source.0, source.1 + 1))
                }
                if source.0 + 1 < max_column {
                    neighbours.push((source.0 + 1, source.1))
                }
                if source.1 - 1 > min_row {
                    neighbours.push((source.0, source.1 - 1))
                }
                if source.0 - 1 > min_column && source.1 - 1 > min_row {
                    neighbours.push((source.0 - 1, source.1 - 1))
                }
                if source.0 - 1 > min_column {
                    neighbours.push((source.0 - 1, source.1))
                }
                if source.0 - 1 > min_column && source.1 + 1 < max_row {
                    neighbours.push((source.0 - 1, source.1 + 1))
                }
            }
        }
    }
    neighbours
}

pub fn node_neighbours_cubic(source:(i32,i32,i32),count_rings_from_origin:i32) -> Vec<(i32,i32,i32)> {
    let mut neighbours = Vec::new();
    if (source.1 - 1).abs() <= count_rings_from_origin
        && (source.2 + 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0, source.1 - 1, source.2 + 1))
    }
    if (source.0 + 1).abs() <= count_rings_from_origin
        && (source.1 - 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0 + 1, source.1 - 1, source.2))
    }
    if (source.0 + 1).abs() <= count_rings_from_origin
        && (source.2 - 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0 + 1, source.1, source.2 - 1))
    }
    if (source.1 + 1).abs() <= count_rings_from_origin
        && (source.2 - 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0, source.1 + 1, source.2 - 1))
    }
    if (source.0 - 1).abs() <= count_rings_from_origin
        && (source.1 + 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0 - 1, source.1 + 1, source.2))
    }
    if (source.0 - 1).abs() <= count_rings_from_origin
        && (source.2 + 1).abs() <= count_rings_from_origin {
        neighbours.push((source.0 - 1, source.1, source.2 + 1))
    }
    neighbours
}

pub fn node_neighbours_axial(source:(i32,i32),count_rings_from_origin:i32) -> Vec<(i32,i32)> {
    let mut neighbours = Vec::new();
    let cubic = axial_to_cubic(source);
    let node = node_neighbours_cubic(cubic,count_rings_from_origin);
    for i in node.iter() {
        neighbours.push(cubic_to_axial(*i));
    }
    neighbours
}

pub fn node_ring_cubic(source:(i32,i32,i32),radius:i32) -> Vec<(i32,i32,i32)> {
    let mut ring_nodes = Vec::new();
    let cube_directions = [
        (0, -1, 1),
        (1, -1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (-1, 1, 0),
        (-1, 0, 1),
    ];
    let scaled_x = cube_directions[4].0 * radius;
    let scaled_y = cube_directions[4].1 * radius;
    let scaled_z = cube_directions[4].2 * radius;
    let mut ring_node_current = (source.0 * scaled_x,source.1 * scaled_y,source.2 * scaled_z);
    for i in 0 .. 6 {
        for _j in 0 .. radius {
            ring_node_current.0 += cube_directions[i].0;
            ring_node_current.1 += cube_directions[i].1;
            ring_node_current.2 += cube_directions[i].2;
            ring_nodes.push(ring_node_current);
        }
    }
    ring_nodes
}

pub fn node_distance(start:(i32,i32,i32),end:(i32,i32,i32)) -> i32 {
    ((start.0 - end.0).abs() + (start.1 - end.1).abs() + (start.2 - end.2).abs())/2
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn flat_top_odd_up_even_node_neighbours(){
        let source:(i32,i32) = (2,2);
        let orientation = HexOrientation::FlatTopOddUp;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let actual = vec![(2, 3), (3, 2), (3, 1), (2, 1), (1, 1), (1, 2)];
        assert_eq!(actual, neighbours);
    }

    #[test]
    fn flat_top_odd_up_odd_node_neighbours() {
        let source:(i32,i32) = (3,2);
        let orientation = HexOrientation::FlatTopOddDown;
        let min_column = -1;
        let max_column = 5;
        let min_row = -1;
        let max_row = 5;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let actual = vec![(3, 3), (4, 3), (4, 2), (3, 1), (2, 2), (2, 3)];
        assert_eq!(actual, neighbours);
    }

    #[test]
    fn flat_top_odd_down_even_node_neighbours() {
        let source:(i32,i32) = (2,2);
        let orientation = HexOrientation::FlatTopOddUp;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let actual = vec![(2, 3), (3, 3), (3, 2), (2, 1), (1, 2), (1, 3)];
        assert_eq!(actual, neighbours);
    }

    #[test]
    fn flat_top_odd_down_odd_node_neighbours() {
        let source:(i32,i32) = (3,2);
        let orientation = HexOrientation::FlatTopOddUp;
        let min_column = -1;
        let max_column = 5;
        let min_row = -1;
        let max_row = 5;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let actual = vec![(3, 3), (4, 2), (4, 1), (3, 1), (2, 1), (2, 2)];
        assert_eq!(actual, neighbours);
    }

    #[test]
    fn offset_neighbors_along_negative_boundary() {
        let source:(i32,i32) = (0,0);
        let orientation = HexOrientation::FlatTopOddUp;
        let min_column = -1;
        let max_column = 5;
        let min_row = -1;
        let max_row = 5;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let expected_neighbour_count = 2;
        assert_eq!(expected_neighbour_count, neighbours.len());
    }

    #[test]
    fn offset_neighbors_along_positive_boundary() {
        let source:(i32,i32) = (2,1);
        let orientation = HexOrientation::FlatTopOddUp;
        let min_column = -1;
        let max_column = 3;
        let min_row = -1;
        let max_row = 3;
        let neighbours = node_neighbours_offset(source,&orientation,min_column,max_column,min_row,max_row);
        let expected_neighbour_count = 3;
        assert_eq!(expected_neighbour_count, neighbours.len());
    }

    #[test]
    fn pointy_top_odd_right_even_node_neighbours() {
        let source: (i32, i32) = (2, 2);
        let orientation = HexOrientation::PointyTopOddRight;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(
            source,
            &orientation,
            min_column,
            max_column,
            min_row,
            max_row,
        );
        let actual = vec![(2, 3), (3, 2), (2, 1), (1, 1), (1, 2), (1, 3)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn pointy_top_odd_right_odd_node_neighbours() {
        let source: (i32, i32) = (1, 1);
        let orientation = HexOrientation::PointyTopOddRight;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(
            source,
            &orientation,
            min_column,
            max_column,
            min_row,
            max_row,
        );
        let actual = vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 1), (1, 2)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn pointy_top_odd_left_even_node_neighbours() {
        let source: (i32, i32) = (2, 2);
        let orientation = HexOrientation::PointyTopOddLeft;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(
            source,
            &orientation,
            min_column,
            max_column,
            min_row,
            max_row,
        );
        let actual = vec![(3, 3), (3, 2), (3, 1), (2, 1), (1, 2), (2, 3)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn pointy_top_odd_left_odd_node_neighbours() {
        let source: (i32, i32) = (1, 1);
        let orientation = HexOrientation::PointyTopOddLeft;
        let min_column = -1;
        let max_column = 4;
        let min_row = -1;
        let max_row = 4;
        let neighbours = node_neighbours_offset(
            source,
            &orientation,
            min_column,
            max_column,
            min_row,
            max_row,
        );
        let actual = vec![(1, 2), (2, 1), (1, 0), (0, 0), (0, 1), (0, 2)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn axial_to_cubic_cords() {
        let axial: (i32, i32) = (2, 1);
        let cubic = axial_to_cubic(axial);
        assert_eq!((2, -3, 1), cubic);
    }
    #[test]
    fn cubic_to_axial_cords() {
        let cubic: (i32, i32, i32) = (1, -2, 1);
        let axial = cubic_to_axial(cubic);
        assert_eq!((1, 1), axial);
    }
    #[test]
    fn axial_neighbours() {
        let source: (i32, i32) = (2, -1);
        let neighbours = node_neighbours_axial(source, 3);
        let actual = vec![(2, 0), (3, -1), (3, -2), (2, -2), (1, -1), (1, 0)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn axial_neighbours_with_boundary() {
        let source: (i32, i32) = (-1, -1);
        let neighbours = node_neighbours_axial(source, 2);
        let actual = vec![(-1, 0), (0, -1), (0, -2), (-2, 0)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn cubic_neighbours() {
        let source: (i32, i32, i32) = (2, -1, -1);
        let neighbours = node_neighbours_cubic(source, 3);
        let actual = vec![
            (2, -2, 0),
            (3, -2, -1),
            (3, -1, -2),
            (2, 0, -2),
            (1, 0, -1),
            (1, -1, 0),
        ];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn cubic_neighbours_with_boundary() {
        let source: (i32, i32, i32) = (2, -1, -1);
        let neighbours = node_neighbours_cubic(source, 2);
        let actual = vec![(2, -2, 0), (2, 0, -2), (1, 0, -1), (1, -1, 0)];
        assert_eq!(actual, neighbours);
    }
    #[test]
    fn convert_axial_to_offset_odd_up() {
        let source: (i32, i32) = (-1, -1);
        let result = axial_to_offset(source, &HexOrientation::FlatTopOddUp);
        let actual: (i32, i32) = (-1, -2);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_axial_to_offset_odd_down() {
        let source: (i32, i32) = (-1, -1);
        let result = axial_to_offset(source, &HexOrientation::FlatTopOddDown);
        let actual: (i32, i32) = (-1, -1);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_axial_to_offset_odd_right() {
        let source: (i32, i32) = (-1, -1);
        let result = axial_to_offset(source, &HexOrientation::PointyTopOddRight);
        let actual: (i32, i32) = (-2, -1);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_axial_to_offset_odd_left() {
        let source: (i32, i32) = (-1, -1);
        let result = axial_to_offset(source, &HexOrientation::PointyTopOddLeft);
        let actual: (i32, i32) = (-1, -1);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_cubic_to_offset_odd_up() {
        let source: (i32, i32, i32) = (-2, 3, -1);
        let result = cubic_to_offset(source, &HexOrientation::FlatTopOddUp);
        let actual: (i32, i32) = (-2, -2);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_cubic_to_offset_odd_up_two() {
        let source: (i32, i32, i32) = (6, -14, 8);
        let result = cubic_to_offset(source, &HexOrientation::FlatTopOddUp);
        let actual: (i32, i32) = (6, 11);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_cubic_to_offset_odd_down() {
        let source: (i32, i32, i32) = (-1, 1, 0);
        let result = cubic_to_offset(source, &HexOrientation::FlatTopOddDown);
        let actual: (i32, i32) = (-1, 0);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_cubic_to_offset_odd_right() {
        let source: (i32, i32, i32) = (1, -2, 1);
        let result = cubic_to_offset(source, &HexOrientation::PointyTopOddRight);
        let actual: (i32, i32) = (1, 1);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_cubic_to_offset_odd_left() {
        let source: (i32, i32, i32) = (1, -2, 1);
        let result = cubic_to_offset(source, &HexOrientation::PointyTopOddLeft);
        let actual: (i32, i32) = (2, 1);
        assert_eq!(actual, result);
    }
    #[test]
    fn ring_1() {
        let source = (0, 0, 0);
        let radius = 1;
        let result = node_ring_cubic(source, radius);
        let actual = vec![
            (-1, 0, 1),
            (0, -1, 1),
            (1, -1, 0),
            (1, 0, -1),
            (0, 1, -1),
            (-1, 1, 0),
        ];
        assert_eq!(actual, result);
    }
    #[test]
    fn ring_2() {
        let source = (0, 0, 0);
        let radius = 2;
        let result = node_ring_cubic(source, radius);
        let actual = vec![
            (-2, 1, 1),
            (-2, 0, 2),
            (-1, -1, 2),
            (0, -2, 2),
            (1, -2, 1),
            (2, -2, 0),
            (2, -1, -1),
            (2, 0, -2),
            (1, 1, -2),
            (0, 2, -2),
            (-1, 2, -1),
            (-2, 2, 0),
        ];
        assert_eq!(actual, result);
    }
    #[test]
    fn ring_3() {
        let source = (0, 0, 0);
        let radius = 3;
        let result = node_ring_cubic(source, radius);
        let actual = vec![
            (-3, 2, 1),
            (-3, 1, 2),
            (-3, 0, 3),
            (-2, -1, 3),
            (-1, -2, 3),
            (0, -3, 3),
            (1, -3, 2),
            (2, -3, 1),
            (3, -3, 0),
            (3, -2, -1),
            (3, -1, -2),
            (3, 0, -3),
            (2, 1, -3),
            (1, 2, -3),
            (0, 3, -3),
            (-1, 3, -2),
            (-2, 3, -1),
            (-3, 3, 0),
        ];
        assert_eq!(actual, result);
    }
    #[test]
    /// test for nodes on ring 3
    fn ring_3_two() {
        let source = (9, -14, 5);
        let radius = 3;
        let result = node_ring_cubic(source, radius);
        let actual = vec![
            (6, -12, 6),
            (6, -13, 7),
            (6, -14, 8),
            (7, -15, 8),
            (8, -16, 8),
            (9, -17, 8),
            (10, -17, 7),
            (11, -17, 6),
            (12, -17, 5),
            (12, -16, 4),
            (12, -15, 3),
            (12, -14, 2),
            (11, -13, 2),
            (10, -12, 2),
            (9, -11, 2),
            (8, -11, 3),
            (7, -11, 4),
            (6, -11, 5),
        ];
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_offset_to_cubic_flat_top_odd_up() {
        let source: (i32, i32) = (9, 9);
        let result = offset_to_cubic(source, &HexOrientation::FlatTopOddUp);
        let actual: (i32, i32, i32) = (9, -14, 5);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_offset_to_cubic_flat_top_odd_up_two() {
        let source: (i32, i32) = (6, 11);
        let result = offset_to_cubic(source, &HexOrientation::FlatTopOddUp);
        let actual: (i32, i32, i32) = (6, -14, 8);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_offset_to_cubic_flat_top_odd_down() {
        let source: (i32, i32) = (9, 9);
        let result = offset_to_cubic(source, &HexOrientation::FlatTopOddDown);
        let actual: (i32, i32, i32) = (9, -13, 4);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_offset_to_cubic_pointy_top_odd_right() {
        let source: (i32, i32) = (1, 1);
        let result = offset_to_cubic(source, &HexOrientation::PointyTopOddRight);
        let actual: (i32, i32, i32) = (1, -2, 1);
        assert_eq!(actual, result);
    }
    #[test]
    fn convert_offset_to_cubic_pointy_top_odd_left() {
        let source: (i32, i32) = (1, 1);
        let result = offset_to_cubic(source, &HexOrientation::PointyTopOddLeft);
        let actual: (i32, i32, i32) = (0, -1, 1);
        assert_eq!(actual, result);
    }
}