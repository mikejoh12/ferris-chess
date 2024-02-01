#[derive(Debug, PartialEq, Clone)]
pub struct Cache {
    // Lots of complicated fields.
    pub rook_rays: Vec<Vec<Vec<usize>>>,
    pub bishop_rays: Vec<Vec<Vec<usize>>>,
}

impl Cache {
    // This method will help users to discover the builder
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }
}

#[derive(Default)]
pub struct CacheBuilder {
    // Probably lots of optional fields.
    // bar: String,
}

impl CacheBuilder {
    pub fn new(/* ... */) -> CacheBuilder {
        // Set the minimally required fields of Cache.
        CacheBuilder {
            // bar: String::from("X"),
        }
    }

    pub fn name(mut self, /* bar: String */) -> CacheBuilder {
        // Set the name on the builder itself, and return the builder by value.
        // self.bar = bar;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    pub fn build(self) -> Cache {
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder
        // to Foo.
        Cache { rook_rays: self.get_board_rook_rays(), bishop_rays: self.get_board_bishop_rays()}
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
}


/*
#[derive(Debug, Clone, PartialEq)]
pub struct Targets {
    rook_rays: Vec<Vec<Vec<usize>>>,
}

impl Targets {
    pub fn new() -> Targets {
        Targets {
            rook_rays: self.get_board_rook_rays(),
        }
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
}

*/