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
            kind: match encoded & 0b111 {
                0 => Kind::Empty,
                1 => Kind::Pawn,
                2 => Kind::Knight,
                3 => Kind::Bishop,
                4 => Kind::Rook,
                5 => Kind::Queen,
                6 => Kind::King,
                _ => panic!("Invalid piece kind"),
            },
            color: if (encoded >> 3) & 0b1 == 0 { Color::White } else { Color::Black },
        }
    }
}

#[no_mangle]
pub fn test_fn() -> bool {
    true
}