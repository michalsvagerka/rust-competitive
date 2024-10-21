//! Computes DFS order on a given rooted tree. Can be made the tree linear, that is 
//! for every vertex its ID is larger than that of his parent. This means that instead of 
//! recursive DFS, a simple DP can be used.
use crate::graph::tree::Tree;

pub struct DfsOrder<'a> {
    pub tree: &'a Tree,
    time: usize,
    pub rev_enter: Vec<usize>,
    pub enter_time: Vec<usize>,
    pub exit_time: Vec<usize>,
}

impl<'a> DfsOrder<'a> {
    pub fn new(tree: &'a Tree, root: usize) -> Self {
        let mut ans = Self {
            tree,
            rev_enter: vec![0; tree.vertex_count],
            time: 0,
            enter_time: vec![0; tree.vertex_count],
            exit_time: vec![0; tree.vertex_count],
        };

        ans.dfs(root, usize::MAX);
        assert_eq!(ans.time, tree.vertex_count);
        ans
    }

    fn dfs(&mut self, u: usize, p: usize) {
        self.rev_enter[self.time] = u;
        self.enter_time[u] = self.time;
        self.time += 1;
        for v in self.tree.neighbors(u) {
            if v != p {
                self.dfs(v, u);
            }
        }
        self.exit_time[u] = self.time;
    }

    pub fn linearize(&self) -> Tree {
        let mut ans = Tree::new(self.tree.vertex_count);
        for u in 0..self.tree.vertex_count {
            for v in self.tree.neighbors(u) {
                if u < v {
                    ans.add_edge(self.enter_time[u], self.enter_time[v])
                }
            }
        }
        ans
    }

    /// Use this to convert linearized vertex IDs to original vertex IDs, for example
    /// to print output.
    pub fn reverse(&self, u: usize) -> usize { self.rev_enter[u] }
}
