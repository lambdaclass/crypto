use super::{BTreeMap, MerkleError, MerklePath, NodeIndex, Rpo256, ValuePath, Vec};
use crate::{hash::rpo::RpoDigest, Word};

// MERKLE PATH SET
// ================================================================================================

/// A set of Merkle paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerklePathSet {
    root: RpoDigest,
    total_depth: u8,
    paths: BTreeMap<u64, MerklePath>,
}

impl MerklePathSet {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Returns an empty MerklePathSet.
    pub fn new(depth: u8) -> Self {
        let root = RpoDigest::default();
        let paths = BTreeMap::new();

        Self {
            root,
            total_depth: depth,
            paths,
        }
    }

    /// Appends the provided paths iterator into the set.
    ///
    /// Analogous to `[Self::add_path]`.
    pub fn with_paths<I>(self, paths: I) -> Result<Self, MerkleError>
    where
        I: IntoIterator<Item = (u64, RpoDigest, MerklePath)>,
    {
        paths.into_iter().try_fold(self, |mut set, (index, value, path)| {
            set.add_path(index, value.into(), path)?;
            Ok(set)
        })
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the root to which all paths in this set resolve.
    pub const fn root(&self) -> RpoDigest {
        self.root
    }

    /// Returns the depth of the Merkle tree implied by the paths stored in this set.
    ///
    /// Merkle tree of depth 1 has two leaves, depth 2 has four leaves etc.
    pub const fn depth(&self) -> u8 {
        self.total_depth
    }

    /// Returns a node at the specified index.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The specified index is not valid for the depth of structure.
    /// * Requested node does not exist in the set.
    pub fn get_node(&self, index: NodeIndex) -> Result<RpoDigest, MerkleError> {
        if index.depth() != self.total_depth {
            return Err(MerkleError::InvalidDepth {
                expected: self.total_depth,
                provided: index.depth(),
            });
        }

        let parity = index.value() & 1;
        let path_key = index.value() - parity;
        self.paths
            .get(&path_key)
            .ok_or(MerkleError::NodeNotInSet(index))
            .map(|path| path[parity as usize])
    }

    /// Returns a leaf at the specified index.
    ///
    /// # Errors
    /// * The specified index is not valid for the depth of the structure.
    /// * Leaf with the requested path does not exist in the set.
    pub fn get_leaf(&self, index: u64) -> Result<Word, MerkleError> {
        let index = NodeIndex::new(self.depth(), index)?;
        Ok(self.get_node(index)?.into())
    }

    /// Returns a Merkle path to the node at the specified index. The node itself is
    /// not included in the path.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The specified index is not valid for the depth of structure.
    /// * Node of the requested path does not exist in the set.
    pub fn get_path(&self, index: NodeIndex) -> Result<MerklePath, MerkleError> {
        if index.depth() != self.total_depth {
            return Err(MerkleError::InvalidDepth {
                expected: self.total_depth,
                provided: index.depth(),
            });
        }

        let parity = index.value() & 1;
        let path_key = index.value() - parity;
        let mut path =
            self.paths.get(&path_key).cloned().ok_or(MerkleError::NodeNotInSet(index))?;
        path.remove(parity as usize);
        Ok(path)
    }

    /// Returns all paths in this path set together with their indexes.
    pub fn to_paths(&self) -> Vec<(u64, ValuePath)> {
        let mut result = Vec::with_capacity(self.paths.len() * 2);

        for (&index, path) in self.paths.iter() {
            // push path for the even index into the result
            let path1 = ValuePath {
                value: path[0],
                path: MerklePath::new(path[1..].to_vec()),
            };
            result.push((index, path1));

            // push path for the odd index into the result
            let mut path2 = path.clone();
            let leaf2 = path2.remove(1);
            let path2 = ValuePath {
                value: leaf2,
                path: path2,
            };
            result.push((index + 1, path2));
        }

        result
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds the specified Merkle path to this [MerklePathSet]. The `index` and `value` parameters
    /// specify the leaf node at which the path starts.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The specified index is is not valid in the context of this Merkle path set (i.e., the
    ///   index implies a greater depth than is specified for this set).
    /// - The specified path is not consistent with other paths in the set (i.e., resolves to a
    ///   different root).
    pub fn add_path(
        &mut self,
        index_value: u64,
        value: Word,
        mut path: MerklePath,
    ) -> Result<(), MerkleError> {
        let mut index = NodeIndex::new(path.len() as u8, index_value)?;
        if index.depth() != self.total_depth {
            return Err(MerkleError::InvalidDepth {
                expected: self.total_depth,
                provided: index.depth(),
            });
        }

        // update the current path
        let parity = index_value & 1;
        path.insert(parity as usize, value.into());

        // traverse to the root, updating the nodes
        let root = Rpo256::merge(&[path[0], path[1]]);
        let root = path.iter().skip(2).copied().fold(root, |root, hash| {
            index.move_up();
            Rpo256::merge(&index.build_node(root, hash))
        });

        // if the path set is empty (the root is all ZEROs), set the root to the root of the added
        // path; otherwise, the root of the added path must be identical to the current root
        if self.root == RpoDigest::default() {
            self.root = root;
        } else if self.root != root {
            return Err(MerkleError::ConflictingRoots([self.root, root].to_vec()));
        }

        // finish updating the path
        let path_key = index_value - parity;
        self.paths.insert(path_key, path);
        Ok(())
    }

    /// Replaces the leaf at the specified index with the provided value.
    ///
    /// # Errors
    /// Returns an error if:
    /// * Requested node does not exist in the set.
    pub fn update_leaf(&mut self, base_index_value: u64, value: Word) -> Result<(), MerkleError> {
        let mut index = NodeIndex::new(self.depth(), base_index_value)?;
        let parity = index.value() & 1;
        let path_key = index.value() - parity;
        let path = match self.paths.get_mut(&path_key) {
            Some(path) => path,
            None => return Err(MerkleError::NodeNotInSet(index)),
        };

        // Fill old_hashes vector -----------------------------------------------------------------
        let mut current_index = index;
        let mut old_hashes = Vec::with_capacity(path.len().saturating_sub(2));
        let mut root = Rpo256::merge(&[path[0], path[1]]);
        for hash in path.iter().skip(2).copied() {
            old_hashes.push(root);
            current_index.move_up();
            let input = current_index.build_node(hash, root);
            root = Rpo256::merge(&input);
        }

        // Fill new_hashes vector -----------------------------------------------------------------
        path[index.is_value_odd() as usize] = value.into();

        let mut new_hashes = Vec::with_capacity(path.len().saturating_sub(2));
        let mut new_root = Rpo256::merge(&[path[0], path[1]]);
        for path_hash in path.iter().skip(2).copied() {
            new_hashes.push(new_root);
            index.move_up();
            let input = current_index.build_node(path_hash, new_root);
            new_root = Rpo256::merge(&input);
        }

        self.root = new_root;

        // update paths ---------------------------------------------------------------------------
        for path in self.paths.values_mut() {
            for i in (0..old_hashes.len()).rev() {
                if path[i + 2] == old_hashes[i] {
                    path[i + 2] = new_hashes[i];
                    break;
                }
            }
        }

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkle::{int_to_leaf, int_to_node};

    #[test]
    fn get_root() {
        let leaf0 = int_to_node(0);
        let leaf1 = int_to_node(1);
        let leaf2 = int_to_node(2);
        let leaf3 = int_to_node(3);

        let parent0 = calculate_parent_hash(leaf0, 0, leaf1);
        let parent1 = calculate_parent_hash(leaf2, 2, leaf3);

        let root_exp = calculate_parent_hash(parent0, 0, parent1);

        let set = super::MerklePathSet::new(2)
            .with_paths([(0, leaf0, vec![leaf1, parent1].into())])
            .unwrap();

        assert_eq!(set.root(), root_exp);
    }

    #[test]
    fn add_and_get_path() {
        let path_6 = vec![int_to_node(7), int_to_node(45), int_to_node(123)];
        let hash_6 = int_to_node(6);
        let index = 6_u64;
        let depth = 3_u8;
        let set = super::MerklePathSet::new(depth)
            .with_paths([(index, hash_6, path_6.clone().into())])
            .unwrap();
        let stored_path_6 = set.get_path(NodeIndex::make(depth, index)).unwrap();

        assert_eq!(path_6, *stored_path_6);
    }

    #[test]
    fn get_node() {
        let path_6 = vec![int_to_node(7), int_to_node(45), int_to_node(123)];
        let hash_6 = int_to_node(6);
        let index = 6_u64;
        let depth = 3_u8;
        let set = MerklePathSet::new(depth).with_paths([(index, hash_6, path_6.into())]).unwrap();

        assert_eq!(int_to_node(6u64), set.get_node(NodeIndex::make(depth, index)).unwrap());
    }

    #[test]
    fn update_leaf() {
        let hash_4 = int_to_node(4);
        let hash_5 = int_to_node(5);
        let hash_6 = int_to_node(6);
        let hash_7 = int_to_node(7);
        let hash_45 = calculate_parent_hash(hash_4, 12u64, hash_5);
        let hash_67 = calculate_parent_hash(hash_6, 14u64, hash_7);

        let hash_0123 = int_to_node(123);

        let path_6 = vec![hash_7, hash_45, hash_0123];
        let path_5 = vec![hash_4, hash_67, hash_0123];
        let path_4 = vec![hash_5, hash_67, hash_0123];

        let index_6 = 6_u64;
        let index_5 = 5_u64;
        let index_4 = 4_u64;
        let depth = 3_u8;
        let mut set = MerklePathSet::new(depth)
            .with_paths([
                (index_6, hash_6, path_6.into()),
                (index_5, hash_5, path_5.into()),
                (index_4, hash_4, path_4.into()),
            ])
            .unwrap();

        let new_hash_6 = int_to_leaf(100);
        let new_hash_5 = int_to_leaf(55);

        set.update_leaf(index_6, new_hash_6).unwrap();
        let new_path_4 = set.get_path(NodeIndex::make(depth, index_4)).unwrap();
        let new_hash_67 = calculate_parent_hash(new_hash_6.into(), 14_u64, hash_7);
        assert_eq!(new_hash_67, new_path_4[1]);

        set.update_leaf(index_5, new_hash_5).unwrap();
        let new_path_4 = set.get_path(NodeIndex::make(depth, index_4)).unwrap();
        let new_path_6 = set.get_path(NodeIndex::make(depth, index_6)).unwrap();
        let new_hash_45 = calculate_parent_hash(new_hash_5.into(), 13_u64, hash_4);
        assert_eq!(new_hash_45, new_path_6[1]);
        assert_eq!(RpoDigest::from(new_hash_5), new_path_4[0]);
    }

    #[test]
    fn depth_3_is_correct() {
        let a = int_to_node(1);
        let b = int_to_node(2);
        let c = int_to_node(3);
        let d = int_to_node(4);
        let e = int_to_node(5);
        let f = int_to_node(6);
        let g = int_to_node(7);
        let h = int_to_node(8);

        let i = Rpo256::merge(&[a, b]);
        let j = Rpo256::merge(&[c, d]);
        let k = Rpo256::merge(&[e, f]);
        let l = Rpo256::merge(&[g, h]);

        let m = Rpo256::merge(&[i, j]);
        let n = Rpo256::merge(&[k, l]);

        let root = Rpo256::merge(&[m, n]);

        let mut set = MerklePathSet::new(3);

        let value = b;
        let index = 1;
        let path = MerklePath::new([a, j, n].to_vec());
        set.add_path(index, value.into(), path).unwrap();
        assert_eq!(*value, set.get_leaf(index).unwrap());
        assert_eq!(root, set.root());

        let value = e;
        let index = 4;
        let path = MerklePath::new([f, l, m].to_vec());
        set.add_path(index, value.into(), path).unwrap();
        assert_eq!(*value, set.get_leaf(index).unwrap());
        assert_eq!(root, set.root());

        let value = a;
        let index = 0;
        let path = MerklePath::new([b, j, n].to_vec());
        set.add_path(index, value.into(), path).unwrap();
        assert_eq!(*value, set.get_leaf(index).unwrap());
        assert_eq!(root, set.root());

        let value = h;
        let index = 7;
        let path = MerklePath::new([g, k, m].to_vec());
        set.add_path(index, value.into(), path).unwrap();
        assert_eq!(*value, set.get_leaf(index).unwrap());
        assert_eq!(root, set.root());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    const fn is_even(pos: u64) -> bool {
        pos & 1 == 0
    }

    /// Calculates the hash of the parent node by two sibling ones
    /// - node — current node
    /// - node_pos — position of the current node
    /// - sibling — neighboring vertex in the tree
    fn calculate_parent_hash(node: RpoDigest, node_pos: u64, sibling: RpoDigest) -> RpoDigest {
        if is_even(node_pos) {
            Rpo256::merge(&[node, sibling])
        } else {
            Rpo256::merge(&[sibling, node])
        }
    }
}
