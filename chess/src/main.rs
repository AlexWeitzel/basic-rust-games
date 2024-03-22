use std::fmt;
use std::io;
use std::io::Write;
use std::collections::HashMap;
//use std::ops::Index;
//use std::ops::IndexMut;

#[derive(Clone, Copy)]
enum PieceTypes {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone, Copy, PartialEq)]
enum Color {
    Black,
    White,
}
#[derive(Clone, Copy)]
struct Piece {
    piece_type: PieceTypes,
    color: Color,
}

struct Location {
    x: usize,
    y: usize,
}
impl Location {
    fn from(x: usize, y: usize) -> Self {
        Location { x , y }
    }
}
struct ChessMove {
    start:  String,
    end:    String,
}
impl ChessMove {
    fn from(start: String, end: String) ->Self {
        ChessMove { start, end }
    }
}
#[derive(Clone, Copy)]
enum Square {
    Empty,
    Occupied(Piece),
    //Location,
}


struct Board{
    squares: [[Square; 8]; 8],
}

impl Board{
    fn empty_board() -> Self {
        Self { squares : [[Square::Empty; 8]; 8]}
    }
    fn place_piece(&mut self, piece: Piece, location: Location){
        if location.x < 8 && location.y < 8 {
            self.squares[location.x][location.y] = Square::Occupied(piece);
        } else {
            println!("Error: location out of bounds");
        }
    }
    fn set_board(&mut self){
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Rook, 
                color: Color::White }, 
            Location::from(0, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Rook, 
                color: Color::White }, 
            Location::from(7, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Knight, 
                color: Color::White }, 
            Location::from(1, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Knight, 
                color: Color::White }, 
            Location::from(6, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Bishop, 
                color: Color::White }, 
            Location::from(2, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Bishop, 
                color: Color::White }, 
            Location::from(5, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::King, 
                color: Color::White }, 
            Location::from(4, 0)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Queen, 
                color: Color::White }, 
            Location::from(3, 0)
        );

        for i in 0..8{
            self.place_piece(
                Piece { 
                    piece_type: PieceTypes::Pawn, 
                    color: Color::White }, 
                Location::from(i, 1)
            );
        }



        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Rook, 
                color: Color::Black }, 
            Location::from(0, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Rook, 
                color: Color::Black }, 
            Location::from(7, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Knight, 
                color: Color::Black }, 
            Location::from(1, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Knight, 
                color: Color::Black }, 
            Location::from(6, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Bishop, 
                color: Color::Black }, 
            Location::from(2, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Bishop, 
                color: Color::Black }, 
            Location::from(5, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::King, 
                color: Color::Black }, 
            Location::from(4, 7)
        );
        self.place_piece(
            Piece { 
                piece_type: PieceTypes::Queen, 
                color: Color::Black }, 
            Location::from(3, 7)
        );

        for i in 0..8{
            self.place_piece(
                Piece { 
                    piece_type: PieceTypes::Pawn, 
                    color: Color::Black }, 
                Location::from(i, 6)
            );
        }
    }
    fn move_piece(&mut self, turn: u8) {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut num_map: HashMap<char, usize> = HashMap::new();
        for (i, c) in ('a'..'i').enumerate() {
            char_map.insert(c, i);
        }
        for (i, c) in ('1'..'9').enumerate() {
            num_map.insert(c, i);
        }

        loop {
            println!("Enter move:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let vec: Vec<&str> = input.split(',').collect();

            if vec.len() == 2 && vec[0].trim().len() == 2 && vec[1].trim().len() == 2{ //two entries, each entry is 2 chars long, trim off newline i think??
                let start = vec[0].trim();
                let end = vec[1].trim();

                if let (Some(&from_x), Some(&from_y), Some(&to_x), Some(&to_y)) =
                    (char_map.get(&start.chars().nth(0).unwrap()),
                    num_map.get(&start.chars().nth(1).unwrap()),
                    char_map.get(&end.chars().nth(0).unwrap()),
                    num_map.get(&end.chars().nth(1).unwrap())) {
                    //if self.squares[from_x][from_y]::occupied(Color::White){
                        match self.check_legality(turn, from_x, from_y, to_x, to_y) {
                            Ok(_) => {
                                let hold_piece = self.squares[from_x][from_y];
                                self.squares[from_x][from_y] = Square::Empty;
                                self.squares[to_x][to_y] = hold_piece;
                                break;
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                        
                    //}
                    
                }
            }
            println!("Expected a start and stop e.g. d3,f6");
        }
    }

    fn check_legality(&mut self, turn: u8, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Result<&'static str, &'static str> {
        let whos_turn = turn % 2;
        let mut turn_color = Color::Black;
        let mut not_turn_color = Color::White;
        match whos_turn{
            0 => {
                turn_color = Color::Black;
                not_turn_color = Color::White;
            },
            _ => {
                turn_color = Color::White;
                not_turn_color = Color::Black;
            },
        }
        let mut piece_color = Color::Black;
        match self.squares[from_x][from_y] {
            Square::Occupied(piece) => {
                piece_color = piece.color;
            },
            Square::Empty => {
    
            }
        }
        println!("the turn is {}, the mod is {}, the turn color is {}, and the selected piece is {}", turn, whos_turn, turn_color, piece_color);
    
        match self.squares[from_x][from_y]{
            Square::Empty => {return Err("there is nothing in that square");},
            Square::Occupied(piece) => {
                if piece.color == turn_color {
                    println!("the piece color is {} and the turn color is {}, they match!", piece.color, turn_color);
                } else {
                    return Err("that's not your color!");
                }
            }
        }
    
        if true {
            Ok("Good move")
        } else {
            Err("Bad move")
        }
    }
    

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..8 {
            s += &format!("{}", y+1);
            for x in 0..8 {
                let square = self.squares[x][y];
                s += "|";
                match square {
                    Square::Empty => s += " ",
                    Square::Occupied(piece) => {
                        match piece.color {
                            Color::Black => {
                                match piece.piece_type {
                                    PieceTypes::King => s += "K",
                                    PieceTypes::Rook => s += "R",
                                    PieceTypes::Bishop => s += "B",
                                    PieceTypes::Knight => s += "N",
                                    PieceTypes::Queen => s += "Q",
                                    PieceTypes::Pawn => s += "P",
                                }
                            }
                            Color::White => {
                                match piece.piece_type {
                                    PieceTypes::King => s += "k",
                                    PieceTypes::Rook => s += "r",
                                    PieceTypes::Bishop => s += "b",
                                    PieceTypes::Knight => s += "n",
                                    PieceTypes::Queen => s += "q",
                                    PieceTypes::Pawn => s += "p",
                                }
                            }
                        }
                        
                    }
                    Square::Occupied(_) => s += "_", // assuming you want to display occupied squares as "_"
                }
            }
            s += "|\n -----------------\n";
        }
        s += "  a b c d e f g h";
        write!(f, "{}", s)
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match self {
            Color::White => s += "white",
            Color::Black => s += "black",
        }
        write!(f, "{}", s)
    }
}

fn main() {
    game();
}

fn game() {
    let mut b = Board::empty_board();
    b.set_board();
    let mut turn: u8 = 1;
    loop{
        println!("{}", b);
        let whos_turn = turn % 2;
        let mut turn_color = "white";
        match whos_turn {
            0 => turn_color = "white",
            _ => turn_color = "black",

        }
        println!("{} to move!", turn_color);
        //let chess_move = get_move();
        Board::move_piece(&mut b, turn);
        
        turn += 1;
    }
    
}

