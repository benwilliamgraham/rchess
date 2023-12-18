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

struct CastlingAvailability {
    w_kingside: bool,
    w_queenside: bool,
    b_kingside: bool,
    b_queenside: bool,
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

struct Move {
    piece_movement: PieceMovement,
    previous_en_passant: Option<Square>,
    previous_castling_availability: CastlingAvailability,
}

struct Game {
    board: [[Option<Piece>; 8]; 8],
    turn: Color,
    castling_availability: CastlingAvailability,
    en_passant: Option<Square>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[None; 8]; 8],
            turn: Color::White,
            castling_availability: CastlingAvailability {
                w_kingside: true,
                w_queenside: true,
                b_kingside: true,
                b_queenside: true,
            },
            en_passant: None,
        }
    }

    fn encode(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                match self.board[rank][file] {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push(piece.encode());
                    }
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        match self.turn {
            Color::White => fen.push('w'),
            Color::Black => fen.push('b'),
        }
        fen.push(' ');
        if self.castling_availability.w_kingside {
            fen.push('K');
        }
        if self.castling_availability.w_queenside {
            fen.push('Q');
        }
        if self.castling_availability.b_kingside {
            fen.push('k');
        }
        if self.castling_availability.b_queenside {
            fen.push('q');
        }
        if !self.castling_availability.w_kingside
            && !self.castling_availability.w_queenside
            && !self.castling_availability.b_kingside
            && !self.castling_availability.b_queenside
        {
            fen.push('-');
        }
        fen.push(' ');
        match self.en_passant {
            Some(square) => {
                fen.push((b'a' + square.file) as char);
                fen.push((b'1' + square.rank) as char);
            }
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
