
use lazy_static::lazy_static;
lazy_static! {
    /// Zero nodes to act as "synthetic" left and right subtrees of other zero nodes.
    static ref ZERO_NODES: Vec<MerkleTree> = {
        (0..=32).map(MerkleTree::Zero).collect()
    };
}

/// Right-sparse Merkle tree.
///
/// Efficiently represents a Merkle tree of fixed depth where only the first N
/// indices are populated by non-zero leaves (perfect for the deposit contract tree).
#[derive(Debug, PartialEq)]
pub enum MerkleTree {
    /// It represents a Merkle tree of 2^depth zero leaves.
    Zero(usize),
}
fn main(){
    println!("{:?}",ZERO_NODES.as_slice())
}