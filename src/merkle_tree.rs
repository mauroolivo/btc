use std::fmt;
use num::integer::{div_floor};
use num::pow;
use crate::helpers::merkle_hash::merkle_parent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MerkleTree {
    total: usize,
    max_depth: usize,
    nodes: Vec<Vec<Vec<u8>>>,
    current_depth: usize,
    current_index: usize
}
impl MerkleTree {
    pub fn new(total: usize) -> Self {
        let max_depth = ((total as f32).log2().ceil()) as usize;
        let mut nodes: Vec<Vec<Vec<u8>>> = vec![];
        for depth in 0..(max_depth+1) {
            let num_items = (total as f32 / pow(2, max_depth - depth) as f32).ceil() as usize;
            let hash = vec![0u8];
            let level_hashes = vec![hash; num_items];
            nodes.push(level_hashes);
        }
        MerkleTree {total, max_depth, nodes, current_depth:0, current_index:0}
    }
    pub fn up(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
            self.current_index = div_floor(self.current_index, 2);
        }
    }
    pub fn left(&mut self) {
        self.current_depth += 1;
        self.current_index *= 2;
    }
    pub fn right(&mut self) {
        self.current_depth += 1;
        self.current_index = self.current_index * 2 + 1;
    }
    pub fn root(&self) -> Vec<u8> {
        self.nodes[0][0].clone()
    }
    pub fn set_current_node(&mut self, value: Vec<u8>) {
        self.nodes[self.current_depth][self.current_index] = value;
    }
    pub fn get_current_node(&self) -> Vec<u8> {
        self.nodes[self.current_depth][self.current_index].clone()
    }
    pub fn get_left_node(&self) -> Vec<u8> {
        self.nodes[self.current_depth + 1][self.current_index * 2].clone()
    }
    pub fn get_right_node(&self) -> Vec<u8> {
        self.nodes[self.current_depth + 1][self.current_index * 2 + 1].clone()
    }
    pub fn is_leaf(&self) -> bool {
        self.current_depth == self.max_depth
    }
    pub fn right_exists(&self) -> bool {
        self.nodes[self.current_depth + 1].len() > self.current_index * 2 + 1
    }
    pub fn populate_tree(&mut self, mut flag_bits: Vec<u8>, mut hashes: Vec<Vec<u8>>) {
        while self.root() == vec![0] {
            if self.is_leaf() {
                flag_bits.remove(0);
                self.set_current_node(hashes.remove(0));
                self.up();
            } else {
                let left_hash = self.get_left_node();
                if left_hash == vec![0] {
                    if flag_bits.remove(0) == 0 {
                        self.set_current_node(hashes.remove(0));
                        self.up();
                    } else {
                        self.left()
                    }
                } else if self.right_exists() {
                    let right_hash = self.get_right_node();
                    if right_hash == vec![0] {
                        self.right()
                    } else {
                        self.set_current_node(merkle_parent(left_hash, right_hash));
                        self.up();
                    }
                } else {
                    self.set_current_node(merkle_parent(left_hash.clone(), left_hash));
                    self.up();
                }
            }
        }
        if hashes.len() != 0 {
            panic!("hashes not all consumed: {}", hashes.len());
        }
        for flag_bit in flag_bits {
            if flag_bit != 0 {
                panic!("flag bits not all consumed");
            }
        }
    }
}
impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut result = String::new();
        for (_idx, level) in self.nodes.iter().enumerate() {
            for hash in level {
                let value = hex::encode(hash);
                if value.len() > 4 {
                    let head = &value.as_str()[..4];
                    result.push_str(head);
                } else {
                    result.push_str(value.as_str());
                }
                result.push_str(" ");
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}
#[cfg(test)]
mod tests {
    use crate::helpers::merkle_hash::{merkle_parent, merkle_parent_level};
    use super::*;
    #[test]
    fn test_merkle_tree_display() {
        let tree = MerkleTree::new(27);
        println!("{}", tree);
    }
    #[test]
    fn test_merkle_tree_hashes() {
        let hex_hashes = [
            "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
            "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
            "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
            "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
            "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
            "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
            "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
            "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
            "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
            "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
            "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
            "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
            "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
            "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
            "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
            "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
        ];
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| hex::decode(x).unwrap()).collect();
        let mut tree = MerkleTree::new(hex_hashes.len());
        tree.nodes[4] = hashes;
        tree.nodes[3] = merkle_parent_level(tree.nodes[4].as_mut());
        tree.nodes[2] = merkle_parent_level(tree.nodes[3].as_mut());
        tree.nodes[1] = merkle_parent_level(tree.nodes[2].as_mut());
        tree.nodes[0] = merkle_parent_level(tree.nodes[1].as_mut());
        println!("{}", tree);
    }
    #[test]
    fn test_merkle_tree_build() {
        let hex_hashes = [
            "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
            "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
            "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
            "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
            "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
            "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
            "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
            "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
            "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
            "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
            "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
            "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
            "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
            "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
            "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
            "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
        ];
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| hex::decode(x).unwrap()).collect();
        let mut tree = MerkleTree::new(hex_hashes.len());
        tree.nodes[4] = hashes;
        while tree.root() == vec![0] {
            if tree.is_leaf() {
                tree.up()
            } else {
                let left_hash = tree.get_left_node();
                let right_hash = tree.get_right_node();
                if left_hash == vec![0] {
                    tree.left()
                } else if right_hash == vec![0] {
                    tree.right()
                } else {
                    tree.set_current_node(merkle_parent(left_hash,right_hash));
                    if tree.current_depth > 0 {
                        tree.up()
                    }
                }
            }
        }
        println!("{}", tree);
    }
    #[test]
    fn test_merkle_tree_build_odd() {
        let hex_hashes = [
            "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
            "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
            "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
            "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
            "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
            "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
            "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
            "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
            "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
            "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
            "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
            "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
            "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
            "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
            "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
            "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
            "38faf8c811988dff0a7e6080b1771c97bcc0801c64d9068cffb85e6e7aacaf51",
        ];
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| hex::decode(x).unwrap()).collect();
        let mut tree = MerkleTree::new(hex_hashes.len());
        tree.nodes[5] = hashes;
        while tree.root() == vec![0] {
            if tree.is_leaf() {
                tree.up()
            } else {
                let left_hash = tree.get_left_node();
                if left_hash == vec![0] {
                    tree.left()
                } else if tree.right_exists() {
                    let right_hash = tree.get_right_node();
                    if right_hash == vec![0] {
                        tree.right()
                    } else {
                        tree.set_current_node(merkle_parent(left_hash,right_hash));
                        if tree.current_depth > 0 {
                            tree.up()
                        }
                    }
                } else {
                    tree.set_current_node(merkle_parent(left_hash.clone(),left_hash));
                    if tree.current_depth > 0 {
                        tree.up()
                    }
                }
            }
        }
        println!("{}", tree);
    }
    #[test]
    fn test_merkle_tree_init() {
        let tree = MerkleTree::new(9);
        assert_eq!(tree.nodes[0].len(), 1);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 3);
        assert_eq!(tree.nodes[3].len(), 5);
        assert_eq!(tree.nodes[4].len(), 9);
    }
    #[test]
    fn test_populate_tree_1() {
        let hex_hashes = [
            "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
            "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
            "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
            "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
            "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
            "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
            "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
            "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
            "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
            "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
            "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
            "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
            "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
            "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
            "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
            "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
        ];
        let mut tree = MerkleTree::new(hex_hashes.len());
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| hex::decode(x).unwrap()).collect();

        let flag_bits: Vec<u8> = vec![1;31];
        tree.populate_tree(flag_bits, hashes);
        let want = "597c4bafe3832b17cbbabe56f878f4fc2ad0f6a402cee7fa851a9cb205f87ed1";
        let root = tree.root();
        assert_eq!(hex::encode(root), want)
    }
    #[test]
    fn test_populate_tree_2() {
        let  hex_hashes = [
            "42f6f52f17620653dcc909e58bb352e0bd4bd1381e2955d19c00959a22122b2e",
            "94c3af34b9667bf787e1c6a0a009201589755d01d02fe2877cc69b929d2418d4",
            "959428d7c48113cb9149d0566bde3d46e98cf028053c522b8fa8f735241aa953",
            "a9f27b99d5d108dede755710d4a1ffa2c74af70b4ca71726fa57d68454e609a2",
            "62af110031e29de1efcad103b3ad4bec7bdcf6cb9c9f4afdd586981795516577",
        ];
        let mut tree = MerkleTree::new(hex_hashes.len());
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| hex::decode(x).unwrap()).collect();
        let flag_bits: Vec<u8> = vec![1;11];
        tree.populate_tree(flag_bits, hashes);
        let want = "a8e8bd023169b81bc56854137a135b97ef47a6a7237f4c6e037baed16285a5ab";
        let root = tree.root();
        assert_eq!(hex::encode(root), want)
    }
}