#[allow(unused)]
type Bitboard = u64;

fn main() {}

fn _get_relevant_bishop_occupancy(square: u8) -> Bitboard {
    let mut test_index: i32 = i32::from(square) + 7;
    let mut test_occupancy = 0x0;
    let nw_occupancy = loop {
        if (test_index % 8) < ((test_index - 7) % 8) && test_index < 64  {
            test_occupancy |= 0x1 << test_index;
        } else {
            break test_occupancy;
        }

        test_index += 7;
    };

    test_index = i32::from(square) + 9;
    let ne_occupancy = loop {
        if (test_index % 8) > ((test_index - 9) % 8) && test_index < 64 {
            test_occupancy |= 0x1 << test_index;
        } else {
            break test_occupancy;
        }

        test_index += 9;
    };

    test_index = i32::from(square) - 7;
    let se_occupancy = loop {
        if (test_index % 8) > ((test_index + 7) % 8) && test_index >= 0 {
            test_occupancy |= 0x1 << test_index;
        } else {
            break test_occupancy;
        }

        test_index -= 7;
    };

    test_index = i32::from(square) - 9;
    let sw_occupancy = loop {
        if (test_index % 8) < ((test_index + 9) % 8) && test_index >= 0 {
            test_occupancy |= 0x1 << test_index;
        } else {
            break test_occupancy;
        }

        test_index -= 9;
    };

    (nw_occupancy | ne_occupancy | se_occupancy | sw_occupancy) & 0x007E7E7E7E7E7E00
}

fn _get_relevant_rook_occupancy(square: u8) -> Bitboard {
    let occupied: Bitboard = 0x1 << square;

    let mut counter = 0;
    let file_occupancy = loop {
        let test_occupancy = 0x0101010101010101 << counter;
    
        if (test_occupancy & occupied) != 0 {
            let test_occupancy = test_occupancy & 0x00FFFFFFFFFFFF00;
            break test_occupancy;
        }

        counter += 1;

        if counter > 7 {
            break 0;
        }
    };

    counter = 0;

    let rank_occupancy = loop {
        let test_occupancy = 0x00000000000000FF << (counter * 8);
    
        if (test_occupancy & occupied) != 0 {
            let test_occupancy = test_occupancy & 0x7E7E7E7E7E7E7E7E;
            break test_occupancy;
        }

        counter += 1;

        if counter > 7 {
            break 0;
        }
    };
    (file_occupancy | rank_occupancy) & !occupied
}

fn _get_bishop_attacks(from_index: u8, occupancy: Bitboard) -> Bitboard {
    _get_ray_attacks(from_index, |r: i32| -> i32 { r + 1 }, |c: i32| -> i32 { c - 1 }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r + 1 }, |c: i32| -> i32 { c + 1 }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r - 1 }, |c: i32| -> i32 { c - 1 }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r - 1 }, |c: i32| -> i32 { c + 1 }, occupancy)
}

fn _get_rook_attacks(from_index: u8, occupancy: Bitboard) -> Bitboard {
    _get_ray_attacks(from_index, |r: i32| -> i32 { r + 1 }, |c: i32| -> i32 { c }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r }, |c: i32| -> i32 { c + 1 }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r - 1 }, |c: i32| -> i32 { c }, occupancy) |
    _get_ray_attacks(from_index, |r: i32| -> i32 { r }, |c: i32| -> i32 { c - 1 }, occupancy)
}

fn _get_ray_attacks<F, G>(square: u8, row_op: F, col_op: G, occupancy: Bitboard) -> Bitboard
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32
{
    let mut cur_row = i32::from(square / 8);
    let mut cur_col = i32::from(square % 8);
    let mut attacks: Bitboard = 0x0;

    cur_row = row_op(cur_row);
    cur_col = col_op(cur_col);

    loop {
        if cur_row >= 0 && cur_row < 8 && cur_col >= 0 && cur_col < 8 {
            let mask = 0x1 << ((cur_row * 8) + cur_col);
            let is_blocker = (mask & occupancy) != 0x0;

            if is_blocker {
                attacks |= mask;

                break attacks;
            }

            attacks |= mask;

            cur_row = row_op(cur_row);
            cur_col = col_op(cur_col);
        }
        else {
            break attacks;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_relevant_rook_occupancy_a1() {
        let result = _get_relevant_rook_occupancy(0);
        assert_eq!(result, 0x000101010101017E);
    }

    #[test]
    fn get_relevant_rook_occupancy_h8() {
        let result = _get_relevant_rook_occupancy(63);
        assert_eq!(result, 0x7E80808080808000);
    }

    #[test]
    fn get_relevant_rook_occupancy_e4() {
        let result = _get_relevant_rook_occupancy(28);
        assert_eq!(result, 0x001010106E101000);
    }

    #[test]
    fn get_relevant_bishop_occupancy_a1() {
        let result = _get_relevant_bishop_occupancy(0);
        assert_eq!(result, 0x0040201008040200);
    }

    #[test]
    fn get_relevant_bishop_occupancy_f7() {
        let result = _get_relevant_bishop_occupancy(53);
        assert_eq!(result, 0x0000500804020000);
    }

    #[test]
    fn get_bishop_attacks_a1_with_d4_blocker() {
        let friendly_occupancy = 0x0000000008000000;
        let result = _get_bishop_attacks(0, 0x0000000008000001) & (!friendly_occupancy);
        assert_eq!(result, 0x0000000000040200);
    }

    #[test]
    fn get_bishop_attacks_e4_with_c2_blocker_c6_capture() {
        let friendly_occupancy = 0x0000000000000400;
        let result = _get_bishop_attacks(28, 0x0000040010000400) & (!friendly_occupancy);
        assert_eq!(result, 0x0080442800284080);
    }

    #[test]
    fn get_rook_attacks_a8_with_no_blockers() {
        let friendly_occupancy = 0x0100000000000000;
        let result = _get_rook_attacks(56, 0x0100000000000000) & (!friendly_occupancy);
        assert_eq!(result, 0xFE01010101010101);
    }

    #[test]
    fn get_rook_attacks_h6_with_d6_h2_blockers() {
        let friendly_occupancy = 0x0000880000008000;
        let result = _get_rook_attacks(47, 0x0000880000008000) & (!friendly_occupancy);
        assert_eq!(result, 0x8080708080800000);
    }

    #[test]
    fn get_rook_attacks_d5_with_f5_blocker_d7_capture() {
        let friendly_occupancy = 0x0000002800000000;
        let result = _get_rook_attacks(35, 0x0008002800000000) & (!friendly_occupancy);
        assert_eq!(result, 0x0008081708080808);
    }
}
