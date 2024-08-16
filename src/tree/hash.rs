#[derive(Clone, Copy, Debug)]
pub struct HashEntry {
    pub hash: u64,
    #[allow(dead_code)]
    pub visits: f32,
    #[allow(dead_code)]
    pub wins: f32,
}

impl Default for HashEntry {
    fn default() -> Self {
        Self {
            hash: 0,
            visits: 0.0,
            wins: 0.0,
        }
    }
}

pub struct HashTable {
    size: usize,
    table: Vec<HashEntry>,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        HashTable {
            size,
            table: vec![HashEntry::default(); size],
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, hash: u64) -> Option<HashEntry> {
        let entry = self.table[(hash as usize) % self.size];

        if entry.hash == hash {
            Some(entry)
        } else {
            None
        }
    }

    pub fn insert(&mut self, hash: u64, visits: f32, wins: f32) {
        self.table[(hash as usize) % self.size] = HashEntry { hash, visits, wins };
    }
}
