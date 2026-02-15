use super::chess::{NotationMove, Piece, ResolvedMove, Square};
use super::hint::{extract_hints, is_castling, resolve_castling, strip_annotations};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub(crate) squares: [[Option<(Piece, Color)>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        let back_rank = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for (file, &piece) in back_rank.iter().enumerate() {
            squares[0][file] = Some((piece, Color::White));
            squares[1][file] = Some((Piece::Pawn, Color::White));
            squares[6][file] = Some((Piece::Pawn, Color::Black));
            squares[7][file] = Some((piece, Color::Black));
        }

        Board { squares }
    }

    pub fn get(&self, file: u8, rank: u8) -> Option<(Piece, Color)> {
        self.squares[rank as usize][file as usize]
    }

    pub(crate) fn set(&mut self, file: u8, rank: u8, piece: (Piece, Color)) {
        self.squares[rank as usize][file as usize] = Some(piece);
    }

    fn clear_square(&mut self, file: u8, rank: u8) {
        self.squares[rank as usize][file as usize] = None;
    }

    /// Returns `true` if any piece of `attacker_color` attacks the `target` square.
    pub fn is_square_attacked_by(&self, target: Square, attacker_color: Color) -> bool {
        for rank in 0..8u8 {
            for file in 0..8u8 {
                if let Some((piece, color)) = self.get(file, rank)
                    && color == attacker_color
                    && self.attacks_square(piece, color, file, rank, &target)
                {
                    return true;
                }
            }
        }
        false
    }

    fn attacks_square(
        &self,
        piece: Piece,
        color: Color,
        file: u8,
        rank: u8,
        target: &Square,
    ) -> bool {
        match piece {
            Piece::Pawn => Self::pawn_attacks_square(color, file, rank, target),
            Piece::Knight => self.knight_can_reach(file, rank, target),
            Piece::Bishop => self.bishop_can_reach(file, rank, target),
            Piece::Rook => self.rook_can_reach(file, rank, target),
            Piece::Queen => {
                self.bishop_can_reach(file, rank, target)
                    || self.rook_can_reach(file, rank, target)
            }
            Piece::King => self.king_can_reach(file, rank, target),
        }
    }

    /// Returns `true` if a pawn of the given color at (file, rank) attacks the target square.
    /// Pawns attack diagonally only — forward movement is not an attack.
    fn pawn_attacks_square(color: Color, file: u8, rank: u8, target: &Square) -> bool {
        let direction: i8 = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let file_distance = (target.file as i8) - (file as i8);
        let rank_distance = (target.rank as i8) - (rank as i8);
        file_distance.abs() == 1 && rank_distance == direction
    }

    /// Returns `true` if the king of the given color is in checkmate:
    /// in check with no legal moves to escape.
    pub fn is_checkmate(&self, color: Color) -> bool {
        self.is_in_check(color) && !self.has_any_legal_move(color)
    }

    fn has_any_legal_move(&self, color: Color) -> bool {
        for rank in 0..8u8 {
            for file in 0..8u8 {
                if let Some((piece, piece_color)) = self.get(file, rank)
                    && piece_color == color
                    && self.piece_has_legal_move(piece, color, file, rank)
                {
                    return true;
                }
            }
        }
        false
    }

    fn piece_has_legal_move(&self, piece: Piece, color: Color, file: u8, rank: u8) -> bool {
        for dest_rank in 0..8u8 {
            for dest_file in 0..8u8 {
                let dest = Square { file: dest_file, rank: dest_rank };
                if self.is_valid_destination(piece, color, file, rank, &dest)
                    && self.move_leaves_king_safe(color, file, rank, &dest)
                {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid_destination(
        &self,
        piece: Piece,
        color: Color,
        file: u8,
        rank: u8,
        dest: &Square,
    ) -> bool {
        // Can't capture own piece
        if let Some((_, dest_color)) = self.get(dest.file, dest.rank)
            && dest_color == color
        {
            return false;
        }
        match piece {
            Piece::Pawn => self.pawn_can_move_to(color, file, rank, dest),
            Piece::Knight => self.knight_can_reach(file, rank, dest),
            Piece::Bishop => self.bishop_can_reach(file, rank, dest),
            Piece::Rook => self.rook_can_reach(file, rank, dest),
            Piece::Queen => {
                self.bishop_can_reach(file, rank, dest) || self.rook_can_reach(file, rank, dest)
            }
            Piece::King => self.king_can_reach(file, rank, dest),
        }
    }

    /// Like `pawn_can_reach` but for legal move generation:
    /// diagonal moves require an enemy piece at the destination (captures must have a target).
    fn pawn_can_move_to(&self, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        let (direction, start_rank): (i8, u8) = match color {
            Color::White => (1, 1),
            Color::Black => (-1, 6),
        };
        let file_distance = (dest.file as i8) - (file as i8);
        let rank_distance = (dest.rank as i8) - (rank as i8);

        // Forward one square
        if file_distance == 0
            && rank_distance == direction
            && self.get(dest.file, dest.rank).is_none()
        {
            return true;
        }
        // Forward two squares from starting rank
        if file_distance == 0 && rank_distance == 2 * direction && rank == start_rank {
            let mid_rank = (rank as i8 + direction) as u8;
            if self.get(file, mid_rank).is_none() && self.get(dest.file, dest.rank).is_none() {
                return true;
            }
        }
        // Diagonal capture — must have an enemy piece at destination
        if file_distance.abs() == 1
            && rank_distance == direction
            && let Some((_, dest_color)) = self.get(dest.file, dest.rank)
        {
            return dest_color != color;
        }
        false
    }

    fn move_leaves_king_safe(&self, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        let mut trial = self.clone();
        let piece_on_origin = trial.get(file, rank);
        trial.clear_square(file, rank);
        trial.squares[dest.rank as usize][dest.file as usize] = piece_on_origin;
        !trial.is_in_check(color)
    }

    /// Returns `true` if the king of the given color is in check.
    pub fn is_in_check(&self, color: Color) -> bool {
        match self.find_king(color) {
            Some(king_square) => self.is_square_attacked_by(king_square, color.opponent()),
            None => false,
        }
    }

    /// Returns the square of the king for the given color, or `None` if no king exists.
    pub fn find_king(&self, color: Color) -> Option<Square> {
        for rank in 0..8u8 {
            for file in 0..8u8 {
                if let Some((Piece::King, piece_color)) = self.get(file, rank)
                    && piece_color == color
                {
                    return Some(Square { file, rank });
                }
            }
        }
        None
    }

    /// Resolves algebraic notation into a fully-specified move with origin, destination,
    /// and any special move data (castling rook, promotion).
    pub fn resolve_move(
        &self,
        chess_move: &NotationMove,
        notation: &str,
        color: Color,
    ) -> Option<ResolvedMove> {
        if is_castling(notation) {
            return resolve_castling(chess_move, color);
        }

        let clean = strip_annotations(notation);
        let (file_hint, rank_hint) = extract_hints(&clean, chess_move.piece);

        let origin = self.find_origin(
            chess_move.piece,
            &chess_move.dest,
            color,
            file_hint,
            rank_hint,
        )?;

        Some(ResolvedMove {
            origin,
            dest: chess_move.dest,
            promotion: chess_move.promotion,
            castling_rook: None,
        })
    }

    pub fn apply_move(&mut self, parsed: &ResolvedMove) {
        // Move the piece from origin to destination (handles king in castling too)
        let piece_on_origin = self.get(parsed.origin.file, parsed.origin.rank);
        self.clear_square(parsed.origin.file, parsed.origin.rank);

        if let Some(promoted_piece) = parsed.promotion {
            let color = piece_on_origin
                .map(|(_, color)| color)
                .expect("piece must exist at origin for promotion");
            self.set(parsed.dest.file, parsed.dest.rank, (promoted_piece, color));
        } else {
            // Captured pieces (if any) are simply overwritten — no tracking yet
            self.squares[parsed.dest.rank as usize][parsed.dest.file as usize] = piece_on_origin;
        }

        // Castling: the king was already moved above; now move the rook
        if let Some((rook_from, rook_to)) = parsed.castling_rook {
            let rook = self.get(rook_from.file, rook_from.rank);
            self.clear_square(rook_from.file, rook_from.rank);
            self.squares[rook_to.rank as usize][rook_to.file as usize] = rook;
        }
    }

    fn find_origin(
        &self,
        piece: Piece,
        dest: &Square,
        color: Color,
        file_hint: Option<u8>,
        rank_hint: Option<u8>,
    ) -> Option<Square> {
        for rank in 0..8u8 {
            for file in 0..8u8 {
                if let Some((found_piece, found_color)) = self.get(file, rank) {
                    if found_piece != piece || found_color != color {
                        continue;
                    }
                    // Skip square if disambiguation hint doesn't match
                    if let Some(hint_file) = file_hint
                        && file != hint_file
                    {
                        continue;
                    }
                    if let Some(hint_rank) = rank_hint
                        && rank != hint_rank
                    {
                        continue;
                    }
                    if self.can_reach(piece, color, file, rank, dest) {
                        return Some(Square { file, rank });
                    }
                }
            }
        }
        None
    }

    fn can_reach(&self, piece: Piece, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        match piece {
            Piece::Pawn => self.pawn_can_reach(color, file, rank, dest),
            Piece::Knight => self.knight_can_reach(file, rank, dest),
            Piece::Bishop => self.bishop_can_reach(file, rank, dest),
            Piece::Rook => self.rook_can_reach(file, rank, dest),
            Piece::Queen => {
                self.bishop_can_reach(file, rank, dest) || self.rook_can_reach(file, rank, dest)
            }
            Piece::King => self.king_can_reach(file, rank, dest),
        }
    }

    fn pawn_can_reach(&self, color: Color, file: u8, rank: u8, dest: &Square) -> bool {
        let (direction, start_rank): (i8, u8) = match color {
            Color::White => (1, 1),
            Color::Black => (-1, 6),
        };
        let file_distance = (dest.file as i8) - (file as i8);
        let rank_distance = (dest.rank as i8) - (rank as i8);

        if file_distance == 0
            && rank_distance == direction
            && self.get(dest.file, dest.rank).is_none()
        {
            return true;
        }
        if file_distance == 0 && rank_distance == 2 * direction && rank == start_rank {
            let mid_rank = (rank as i8 + direction) as u8;
            if self.get(file, mid_rank).is_none() && self.get(dest.file, dest.rank).is_none() {
                return true;
            }
        }
        if file_distance.abs() == 1 && rank_distance == direction {
            return true;
        }
        false
    }

    fn knight_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let file_distance = ((dest.file as i8) - (file as i8)).abs();
        let rank_distance = ((dest.rank as i8) - (rank as i8)).abs();
        (file_distance == 2 && rank_distance == 1) || (file_distance == 1 && rank_distance == 2)
    }

    fn bishop_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let file_distance = (dest.file as i8) - (file as i8);
        let rank_distance = (dest.rank as i8) - (rank as i8);
        if file_distance.abs() != rank_distance.abs() || file_distance == 0 {
            return false;
        }
        self.path_clear(
            file,
            rank,
            dest,
            file_distance.signum(),
            rank_distance.signum(),
        )
    }

    fn rook_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let file_distance = (dest.file as i8) - (file as i8);
        let rank_distance = (dest.rank as i8) - (rank as i8);
        if (file_distance != 0 && rank_distance != 0) || (file_distance == 0 && rank_distance == 0)
        {
            return false;
        }
        self.path_clear(
            file,
            rank,
            dest,
            file_distance.signum(),
            rank_distance.signum(),
        )
    }

    fn king_can_reach(&self, file: u8, rank: u8, dest: &Square) -> bool {
        let file_distance = ((dest.file as i8) - (file as i8)).abs();
        let rank_distance = ((dest.rank as i8) - (rank as i8)).abs();
        file_distance <= 1 && rank_distance <= 1 && (file_distance + rank_distance) > 0
    }

    fn path_clear(&self, file: u8, rank: u8, dest: &Square, file_step: i8, rank_step: i8) -> bool {
        let mut current_file = file as i8 + file_step;
        let mut current_rank = rank as i8 + rank_step;
        while current_file != dest.file as i8 || current_rank != dest.rank as i8 {
            if self.get(current_file as u8, current_rank as u8).is_some() {
                return false;
            }
            current_file += file_step;
            current_rank += rank_step;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_opponent_is_black() {
        assert_eq!(Color::White.opponent(), Color::Black);
    }

    #[test]
    fn black_opponent_is_white() {
        assert_eq!(Color::Black.opponent(), Color::White);
    }

    #[test]
    fn find_king_white_initial_position() {
        let board = Board::new();
        assert_eq!(board.find_king(Color::White), Some(Square { file: 4, rank: 0 }));
    }

    #[test]
    fn find_king_black_initial_position() {
        let board = Board::new();
        assert_eq!(board.find_king(Color::Black), Some(Square { file: 4, rank: 7 }));
    }

    #[test]
    fn find_king_empty_board_returns_none() {
        let board = Board { squares: [[None; 8]; 8] };
        assert_eq!(board.find_king(Color::White), None);
    }

    #[test]
    fn white_pawn_attacks_diagonals() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 3, (Piece::Pawn, Color::White)); // e4
        let left_diagonal = Square { file: 3, rank: 4 }; // d5
        let right_diagonal = Square { file: 5, rank: 4 }; // f5
        assert!(board.is_square_attacked_by(left_diagonal, Color::White));
        assert!(board.is_square_attacked_by(right_diagonal, Color::White));
    }

    #[test]
    fn white_pawn_does_not_attack_forward() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 3, (Piece::Pawn, Color::White)); // e4
        let forward = Square { file: 4, rank: 4 }; // e5
        assert!(!board.is_square_attacked_by(forward, Color::White));
    }

    #[test]
    fn black_pawn_attacks_diagonals() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 6, (Piece::Pawn, Color::Black)); // e7
        let left_diagonal = Square { file: 3, rank: 5 }; // d6
        let right_diagonal = Square { file: 5, rank: 5 }; // f6
        assert!(board.is_square_attacked_by(left_diagonal, Color::Black));
        assert!(board.is_square_attacked_by(right_diagonal, Color::Black));
    }

    #[test]
    fn knight_attacks_l_shape() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 3, (Piece::Knight, Color::White)); // Ne4
        let target = Square { file: 5, rank: 5 }; // f6
        assert!(board.is_square_attacked_by(target, Color::White));
    }

    #[test]
    fn rook_attacks_clear_path() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(0, 0, (Piece::Rook, Color::White)); // Ra1
        let target = Square { file: 0, rank: 7 }; // a8
        assert!(board.is_square_attacked_by(target, Color::White));
    }

    #[test]
    fn rook_blocked_does_not_attack() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(0, 0, (Piece::Rook, Color::White)); // Ra1
        board.set(0, 3, (Piece::Pawn, Color::White)); // blocker on a4
        let target = Square { file: 0, rank: 7 }; // a8
        assert!(!board.is_square_attacked_by(target, Color::White));
    }

    #[test]
    fn king_attacks_adjacent_square() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 0, (Piece::King, Color::White)); // Ke1
        let target = Square { file: 5, rank: 1 }; // f2
        assert!(board.is_square_attacked_by(target, Color::White));
    }

    #[test]
    fn initial_position_not_in_check() {
        let board = Board::new();
        assert!(!board.is_in_check(Color::White));
        assert!(!board.is_in_check(Color::Black));
    }

    #[test]
    fn queen_gives_check() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 0, (Piece::King, Color::White)); // Ke1
        board.set(4, 7, (Piece::Queen, Color::Black)); // Qe8 attacks along file
        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn king_not_in_check_when_no_king() {
        let board = Board { squares: [[None; 8]; 8] };
        assert!(!board.is_in_check(Color::White));
    }

    /// Helper to play a sequence of moves on a fresh board for testing.
    fn play_moves(notations: &[&str]) -> Board {
        let mut board = Board::new();
        for (index, notation) in notations.iter().enumerate() {
            let chess_move = NotationMove::parse(notation, index).unwrap();
            let color = if index % 2 == 0 { Color::White } else { Color::Black };
            let resolved = board.resolve_move(&chess_move, notation, color).unwrap();
            board.apply_move(&resolved);
        }
        board
    }

    #[test]
    fn scholars_mate_is_checkmate() {
        // 1.e4 e5 2.Bc4 Nc6 3.Qh5 Nf6 4.Qxf7#
        let board = play_moves(&["e4", "e5", "Bc4", "Nc6", "Qh5", "Nf6", "Qxf7#"]);
        assert!(board.is_checkmate(Color::Black));
    }

    #[test]
    fn not_checkmate_when_king_can_escape() {
        // White queen checks but king has escape squares
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 7, (Piece::King, Color::Black));
        board.set(4, 5, (Piece::Queen, Color::White)); // Queen checks along file
        // King can move to d8, d7, f8, f7 — not checkmate
        assert!(!board.is_checkmate(Color::Black));
    }

    #[test]
    fn not_checkmate_when_piece_can_block() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 0, (Piece::King, Color::White));
        board.set(4, 7, (Piece::Rook, Color::Black)); // Rook checks along e-file
        board.set(1, 2, (Piece::Rook, Color::White)); // White rook can block on e2
        assert!(!board.is_checkmate(Color::White));
    }

    #[test]
    fn not_checkmate_when_attacker_can_be_captured() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(4, 0, (Piece::King, Color::White));
        board.set(4, 1, (Piece::Queen, Color::Black)); // Queen gives check on e2
        board.set(3, 0, (Piece::Rook, Color::White)); // King can't escape but...
        // King can capture the queen at e2
        assert!(!board.is_checkmate(Color::White));
    }

    #[test]
    fn not_checkmate_when_not_in_check() {
        let board = Board::new();
        assert!(!board.is_checkmate(Color::White));
        assert!(!board.is_checkmate(Color::Black));
    }

    #[test]
    fn back_rank_mate_is_checkmate() {
        let mut board = Board { squares: [[None; 8]; 8] };
        board.set(6, 0, (Piece::King, Color::White)); // Kg1
        board.set(5, 1, (Piece::Pawn, Color::White)); // Pf2
        board.set(6, 1, (Piece::Pawn, Color::White)); // Pg2
        board.set(7, 1, (Piece::Pawn, Color::White)); // Ph2
        board.set(0, 0, (Piece::Rook, Color::Black)); // Ra1 — back rank mate
        assert!(board.is_checkmate(Color::White));
    }

    #[test]
    fn initial_position_white_pawns() {
        let board = Board::new();
        for file in 0..8 {
            assert_eq!(board.get(file, 1), Some((Piece::Pawn, Color::White)));
        }
    }

    #[test]
    fn initial_position_black_pawns() {
        let board = Board::new();
        for file in 0..8 {
            assert_eq!(board.get(file, 6), Some((Piece::Pawn, Color::Black)));
        }
    }

    #[test]
    fn initial_position_white_back_rank() {
        let board = Board::new();
        assert_eq!(board.get(0, 0), Some((Piece::Rook, Color::White)));
        assert_eq!(board.get(1, 0), Some((Piece::Knight, Color::White)));
        assert_eq!(board.get(2, 0), Some((Piece::Bishop, Color::White)));
        assert_eq!(board.get(3, 0), Some((Piece::Queen, Color::White)));
        assert_eq!(board.get(4, 0), Some((Piece::King, Color::White)));
    }

    #[test]
    fn initial_position_empty_middle() {
        let board = Board::new();
        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board.get(file, rank), None);
            }
        }
    }

    #[test]
    fn apply_simple_move() {
        let mut board = Board::new();
        let parsed = ResolvedMove {
            origin: Square { file: 4, rank: 1 },
            dest: Square { file: 4, rank: 3 },
            promotion: None,
            castling_rook: None,
        };
        board.apply_move(&parsed);
        assert_eq!(board.get(4, 1), None);
        assert_eq!(board.get(4, 3), Some((Piece::Pawn, Color::White)));
    }

    #[test]
    fn apply_castling_kingside_white() {
        let mut board = Board::new();
        board.clear_square(5, 0);
        board.clear_square(6, 0);
        let parsed = ResolvedMove {
            origin: Square { file: 4, rank: 0 },
            dest: Square { file: 6, rank: 0 },
            promotion: None,
            castling_rook: Some((Square { file: 7, rank: 0 }, Square { file: 5, rank: 0 })),
        };
        board.apply_move(&parsed);
        assert_eq!(board.get(6, 0), Some((Piece::King, Color::White)));
        assert_eq!(board.get(5, 0), Some((Piece::Rook, Color::White)));
        assert_eq!(board.get(4, 0), None);
        assert_eq!(board.get(7, 0), None);
    }

    #[test]
    fn apply_promotion() {
        let mut board = Board::new();
        board.set(4, 6, (Piece::Pawn, Color::White));
        board.clear_square(4, 7);
        let parsed = ResolvedMove {
            origin: Square { file: 4, rank: 6 },
            dest: Square { file: 4, rank: 7 },
            promotion: Some(Piece::Queen),
            castling_rook: None,
        };
        board.apply_move(&parsed);
        assert_eq!(board.get(4, 7), Some((Piece::Queen, Color::White)));
        assert_eq!(board.get(4, 6), None);
    }

    #[test]
    fn find_origin_pawn_e4() {
        let board = Board::new();
        let dest = Square { file: 4, rank: 3 };
        let origin = board.find_origin(Piece::Pawn, &dest, Color::White, None, None);
        assert_eq!(origin, Some(Square { file: 4, rank: 1 }));
    }

    #[test]
    fn find_origin_knight_f3() {
        let board = Board::new();
        let dest = Square { file: 5, rank: 2 };
        let origin = board.find_origin(Piece::Knight, &dest, Color::White, None, None);
        assert_eq!(origin, Some(Square { file: 6, rank: 0 }));
    }

    #[test]
    fn find_origin_with_file_hint() {
        let mut board = Board::new();
        board.set(0, 3, (Piece::Rook, Color::White));
        board.set(7, 3, (Piece::Rook, Color::White));
        let dest = Square { file: 3, rank: 3 };
        let origin = board.find_origin(Piece::Rook, &dest, Color::White, Some(0), None);
        assert_eq!(origin, Some(Square { file: 0, rank: 3 }));
    }

    #[test]
    fn pawn_double_push_blocked() {
        let mut board = Board::new();
        board.set(4, 2, (Piece::Pawn, Color::Black));
        let dest = Square { file: 4, rank: 3 };
        let origin = board.find_origin(Piece::Pawn, &dest, Color::White, None, None);
        assert_eq!(origin, None);
    }

    #[test]
    fn bishop_blocked_by_piece() {
        let board = Board::new();
        let dest = Square { file: 0, rank: 2 };
        let origin = board.find_origin(Piece::Bishop, &dest, Color::White, None, None);
        assert_eq!(origin, None);
    }
}
