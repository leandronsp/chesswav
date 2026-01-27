use crate::chess::Square;

const BASE_FREQ: [u32; 8] = [
    262, // a -> C
    294, // b -> D
    330, // c -> E
    349, // d -> F
    392, // e -> G
    440, // f -> A
    494, // g -> B
    523, // h -> C (octave up)
];

pub fn from_square(square: &Square) -> u32 {
    let base = BASE_FREQ[square.file as usize];
    let octave_diff = (square.rank as i32) - 3;

    if octave_diff > 0 {
        base << octave_diff
    } else if octave_diff < 0 {
        base >> (-octave_diff)
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A2: Square = Square { file: 0, rank: 1 };
    const A3: Square = Square { file: 0, rank: 2 };
    const A4: Square = Square { file: 0, rank: 3 };
    const A5: Square = Square { file: 0, rank: 4 };
    const A6: Square = Square { file: 0, rank: 5 };
    const B4: Square = Square { file: 1, rank: 3 };
    const E4: Square = Square { file: 4, rank: 3 };
    const F4: Square = Square { file: 5, rank: 3 };
    const G4: Square = Square { file: 6, rank: 3 };
    const H4: Square = Square { file: 7, rank: 3 };

    #[test]
    fn a4_c4() {
        assert_eq!(from_square(&A4), 262);
    }

    #[test]
    fn f4_a4() {
        assert_eq!(from_square(&F4), 440);
    }

    #[test]
    fn e4_g4() {
        assert_eq!(from_square(&E4), 392);
    }

    #[test]
    fn b4_d4() {
        assert_eq!(from_square(&B4), 294);
    }

    #[test]
    fn g4_b4() {
        assert_eq!(from_square(&G4), 494);
    }

    #[test]
    fn h4_c5() {
        assert_eq!(from_square(&H4), 523);
    }

    #[test]
    fn octave_up_a5() {
        assert_eq!(from_square(&A5), 524);
    }

    #[test]
    fn octave_down_a3() {
        assert_eq!(from_square(&A3), 131);
    }

    #[test]
    fn two_octaves_up() {
        assert_eq!(from_square(&A6), 1048);
    }

    #[test]
    fn two_octaves_down() {
        assert_eq!(from_square(&A2), 65);
    }
}
