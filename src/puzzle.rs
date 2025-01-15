use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}
impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Empty => "  ".to_string(),
            Tile::One => "01".to_string(),
            Tile::Two => "02".to_string(),
            Tile::Three => "03".to_string(),
            Tile::Four => "04".to_string(),
            Tile::Five => "05".to_string(),
            Tile::Six => "06".to_string(),
            Tile::Seven => "07".to_string(),
            Tile::Eight => "08".to_string(),
            Tile::Nine => "09".to_string(),
            Tile::Ten => "10".to_string(),
            Tile::Eleven => "11".to_string(),
            Tile::Twelve => "12".to_string(),
            Tile::Thirteen => "13".to_string(),
            Tile::Fourteen => "14".to_string(),
            Tile::Fifteen => "15".to_string(),
        }
    }
}

enum Direction {
    Up = 259,
    Down = 258,
    Left = 260,
    Right = 261,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        match value {
            259 | 75 | 107 /* up-arrow, uppercase 'K', lowercase 'k' */ => Direction::Up,
            258 | 74 | 106 /* down-arrow, uppercase 'J', lowercase 'j' */ => Direction::Down,
            260 | 72 | 104 /* left-arrow, uppercase 'H', lowercase 'h' */ => Direction::Left,
            261 | 76 | 108 /* right-arrow, uppercase 'L', lowercase 'l' */ => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Direction {
    fn is_direction(value: i32) -> bool {
        /*
         * 258: down-arrow
         * 259: up-arrow
         * 260: left-arrow
         * 261: right-arrow
         * 72: uppercase 'H'
         * 74: uppercase 'J'
         * 75: uppercase 'K'
         * 76: uppercase 'L'
         * 104: lowercase 'h'
         * 106: lowercase 'j'
         * 107: lowercase 'k'
         * 108: lowercase 'l'
         * */
        match value {
            258 | 259 | 260 | 261 | 72 | 74 | 75 | 76 | 104 | 106 | 107 | 108 => true,
            _ => false,
        }
    }
}

pub struct Puzzle {
    pub tiles: Vec<Tile>,
    moves: u32,
}

impl Puzzle {
    pub fn new() -> Self {
        let tiles = vec![
            Tile::One,
            Tile::Two,
            Tile::Three,
            Tile::Four,
            Tile::Five,
            Tile::Six,
            Tile::Seven,
            Tile::Eight,
            Tile::Nine,
            Tile::Ten,
            Tile::Eleven,
            Tile::Twelve,
            Tile::Thirteen,
            Tile::Fourteen,
            Tile::Fifteen,
            Tile::Empty,
        ];
        Self { tiles, moves: 0 }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        let mut old_tiles = self.tiles.clone();
        self.tiles = Vec::new();

        while !old_tiles.is_empty() {
            let index = rng.gen::<usize>() % old_tiles.len();
            self.tiles.push(old_tiles.remove(index));
        }
    }

    pub fn print_puzzle(&self) {
        println!("{}", self.to_string());
    }

    pub fn is_solved(&self) -> bool {
        let solved_puzzle = vec![
            Tile::One,
            Tile::Two,
            Tile::Three,
            Tile::Four,
            Tile::Five,
            Tile::Six,
            Tile::Seven,
            Tile::Eight,
            Tile::Nine,
            Tile::Ten,
            Tile::Eleven,
            Tile::Twelve,
            Tile::Thirteen,
            Tile::Fourteen,
            Tile::Fifteen,
            Tile::Empty,
        ];
        solved_puzzle
            .iter()
            .zip(self.tiles.iter())
            .all(|(a, b)| a == b)
    }

    pub fn reset(&mut self) -> () {
        self.tiles = vec![
            Tile::One,
            Tile::Two,
            Tile::Three,
            Tile::Four,
            Tile::Five,
            Tile::Six,
            Tile::Seven,
            Tile::Eight,
            Tile::Nine,
            Tile::Ten,
            Tile::Eleven,
            Tile::Twelve,
            Tile::Thirteen,
            Tile::Fourteen,
            Tile::Fifteen,
            Tile::Empty,
        ];
    }

    pub fn handle_input(&mut self, keypress: i32) {
        let empty_index = self.tiles.iter().position(|&x| x == Tile::Empty).unwrap();

        if !Direction::is_direction(keypress) {
            return;
        }

        let direction = Direction::from(keypress);

        match direction {
            Direction::Up => self.move_down(empty_index),
            Direction::Down => self.move_up(empty_index),
            Direction::Left => self.move_left(empty_index),
            Direction::Right => self.move_right(empty_index),
        }
    }

    fn move_up(&mut self, empty_index: usize) {
        match empty_index {
            0 | 1 | 2 | 3 => return,
            _ => {
                self.moves += 1;
                self.tiles.swap(empty_index, empty_index - 4)
            }
        }
    }

    fn move_down(&mut self, empty_index: usize) {
        match empty_index {
            12 | 13 | 14 | 15 => return,
            _ => {
                self.moves += 1;
                self.tiles.swap(empty_index, empty_index + 4)
            }
        }
    }

    fn move_left(&mut self, empty_index: usize) {
        match empty_index {
            3 | 7 | 11 | 15 => return,
            _ => {
                self.moves += 1;
                self.tiles.swap(empty_index, empty_index + 1)
            }
        }
    }

    fn move_right(&mut self, empty_index: usize) {
        match empty_index {
            0 | 4 | 8 | 12 => return,
            _ => {
                self.moves += 1;
                self.tiles.swap(empty_index, empty_index - 1)
            }
        }
    }

    pub fn get_moves(&self) -> u32 {
        self.moves
    }
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("\n+----+----+----+----+\n");
        for i in 0..4 {
            result.push_str("| ");
            for j in 0..4 {
                let index = i * 4 + j;
                result.push_str(self.tiles[index].to_string().as_str());
                result.push_str(" | ");
            }
            result.push_str("\n+----+----+----+----+\n");
        }
        result
    }
}
