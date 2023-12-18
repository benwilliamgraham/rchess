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

#[no_mangle]
pub fn test_fn() -> bool {
    true
}
