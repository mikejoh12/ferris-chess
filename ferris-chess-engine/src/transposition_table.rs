use ferris_chess_board::MoveData;

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TTableData {
    pub zobrist: u64,
    pub best_move: Option<MoveData>,
    pub depth: usize,
    pub score: i32,
    pub node_type: NodeType,
}

pub struct TranspositonTable {
    pub data: Vec<Option<TTableData>>,
    pub entries: u64,
}

const TABLE_SIZE: usize = 10_000_000;

impl TranspositonTable {
    pub fn new() -> Self {
        let data: Vec<Option<TTableData>> = vec![None; TABLE_SIZE as usize];

        TranspositonTable { data, entries: 0 }
    }

    pub fn get(&self, zobrist_hash: u64, depth: usize) -> Option<TTableData> {
        if let Some(info) = self.data[zobrist_hash as usize % TABLE_SIZE] {
            if info.zobrist == zobrist_hash && depth <= info.depth {
                // Clone for now. Todo: Optimize
                Some(info.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_perft_data(&self, zobrist_hash: u64, depth: usize) -> Option<TTableData> {
        if let Some(info) = self.data[zobrist_hash as usize % TABLE_SIZE] {
            if info.zobrist == zobrist_hash && depth == info.depth {
                // Clone for now. Todo: Optimize
                Some(info.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.data = vec![None; TABLE_SIZE];
        self.entries = 0;
    }

    pub fn insert(&mut self, data: TTableData) {
        let idx = data.zobrist as usize % TABLE_SIZE;

        if self.data[idx].is_none() {
            self.entries += 1;
        }

        self.data[idx] = Some(data);
    }
}
