use num::pow;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MerkleTree {
    total: usize,
    max_depth: usize,
    nodes: Vec<Vec<u8>>,
    current_depth: usize,
    current_index: usize
}
impl MerkleTree {
    pub fn new(total: usize) -> Self {
        let max_depth = ((total as f32).log2().ceil()) as usize;
        let mut nodes: Vec<Vec<u8>> = vec![];
        for depth in 0..(max_depth+1) {
            let num_items = pow(2, max_depth - depth);

            let hash = vec![0u8];
            nodes = vec![hash; num_items];

        }
        MerkleTree {total, max_depth, nodes, current_depth:0, current_index:0}
    }
}


// self.total = total
// self.max_depth = math.ceil(math.log(self.total, 2))
// self.nodes = []
// for depth in range(self.max_depth + 1):
// num_items = math.ceil(self.total / 2**(self.max_depth - depth))
// level_hashes = [None] * num_items
// self.nodes.append(level_hashes)
// self.current_depth = 0  # <1>
// self.current_index = 0