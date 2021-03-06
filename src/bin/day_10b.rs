use std::fs;
use std::time::Instant;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, point)| (x, y, point))
            })
            .flatten()
            .filter_map(|info| {
                let (x, y, point) = info;
                match point {
                    '#' => Some(Self::new(x as i32, y as i32)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn distance_to(&self, other: &Asteroid) -> Option<Vector> {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx == 0 && dy == 0 {
            return None;
        }

        let quadrant = match (dx, dy) {
            (dx, dy) if dx >= 0 && dy < 0 => Quadrant::A,
            (dx, dy) if dx > 0 && dy >= 0 => Quadrant::B,
            (dx, dy) if dx <= 0 && dy > 0 => Quadrant::C,
            (_, _) => Quadrant::D,
        };

        let (dx, dy) = match quadrant {
            Quadrant::A | Quadrant::C => (dx.abs(), dy.abs()),
            _ => (dy.abs(), dx.abs()),
        };

        let angel = (dx * 1_000_000) / (dy * 1_000);
        Some(Vector { quadrant, angel })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Quadrant {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Vector {
    quadrant: Quadrant,
    angel: i32,
}

struct TaskA<'a> {
    field: &'a [Asteroid],
}

impl<'a> TaskA<'a> {
    pub fn solve(&self) -> Option<(usize, i32, i32)> {
        self.field
            .iter()
            .map(|i| {
                let mut total = self
                    .field
                    .iter()
                    .filter_map(|j| i.distance_to(j))
                    .collect::<Vec<_>>();

                total.sort();
                total.dedup();

                (total.len(), i.x, i.y)
            })
            .max_by_key(|(len, _, _)| *len)
    }

    pub fn new(field: &'a [Asteroid]) -> Self {
        Self { field }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Active,
    Vaporized,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AsteroidInfo {
    vector: Vector,
    distance: i32,
    state: State,
    x: i32,
    y: i32,
}

impl AsteroidInfo {
    pub fn active(&self) -> bool {
        self.state == State::Active
    }

    pub fn vaporize(&mut self) {
        self.state = State::Vaporized;
    }
}

struct LaserIter {
    field: Vec<AsteroidInfo>,
    last_vector: Option<Vector>,
}

impl LaserIter {
    pub fn new(input: &[Asteroid], x: i32, y: i32) -> Self {
        let origin = Asteroid { x: x, y: y };
        let mut field = input
            .iter()
            .filter_map(|a| {
                let vector = origin.distance_to(a);
                match vector {
                    Some(vector) => {
                        let distance = (origin.x - a.x).abs() + (origin.y - a.y).abs();

                        Some(AsteroidInfo {
                            vector,
                            distance,
                            state: State::Active,
                            x: a.x,
                            y: a.y,
                        })
                    }
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        field.sort();

        Self {
            field,
            last_vector: None,
        }
    }

    fn next_active(&self) -> Option<usize> {
        self.field.iter().position(|i| i.active())
    }

    fn next_active_after(&self, v: &Vector) -> Option<usize> {
        self.field.iter().position(|i| i.active() && i.vector > *v)
    }

    fn save_info(&mut self, index: usize) -> (i32, i32) {
        self.last_vector = Some(self.field[index].vector.clone());
        self.field[index].vaporize();
        (self.field[index].x, self.field[index].y)
    }
}

impl Iterator for LaserIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.last_vector
            .as_ref()
            .and_then(|v| self.next_active_after(v))
            .or(self.next_active())
            .map(|i| self.save_info(i))
    }
}

struct TaskB {
    iter: LaserIter,
}

impl TaskB {
    pub fn new(field: &[Asteroid], x: i32, y: i32) -> Self {
        let iter = LaserIter::new(field, x, y);
        Self { iter }
    }

    pub fn solve(&mut self) -> Option<(i32, i32)> {
        self.iter.nth(199)
    }
}

fn main() -> Result<()> {
    let now = Instant::now();

    let input = fs::read_to_string("src/bin/day_10_data.txt")?;
    let field = Asteroid::parse(&input);
    let field_size = field.len();

    let task_a = TaskA::new(&field).solve().unwrap();
    let task_b = TaskB::new(&field, task_a.1, task_a.2).solve().unwrap();

    let total_time = now.elapsed();

    println!("Total asteroids: {}", field_size);
    println!("Task I :  {} (x: {}, y: {})", task_a.0, task_a.1, task_a.2);
    println!(
        "Task II:  {} (x: {}, y: {})",
        task_b.0 * 100 + task_b.1,
        task_b.0,
        task_b.1
    );
    println!("Total time: {}μs", total_time.as_micros());
    Ok(())
}
