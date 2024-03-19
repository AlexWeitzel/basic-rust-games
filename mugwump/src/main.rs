use std::fmt;
use std::io;
use std::io::Write;
use std::ops::Index;
use std::ops::IndexMut;

extern crate rand;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum Cell {
    Unguessed,
    Correct,
    Incorrect,
}

struct Board {
    cells: Vec<Cell>,
    width: usize,   // X
    height: usize,  // Y
}

impl Board{
    fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![Cell::Unguessed; width * height],
            width,
            height,
        }
    }
    fn update(&mut self, guess_x: usize, guess_y: usize, is_correct: bool) {
        println!("your x guess is {0}, and your y guess is {1}", guess_x, guess_y);
        if is_correct {
            self.cells[guess_y *  self.width + guess_x] = Cell::Correct;
        } else {
            self.cells[guess_y *  self.width + guess_x] = Cell::Incorrect;
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(self.height * self.width + self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self[(x, y)]; // Dereference the reference
                if *cell == Cell::Unguessed {
                    s += "*";
                } else if *cell == Cell::Correct {
                    s += "O";
                } else {
                    s += "X";
                }
            }
            s += "\n";
        }
        write!(f, "{}", s) // Return a Result
    }
}


#[derive(PartialEq)]
struct Location {
    x: f32,
    y: f32,
}

impl Location {
    fn rand() -> Self {
        Location {
            x: rand::thread_rng().gen_range(0_f32, 10_f32).floor(),
            y: rand::thread_rng().gen_range(0_f32, 10_f32).floor(),
        }
    }
    fn from(x: f32, y: f32) -> Self {
        Location { x, y }
    }
    fn dist(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn main() {
    println!();
    println!(" *******************");
    println!(" *  M U G W U M P  *");
    println!(" *******************");
    println!();
    println!("The object of this game if to find four Mugwumps");
    println!("hidden on a 10 by 10 grid.  Homebase is position 0,0");
    println!("Any guess you make must be two numbers with each");
    println!("number between 0 and 9, inclusive.  First number");
    println!("is distance to right of homebase and second number");
    println!("is distance above homebase.");
    println!();
    println!("You get 10 tries.  After each try, I will tell");
    println!("you how far you are from each Mugwump.");

    loop {
        game();
        println!();
        println!("That was fun!  Let's play again.....");
        println!("Four more Mugwumps are now in hiding.");
    }
}

fn game() {
    // The original logic allowed for duplicates
    let mut mugwumps = [
        Location::rand(),
        Location::rand(),
        Location::rand(),
        Location::rand(),
    ];
    let mut b = Board::new(10,10);

    println!("\n{}", b);

    let mut turn = 0;
    loop {
        turn += 1;
        let guess = guess(turn);

        let mut mugwumps_remaining = mugwumps.len();
        let mut correct_num : usize = 0;
        for (i, m) in mugwumps.iter_mut().enumerate() {
            if guess == *m {
                correct_num += 1;
                m.x = -m.x;
                println!("You have found Mugwump {}", i + 1)

            }
            
            if m.x < 0.0 {
                mugwumps_remaining -= 1;
                continue;
            }
            println!("You are {:.1} units from Mugwump {}", guess.dist(m), i + 1);
        }
        if correct_num > 0 {
            Board::update(&mut b, guess.x as usize, guess.y as usize, true);
        } else {
            Board::update(&mut b, guess.x as usize, guess.y as usize, false);

        }
        println!("Here is the updated board!\n\n{}", b);

        if mugwumps_remaining == 0 {
            println!();
            println!("You got them all in {} turns!", turn);
            break;
        }
        if turn == 10 {
            println!();
            println!("Sorry, that's {} tries. Here is where they're hiding", turn);
            for (i, m) in mugwumps.iter().enumerate() {
                if m.x < 0.0 {
                    continue;
                }
                println!("Mugwump {} is at ({})", i + 1, m);
            }
            break;
        }
    }
}

fn guess(turn: u32) -> Location {
    println!("\n");
    loop {
        print!("Turn no. {} what is your guess? ", turn);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let vec: Vec<&str> = input.split(",").collect();
        if vec.len() == 2 {
            if let Ok(x) = vec[0].trim().parse::<f32>() {
                if let Ok(y) = vec[1].trim().parse::<f32>() {
                    if x > 9.0 || y > 9.0 {
                        println!{"Outise board range, guess again"}
                    }
                    else {
                        return Location::from(x, y);
                    }
                }
            }
        }
        else {
            println!("Expected digit-comma-digit e.g. 0,9");
        }
    }
}

