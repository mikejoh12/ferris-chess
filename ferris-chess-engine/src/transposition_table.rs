struct TTableData {
    zobrist: u64,
    score: i64,
}

pub struct TranspositonTable {
    data: Vec<TTableData>
}

impl TranspositonTable {

    pub fn new() -> Self {
        let data: Vec<TTableData> = Vec::with_capacity(100_000);

        TranspositonTable { data }
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(100_000);
    }
}