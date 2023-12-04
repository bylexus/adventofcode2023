use adventofcode2023::types::Coord2d;

#[test]
fn test_coord2d_add() {
    let coord = Coord2d { x: 1, y: 2 };
    let res = coord.add(&Coord2d { x: 3, y: 4 });
    assert_eq!(res, Coord2d { x: 4, y: 6 });

    let res = coord.add(&Coord2d { x: -3, y: -5 });
    assert_eq!(res, Coord2d { x: -2, y: -3 });
}

#[test]
fn test_coord2d_up() {
    let coord = Coord2d { x: 1, y: 2 };
    let res = coord.up();
    assert_eq!(res, Coord2d { x: 1, y: 1 });

    let res = coord.up_n(5);
    assert_eq!(res, Coord2d { x: 1, y: -3 });
}

#[test]
fn test_coord2d_down() {
    let coord = Coord2d { x: 1, y: 2 };
    let res = coord.down();
    assert_eq!(res, Coord2d { x: 1, y: 3 });

    let res = coord.down_n(5);
    assert_eq!(res, Coord2d { x: 1, y: 7 });
}

#[test]
fn test_coord2d_left() {
    let coord = Coord2d { x: 1, y: 2 };
    let res = coord.left();
    assert_eq!(res, Coord2d { x: 0, y: 2 });

    let res = coord.left_n(5);
    assert_eq!(res, Coord2d { x: -4, y: 2 });
}

#[test]
fn test_coord2d_right() {
    let coord = Coord2d { x: 1, y: 2 };
    let res = coord.right();
    assert_eq!(res, Coord2d { x: 2, y: 2 });

    let res = coord.right_n(5);
    assert_eq!(res, Coord2d { x: 6, y: 2 });
}

#[test]
fn test_coord2d_manhattan_dist() {
    let coord = Coord2d { x: 1, y: 2 };
    let coord2 = Coord2d { x: 5, y: 2 };
    let res = coord.manhattan_dist(&coord2);
    assert_eq!(res, 4);

    let coord2 = Coord2d { x: 1, y: -3 };
    let res = coord.manhattan_dist(&coord2);
    assert_eq!(res, 5);

    let coord2 = Coord2d { x: -8, y: 3 };
    let res = coord.manhattan_dist(&coord2);
    assert_eq!(res, 10);
}
