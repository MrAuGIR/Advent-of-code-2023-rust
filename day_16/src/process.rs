use std::{collections::{HashSet, VecDeque}, iter::Inspect};

use array2d::Array2D;

use crate::component::{Point, Position, Direction};


pub fn process_part_one(input: &Array2D<char>, start_point: Point) -> Option<usize>{

    let mut visited_positions: HashSet<Position> = HashSet::new();
    let mut seen_node: HashSet<Point> = HashSet::new();

    // init de la pile
    let mut move_stack: VecDeque<Point> = VecDeque::new();

    move_stack.push_back(start_point.clone());
    seen_node.insert(start_point);

    while let Some(node) = move_stack.pop_front() {

        let next_signal_start = make_signal_path(node, input, &mut visited_positions);
        
        if next_signal_start.len() == 2 {
            for p in next_signal_start {
                if !seen_node.contains(&p) {
                    seen_node.insert(p.clone());
                    move_stack.push_back(p.clone());
                }
            }
        }
    }

    Some(visited_positions.iter().len())
}

pub fn process_part_two(input: &Array2D<char>) -> Option<usize> {

    // On parcoure tous les points de depart possibles
    let last_index_x  = input.column_len() - 1;
    let last_index_y = input.row_len() - 1;

    let mut starts: Vec<Point> = Vec::new();
    let mut most_energizes = 0;

    for (y,_) in input.as_rows().iter().enumerate() {

        let left_start = get_current_point(0, y, input, Direction::Right);
        let right_start = get_current_point(last_index_x, y, input, Direction::Left);
        starts.push(left_start.clone());
        starts.push(right_start.clone());
    }

    for (x,_) in input.as_columns().iter().enumerate() {
        let top_start = get_current_point(x, 0, input, Direction::Down);
        let bottom_start = get_current_point(x, last_index_y,input, Direction::Up);
        starts.push(top_start.clone());
        starts.push(bottom_start.clone());
    }

    for start in starts {

        if let Some(energizes) = process_part_one(&input, start) {
            if energizes > most_energizes {
                most_energizes = energizes;
            }
        }
    }
    Some(most_energizes)
}

fn get_current_point(x: usize, y:usize, map: &Array2D<char>, direction: Direction) -> Point {
    Point {
        x,
        y,
        c: map.get(y,0).unwrap().to_ascii_lowercase(),
        direction: direction
    }
}

fn make_signal_path(position: Point, map: &Array2D<char>, visited_positions: &mut HashSet<Position>) -> Vec<Point> {
    
    let mut current_point = Point {x: position.x, y: position.y, c: position.c, direction: position.direction};

    let mut seen_node: HashSet<Position> = HashSet::new();
    let mut prev_len = seen_node.len();

    loop {
        visited_positions.insert(Position {x: current_point.x, y: current_point.y});

        let (next_points, is_split) = move_signal(current_point.clone(), &map);

        if next_points.len() == 2 || next_points.is_empty() {
            return next_points;
        }

        for p in next_points.clone() {
            visited_positions.insert(Position { x: p.x, y: p.y});
            seen_node.insert(Position {x: p.x, y: p.y});
        }

        if is_split && seen_node.len() == prev_len {
            return next_points;
        }

        prev_len = seen_node.len();
        current_point = next_points[0].clone();

    }
}

fn move_signal(point: Point, map: &Array2D<char>) -> (Vec<Point>, bool){
    let Point {x,y,c,direction} = point;

    let checked_x_sub = x.checked_sub(1);
    let checked_y_sub = y.checked_sub(1);

    let right = map.get(y,x+1).map(|c| Point{x: x+1, y, c: *c, direction: Direction::Right});
    let left = checked_x_sub.and_then(|x| map.get(y, x)).map(|c| Point{x: x - 1,y,c: *c, direction: Direction::Left});

    let up = checked_y_sub.and_then(|y| map.get(y, x)).map(|c| Point{x, y: y - 1, c: *c, direction: Direction::Up});
    let down = map.get(y+1,x).map(|c| Point{x, y: y+1, c: *c, direction: Direction::Down});

    let pass_through_right = right.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_left = left.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_up = up.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_down = down.iter().map(|v| v.clone()).collect::<Vec<_>>();

    let mut split_vertical = pass_through_up.clone();
    split_vertical.append(&mut pass_through_down.clone());
    let mut split_horizontal = pass_through_right.clone();
    split_horizontal.append(&mut pass_through_left.clone());

    match c {
        '.' => match direction {
            Direction::Right => (pass_through_right,false),
            Direction::Left => (pass_through_left,false),
            Direction::Up => (pass_through_up,false),
            Direction::Down => (pass_through_down,false),
        },
        '|' => match direction {
            Direction::Right => (split_vertical,true),
            Direction::Left => (split_vertical,true),
            Direction::Up => (pass_through_up,false),
            Direction::Down => (pass_through_down,false),
        },
        '-' => match direction {
            Direction::Right => (pass_through_right,false),
            Direction::Left => (pass_through_left,false),
            Direction::Up => (split_horizontal,true),
            Direction::Down => (split_horizontal,true),
        },
        '/' => match direction {
            Direction::Right => (pass_through_up,false),
            Direction::Left => (pass_through_down,false),
            Direction::Up => (pass_through_right,false),
            Direction::Down => (pass_through_left,false),
        },
        '\\' => match direction {
            Direction::Right => (pass_through_down,false),
            Direction::Left => (pass_through_up,false),
            Direction::Up => (pass_through_left,false),
            Direction::Down => (pass_through_right,false),
        }
        _ => {
            println!("aucun match pour {:?}",c);
            (vec![],false)
        }
    }
}