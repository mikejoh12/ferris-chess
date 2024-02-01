#[derive(Debug, PartialEq, Clone)]
pub struct Cache {
    // Lots of complicated fields.
    pub rook_rays: Vec<Vec<Vec<usize>>>,
    pub bishop_rays: Vec<Vec<Vec<usize>>>,
    pub knight_targets: Vec<Vec<usize>>,
}

impl Cache {
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }
}

#[derive(Default)]
pub struct CacheBuilder {}

impl CacheBuilder {
    pub fn build(self) -> Cache {
        Cache { rook_rays: self.get_board_rook_rays(), bishop_rays: self.get_board_bishop_rays(), knight_targets: self.get_board_knight_targets()}
    }

    fn get_board_rook_rays(&self) -> Vec<Vec<Vec<usize>>> {
        let mut rays = vec![];
        for pos in 0..64 {
            rays.push(self.get_rook_rays(pos));
        }
        rays
    }

    fn get_rook_rays(&self, pos: usize) -> Vec<Vec<usize>> {
        let mut rook_rays: Vec<Vec<usize>> = vec![];

        let mut up: Vec<usize> = vec![];
        let mut up_pos = pos.checked_sub(8);
        while let Some(v) = up_pos {
            up.push(v);
            up_pos = v.checked_sub(8);
        }
        rook_rays.push(up);

        let mut down: Vec<usize> = vec![];
        let mut down_pos = pos + 8;
        while down_pos < 64 {
            down.push(down_pos);
            down_pos += 8;
        }
        rook_rays.push(down);

        let file_idx = pos % 8;

        let mut right: Vec<usize> = vec![];
        for p in (pos + 1)..(pos + 8 - file_idx) {
            right.push(p);
        }
        rook_rays.push(right);

        let mut left: Vec<usize> = vec![];
        for p in ((pos - file_idx)..pos).rev() {
            left.push(p);
        }
        rook_rays.push(left);

        rook_rays
    }

    fn get_board_bishop_rays(&self) -> Vec<Vec<Vec<usize>>> {
        let mut rays = vec![];
        for pos in 0..64 {
            rays.push(self.get_bishop_rays(pos));
        }
        rays
    }

    fn get_bishop_rays(&self, pos: usize) -> Vec<Vec<usize>> {
        let mut bishop_rays: Vec<Vec<usize>> = vec![];

        let mut down_left: Vec<usize> = vec![];
        let mut down_left_pos = pos.checked_sub(9);
        while let Some(v) = down_left_pos {
            if v % 8 == 7 {
                break;
            }
            down_left.push(v);
            down_left_pos = v.checked_sub(9);
        }
        bishop_rays.push(down_left);

        let mut down_right: Vec<usize> = vec![];
        let mut down_right_pos = pos.checked_sub(7);
        while let Some(v) = down_right_pos {
            if v % 8 == 0 {
                break;
            }
            down_right.push(v);
            down_right_pos = v.checked_sub(7);
        }
        bishop_rays.push(down_right);

        let mut up_right: Vec<usize> = vec![];
        let mut up_right_pos = pos + 9;
        while up_right_pos < 64 {
            if up_right_pos % 8 == 0 {
                break;
            }
            up_right.push(up_right_pos);
            up_right_pos += 9;
        }
        bishop_rays.push(up_right);

        let mut up_left: Vec<usize> = vec![];
        let mut up_left_pos = pos + 7;
        while up_left_pos < 64 {
            if up_left_pos % 8 == 7 {
                break;
            }
            up_left.push(up_left_pos);
            up_left_pos += 7;
        }
        bishop_rays.push(up_left);

        bishop_rays
    }

    fn get_board_knight_targets(&self) -> Vec<Vec<usize>> {
        let mut rays = vec![];
        for pos in 0..64 {
            rays.push(self.get_knight_targets(pos));
        }
        rays
    }

    fn get_knight_targets(&self, pos: usize) -> Vec<usize> {
        let mut targets: Vec<usize> = vec![];
        let rank_idx = pos / 8;
        let file_idx = pos % 8;

        let offsets: [[isize; 2]; 8] = [
            [2, -1],
            [2, 1],
            [1, 2],
            [-1, 2],
            [-2, 1],
            [-2, -1],
            [-1, -2],
            [1, -2],
        ];
        for [rank_offset, file_offset] in offsets {
            let new_rank = rank_idx as isize + rank_offset;
            let new_file = file_idx as isize + file_offset;
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let new_pos = new_rank as usize * 8 + new_file as usize;
                targets.push(new_pos);
            }
        }

        targets
    }
}