#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
struct Piece {
    kind: Kind,
    color: Color,
}

impl Piece {
    fn new(kind: Kind, color: Color) -> Piece {
        Piece { kind, color }
    }

    // Encode a piece value into a single byte in FEN format
    fn encode(&self) -> char {
        let lower_case = match self.kind {
            Kind::Pawn => 'p',
            Kind::Knight => 'n',
            Kind::Bishop => 'b',
            Kind::Rook => 'r',
            Kind::Queen => 'q',
            Kind::King => 'k',
        };
        match self.color {
            Color::White => lower_case.to_ascii_uppercase(),
            Color::Black => lower_case,
        }
    }

    // Decode a piece value from a single byte in FEN format
    fn decode(encoded: char) -> Piece {
        let lower_case = encoded.to_ascii_lowercase();
        let kind = match lower_case {
            'p' => Kind::Pawn,
            'n' => Kind::Knight,
            'b' => Kind::Bishop,
            'r' => Kind::Rook,
            'q' => Kind::Queen,
            'k' => Kind::King,
            _ => panic!("Invalid piece encoding"),
        };
        let color = match encoded.is_ascii_uppercase() {
            true => Color::White,
            false => Color::Black,
        };
        Piece::new(kind, color)
    }
}

#[derive(Copy, Clone)]
struct Square {
    rank: u8,
    file: u8,
}

impl Square {
    fn new(rank: u8, file: u8) -> Square {
        Square { rank, file }
    }

    fn encode(&self) -> String {
        let mut encoded = String::new();
        encoded.push((b'a' + self.file) as char);
        encoded.push((b'1' + self.rank) as char);
        encoded
    }

    fn decode(encoded: &str) -> Square {
        let mut chars = encoded.chars();
        let file = chars.next().unwrap() as u8 - b'a';
        let rank = chars.next().unwrap() as u8 - b'1';
        Square::new(rank, file)
    }
}

struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    fn new() -> Board {
        Board {
            squares: [[None; 8]; 8],
        }
    }

    fn get(&self, square: Square) -> Option<Piece> {
        self.squares[square.rank as usize][square.file as usize]
    }

    fn set(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.rank as usize][square.file as usize] = piece;
    }

    fn encode(&self) -> String {
        let mut fen = String::new();
        // TODO: Implement
        fen
    }

    fn decode(fen: &str) -> Board {
        let mut board = Board::new();
        let mut rank = 7;
        let mut file = 0;
        // TODO: Implement
        board
    }
}

struct CastlingAvailability {
    w_kingside: bool,
    w_queenside: bool,
    b_kingside: bool,
    b_queenside: bool,
}

impl CastlingAvailability {
    fn new() -> CastlingAvailability {
        CastlingAvailability {
            w_kingside: true,
            w_queenside: true,
            b_kingside: true,
            b_queenside: true,
        }
    }

    fn encode(&self) -> String {
        let mut encoded = String::new();
        if self.w_kingside {
            encoded.push('K');
        }
        if self.w_queenside {
            encoded.push('Q');
        }
        if self.b_kingside {
            encoded.push('k');
        }
        if self.b_queenside {
            encoded.push('q');
        }
        if !self.w_kingside && !self.w_queenside && !self.b_kingside && !self.b_queenside {
            encoded.push('-');
        }
        encoded
    }

    fn decode(encoded: &str) -> CastlingAvailability {
        let mut castling_availability = CastlingAvailability::new();
        for c in encoded.chars() {
            match c {
                'K' => castling_availability.w_kingside = true,
                'Q' => castling_availability.w_queenside = true,
                'k' => castling_availability.b_kingside = true,
                'q' => castling_availability.b_queenside = true,
                '-' => {
                    castling_availability.w_kingside = false;
                    castling_availability.w_queenside = false;
                    castling_availability.b_kingside = false;
                    castling_availability.b_queenside = false;
                }
                _ => panic!("Invalid castling availability encoding"),
            }
        }
        castling_availability
    }
}

enum PieceMovement {
    Move {
        from: Square,
        to: Square,
    },
    Capture {
        from: Square,
        to: Square,
        captured: Kind,
    },
    Promotion {
        from: Square,
        to: Square,
        promotion: Kind,
    },
    EnPassant {
        from: Square,
        to: Square,
        captured: Kind,
    },
    Castling {
        from: Square,
        to: Square,
    },
}

pub struct Move {
    piece_movement: PieceMovement,
    previous_en_passant: Option<Square>,
    previous_castling_availability: CastlingAvailability,
}

pub struct Game {
    board: Board,
    turn: Color,
    castling_availability: CastlingAvailability,
    en_passant: Option<Square>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            turn: Color::White,
            castling_availability: CastlingAvailability::new(),
            en_passant: None,
        }
    }

    fn encode(&self) -> String {
        let mut fen = String::new();
        fen.push_str(&self.board.encode());
        fen.push(' ');
        fen.push(match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        });
        fen.push(' ');
        fen.push_str(&self.castling_availability.encode());
        fen.push(' ');
        match self.en_passant {
            Some(square) => fen.push_str(&square.encode()),
            None => fen.push('-'),
        }
        fen.push_str(" 0 1");
        fen
    }

    fn decode(fen: &str) -> Game {
        let mut game = Game::new();
        game
    }
}

#[no_mangle]
pub fn test_fn() -> bool {
    true
}
