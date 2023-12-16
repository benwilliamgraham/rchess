#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Kind {
    Empty,
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
        Piece {
            kind,
            color,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self.kind, Kind::Empty)
    }

    fn encode(&self) -> u8 {
        self.kind as u8 | (self.color as u8) << 3
    }

    fn decode(encoded: u8) -> Piece {
        Piece {
            kind: encoded & 0b111,
            color: (encoded >> 3) & 0b1,
        }
    }
}

#[no_mangle]
pub fn test_fn() -> bool {
    true
}