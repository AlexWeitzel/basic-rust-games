use std::fmt;
use std::io;
use std::io::Write;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone, PartialEq)] //i don't quite understand this
enum Square {
    Empty,
    Occupied,
}
#[derive(Clone, PartialEq)] //i don't quite understand this

enum Colors {
    White,
    Black,
}
#[derive(Clone, PartialEq)] //i don't quite understand this

enum Piece_Types {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight
}

struct Piece {
    color: Vec<Colors>,
    p_type: Vec<Piece_Types>,
    //location: Location,
}

struct Board {
    squares: Vec<Square>,
    width: usize,   // X
    height: usize,  // Y
}

impl Board{
    // First, make an empty board
    fn new(width: usize, height: usize) -> Self {
        Board{
            squares: vec![Square::Empty; width * height],
            width,
            height,
        }
    }
    fn set(&mut self) {
        //top of board
        for y in 0..2 {
            for x in 0..self.width {
                self.squares[y * self.width + x] = Square::Occupied;
            }
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Square; //don't understand
    fn index(&self, (x,y): (usize, usize)) -> &Self::Output {
        &self.squares[y * self.width + x] // how does this work?
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.squares[y * self.width + x] // I think I get this now
        // this is a really weird way of storing a 2d array in a 1d vector... wow
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(4*(self.height*self.width + self.height)); // the 4 times is to add other stuff
        for y in 0..self.height {
            for x in 0..self.width {
                let square = &self[(x , y)];
                s += "|";
                if *square == Square::Empty {
                    s += "*";
                } else if *square == Square::Occupied {
                    s += "_";
                }
            }
            s += "|\n-----------------\n";
        }
        write!(f, "{}", s) // Returns this
    }
}

struct Location {
    start:  String,
    end:    String,
}

impl Location {
    fn from(start: String, end: String) ->Self {
        Location { start, end }
    }
}


fn main() {
    println!("Hello, world!");
    loop {
        game();
    }
}

fn game() {
    let mut b = Board::new(8,8);
    println!("\n{}", b);

    let mut turn = 0;
    loop {
        turn += 1;
        let my_move = get_move(turn);
        println!("{}", my_move.start);
        println!("\n{}", b);
        Board::set(&mut b);
    }
}

fn get_move(turn: u32) -> Location {
    println!("]\n");
    loop {
        println!("enter move");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let vec: Vec<&str> = input.split(",").collect();

        // i want to use match here to get chess moves, we'll see
        if vec.len() == 2 {
            if let Ok(start) = vec[0].trim().parse::<String>() {
                if let Ok(end) = vec[1].trim().parse::<String>() {
                    if end.len() == 2 && start.len() == 2{
                        return Location::from(start, end);
                    } else { println!("only 2 letters you goomba!");}
                }
            }
        } else {
            println!("Expected a start and stop e.g. d3,f6");
        }
    }
}