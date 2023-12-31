/* This is the chess module.
 * It's goal is to implement the chess logic.
 */

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
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
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                match self.get(Square::new(rank, file)) {
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

#[derive(Copy, Clone)]
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

enum Move {
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

struct MoveWithState {
    move_: Move,
    previous_en_passant: Option<Square>,
    previous_castling_availability: CastlingAvailability,
}

struct Game {
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

    fn make_move(&mut self, move_: Move) -> MoveWithState {
        let previous_en_passant = self.en_passant;
        let previous_castling_availability = self.castling_availability;
        fn update_en_passant(piece: Piece, from: Square, to: Square) -> Option<Square> {
            match piece.kind {
                Kind::Pawn => {
                    if (from.rank as i8 - to.rank as i8).abs() == 2 {
                        Some(from)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        match move_ {
            Move::Move { from, to } => {
                let piece = self.board.get(from).unwrap();
                update_en_passant(piece, from, to);
                self.board.set(to, self.board.get(from));
                self.board.set(from, None);
            }
            Move::Capture {
                from,
                to,
                captured,
            } => {
                let piece = self.board.get(from).unwrap();
                update_en_passant(piece, from, to);
                self.board.set(to, self.board.get(from));
                self.board.set(from, None);
            }
            Move::Promotion {
                from,
                to,
                promotion,
            } => {
                self.board.set(to, Some(Piece::new(promotion, self.turn)));
                self.board.set(from, None);
            }
            Move::EnPassant {
                from,
                to,
                captured,
            } => {
                self.board.set(to, self.board.get(from));
                self.board.set(from, None);
                self.board.set(Square::new(to.rank, from.file), None);
            }
            Move::Castling { from, to } => {
                self.board.set(to, self.board.get(from));
                self.board.set(from, None);
                match to.file {
                    2 => {
                        self.board.set(Square::new(to.rank, 3), self.board.get(Square::new(to.rank, 0)));
                        self.board.set(Square::new(to.rank, 0), None);
                    }
                    6 => {
                        self.board.set(Square::new(to.rank, 5), self.board.get(Square::new(to.rank, 7)));
                        self.board.set(Square::new(to.rank, 7), None);
                    }
                    _ => panic!("Invalid castling move"),
                }
            }
        }
        // Update castling availability
        if self.board.get(Square::new(7, 4)) != Some(Piece::new(Kind::King, Color::White)) {
            self.castling_availability.w_kingside = false;
            self.castling_availability.w_queenside = false;
        }
        if self.board.get(Square::new(7, 0)) != Some(Piece::new(Kind::Rook, Color::White)) {
            self.castling_availability.w_queenside = false;
        }
        if self.board.get(Square::new(7, 7)) != Some(Piece::new(Kind::Rook, Color::White)) {
            self.castling_availability.w_kingside = false;
        }
        if self.board.get(Square::new(0, 4)) != Some(Piece::new(Kind::King, Color::Black)) {
            self.castling_availability.b_kingside = false;
            self.castling_availability.b_queenside = false;
        }
        if self.board.get(Square::new(0, 0)) != Some(Piece::new(Kind::Rook, Color::Black)) {
            self.castling_availability.b_queenside = false;
        }
        if self.board.get(Square::new(0, 7)) != Some(Piece::new(Kind::Rook, Color::Black)) {
            self.castling_availability.b_kingside = false;
        }
        MoveWithState {
            move_,
            previous_en_passant,
            previous_castling_availability,
        }
    }

    fn undo_move(&mut self, move_with_state: MoveWithState) {
        self.en_passant = move_with_state.previous_en_passant;
        self.castling_availability = move_with_state.previous_castling_availability;
        match move_with_state.move_ {
            Move::Move { from, to } => {
                self.board.set(from, self.board.get(to));
                self.board.set(to, None);
            }
            Move::Capture {
                from,
                to,
                captured,
            } => {
                self.board.set(from, self.board.get(to));
                self.board.set(to, Some(Piece::new(captured, self.turn)));
            }
            Move::Promotion {
                from,
                to,
                promotion,
            } => {
                self.board.set(from, Some(Piece::new(Kind::Pawn, self.turn)));
                self.board.set(to, Some(Piece::new(promotion, self.turn)));
            }
            Move::EnPassant {
                from,
                to,
                captured,
            } => {
                self.board.set(from, self.board.get(to));
                self.board.set(to, None);
                self.board.set(Square::new(to.rank, from.file), Some(Piece::new(captured, self.turn)));
            }
            Move::Castling { from, to } => {
                self.board.set(from, self.board.get(to));
                self.board.set(to, None);
                match to.file {
                    2 => {
                        self.board.set(Square::new(to.rank, 3), self.board.get(Square::new(to.rank, 0)));
                        self.board.set(Square::new(to.rank, 0), None);
                    }
                    6 => {
                        self.board.set(Square::new(to.rank, 5), self.board.get(Square::new(to.rank, 7)));
                        self.board.set(Square::new(to.rank, 7), None);
                    }
                    _ => panic!("Invalid castling move"),
                }
            }
        }
    }
}