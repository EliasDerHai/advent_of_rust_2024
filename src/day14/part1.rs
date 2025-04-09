use crate::util::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Robot {
    start_pos: Point,
    velocity: Point,
}

impl Robot {
    pub(crate) fn project_pos(
        &self,
        iterations: u32,
        canvas_width: u32,
        canvas_height: u32,
    ) -> Point {
        let raw = self.start_pos + (self.velocity * iterations as i32);
        let bounded_x = raw.x.rem_euclid(canvas_width as i32);
        let bounded_y = raw.y.rem_euclid(canvas_height as i32);
        Point::new(bounded_x, bounded_y)
    }

    fn new(start_pos: Point, velocity: Point) -> Self {
        Robot {
            start_pos,
            velocity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Lobby {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) robots: Vec<Robot>,
}
impl TryFrom<(&str, u32, u32)> for Lobby {
    type Error = String;

    fn try_from(value: (&str, u32, u32)) -> Result<Self, Self::Error> {
        let (input, width, height) = value;
        let robots: Vec<Robot> = input
            .lines()
            .filter_map(|mut l| {
                l = l.trim();
                if !l.starts_with("p=") {
                    return None;
                }

                let (pos_info, velocity_info) = l.split_once(" ").unwrap();
                let pos_info = pos_info[2..].split_once(",").unwrap();
                let velocity_info = velocity_info[2..].split_once(",").unwrap();
                let pos_info = Point::new(
                    pos_info.0.parse::<i32>().unwrap(),
                    pos_info.1.parse::<i32>().unwrap(),
                );
                let velocity_info = Point::new(
                    velocity_info.0.parse::<i32>().unwrap(),
                    velocity_info.1.parse::<i32>().unwrap(),
                );
                Some(Robot::new(pos_info, velocity_info))
            })
            .collect();

        Ok(Lobby {
            width,
            height,
            robots,
        })
    }
}

pub fn solve_day_14_part_01(
    input: &str,
    canvas_width: u32,
    canvas_height: u32,
    iterations: u32,
) -> u32 {
    let lobby = Lobby::try_from((input, canvas_width, canvas_height)).unwrap();

    let positions_at_target_time: Vec<Point> = lobby
        .robots
        .into_iter()
        .map(|r| Robot::project_pos(&r, iterations, lobby.width, lobby.height))
        .collect();

    let half_width: i32 = (canvas_width / 2) as i32;
    let half_height: i32 = (canvas_height / 2) as i32;

    // println!("{half_width}/{half_height}");
    let q1 = positions_at_target_time
        .iter()
        .filter(|Point { x, y }| x < &half_width && y < &half_height)
        .count();
    let q2 = positions_at_target_time
        .iter()
        .filter(|Point { x, y }| x > &half_width && y < &half_height)
        .count();
    let q3 = positions_at_target_time
        .iter()
        .filter(|Point { x, y }| x < &half_width && y > &half_height)
        .count();
    let q4 = positions_at_target_time
        .iter()
        .filter(|Point { x, y }| x > &half_width && y > &half_height)
        .count();

    // println!("{q1}/{q2}/{q3}/{q4}");
    (q1 * q2 * q3 * q4) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_14_part_01() {
        let input = read_string("./src/day14/input.txt").unwrap();

        let solution = solve_day_14_part_01(&input, 101, 103, 100);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_14_part_01_sample() {
        let input = "
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3"
            .trim();

        let actual = solve_day_14_part_01(&input, 11, 7, 100);

        assert_eq!(12, actual);
    }

    #[test]
    fn should_parse_input() {
        let input = "
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3"
            .trim();

        let Ok(lobby) = Lobby::try_from((input, 10, 11)) else {
            panic!("couldn't parse input")
        };

        assert_eq!(lobby.width, 10);
        assert_eq!(lobby.height, 11);
        assert_eq!(lobby.robots.len(), 12);
    }

    #[test]
    fn should_project_single_point_neg_wrap_around() {
        let r = Robot::new(Point::new(0, 0), Point::new(-1, -1));

        let actual = Robot::project_pos(&r, 3, 10, 10);

        assert_eq!(Point::new(7, 7), actual);
    }

    #[test]
    fn should_project_single_point_pos_wrap_around() {
        let r = Robot::new(Point::new(0, 0), Point::new(3, 3));

        let actual = Robot::project_pos(&r, 4, 10, 10);

        assert_eq!(Point::new(2, 2), actual);
    }

    #[test]
    fn should_project_single_robot() {
        let r = Robot::new(Point::new(2, 4), Point::new(2, -3));

        assert_eq!(Point::new(2, 4), Robot::project_pos(&r, 0, 11, 7));
        assert_eq!(Point::new(4, 1), Robot::project_pos(&r, 1, 11, 7));
        assert_eq!(Point::new(6, 5), Robot::project_pos(&r, 2, 11, 7));
        assert_eq!(Point::new(1, 3), Robot::project_pos(&r, 5, 11, 7));
    }

    #[test]
    fn difference_modulo_and_remainder() {
        // https://stackoverflow.com/a/57342011/9627206
        assert_eq!(-2, -2 % 4);
        assert_eq!(2, (-2i8).rem_euclid(4));
    }
}

/*
Initial state:
...........
...........
...........
...........
..1........
...........
...........

After 1 second:
...........
....1......
...........
...........
...........
...........
...........

After 2 seconds:
...........
...........
...........
...........
...........
......1....
...........

After 3 seconds:
...........
...........
........1..
...........
...........
...........
...........

After 4 seconds:
...........
...........
...........
...........
...........
...........
..........1

After 5 seconds:
...........
...........
...........
.1.........
...........
...........
...........
 */
