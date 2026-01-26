use crate::types::Square;

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

    fn sq(s: &str) -> Square {
        Square::parse(s).unwrap()
    }

    #[test]
    fn a4_c4() {
        assert_eq!(from_square(&sq("a4")), 262);
    }

    #[test]
    fn f4_a4() {
        assert_eq!(from_square(&sq("f4")), 440);
    }

    #[test]
    fn e4_g4() {
        assert_eq!(from_square(&sq("e4")), 392);
    }

    #[test]
    fn b4_d4() {
        assert_eq!(from_square(&sq("b4")), 294);
    }

    #[test]
    fn g4_b4() {
        assert_eq!(from_square(&sq("g4")), 494);
    }

    #[test]
    fn h4_c5() {
        assert_eq!(from_square(&sq("h4")), 523);
    }

    #[test]
    fn octave_up_a5() {
        assert_eq!(from_square(&sq("a5")), 524);
    }

    #[test]
    fn octave_down_a3() {
        assert_eq!(from_square(&sq("a3")), 131);
    }

    #[test]
    fn two_octaves_up() {
        assert_eq!(from_square(&sq("a6")), 1048);
    }

    #[test]
    fn two_octaves_down() {
        assert_eq!(from_square(&sq("a2")), 65);
    }
}
