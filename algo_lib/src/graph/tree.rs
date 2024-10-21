use crate::misc::logceil::logceil;
use std::mem::swap;

pub struct Tree {
    pub vertex_count: usize,
    pub edge_count: usize,
    pub edges: Vec<Vec<usize>>,
}

impl Tree {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            vertex_count,
            edge_count: 0,
            edges: vec![vec![]; vertex_count],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.vertex_count);
        assert!(v < self.vertex_count);

        self.edges[u].push(v);
        self.edges[v].push(u);
        self.edge_count += 1;
    }

    pub fn neighbors<'a>(&'a self, u: usize) -> impl Iterator<Item=usize> + 'a {
        self.edges[u].iter().copied()
    }

    pub fn root(&self, root: usize) -> RootedTree {
        RootedTree::new(self, root)
    }
}


pub struct RootedTree<'a> {
    pub tree: &'a Tree,
    pub parents: Vec<usize>,
    pub depth: Vec<usize>,
}

impl<'a> RootedTree<'a> {
    pub fn new(tree: &'a Tree, root: usize) -> Self {
        assert!(root < tree.vertex_count);
        assert_eq!(tree.edge_count, tree.vertex_count - 1);

        let mut ans = Self {
            tree,
            parents: vec![0; tree.vertex_count],
            depth: vec![0; tree.vertex_count],
        };

        ans.dfs(root, usize::MAX, 0);
        ans
    }

    fn dfs(&mut self, u: usize, v: usize, depth: usize) {
        self.parents[u] = v;
        self.depth[u] = depth;
        for &w in &self.tree.edges[u] {
            if v != w {
                self.dfs(w, u, depth + 1)
            }
        }
    }

    pub fn depth(&self, u: usize) -> usize { self.depth[u] }

    pub fn parent(&self, u: usize) -> usize { self.parents[u] }
}


pub struct LevelAncestry<'a> {
    tree: &'a RootedTree<'a>,
    log_vertex_count: usize,
    parents_exp: Vec<Vec<usize>>,
}

impl<'a> LevelAncestry<'a> {
    pub fn new(tree: &'a RootedTree) -> Self {
        let vertex_count = tree.tree.vertex_count;
        let log_vertex_count = logceil(vertex_count);
        let mut ans = Self {
            tree,
            log_vertex_count,
            parents_exp: vec![vec![usize::MAX; vertex_count]; log_vertex_count],
        };

        ans.parents_exp[0] = ans.tree.parents.clone();
        for i in 1..log_vertex_count {
            for j in 0..vertex_count {
                if ans.parents_exp[i - 1][j] != usize::MAX {
                    ans.parents_exp[i][j] = ans.parents_exp[i - 1][ans.parents_exp[i - 1][j]];
                }
            }
        }
        ans
    }

    pub fn is_ancestor(&self, top: usize, bottom: usize) -> bool {
        self.lca(top, bottom) == top
    }

    pub fn same_branch(&self, a: usize, b: usize) -> bool {
        let l = self.lca(a, b);
        l == a || l == b
    }

    pub fn level_ancestry(&self, mut u: usize, d: usize) -> usize {
        assert!(u < self.tree.tree.vertex_count);

        for i in 0..self.log_vertex_count {
            if (d & (1 << i)) != 0 {
                u = self.parents_exp[i][u];
                if u == usize::MAX { return u; }
            }
        }

        u
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.tree.tree.vertex_count);
        assert!(v < self.tree.tree.vertex_count);

        let mut u_depth = *unsafe { self.tree.depth.get_unchecked(u) };
        let mut v_depth = *unsafe { self.tree.depth.get_unchecked(v) };
        if u_depth < v_depth {
            swap(&mut u, &mut v);
            swap(&mut u_depth, &mut v_depth);
        }
        u = self.level_ancestry(u, u_depth - v_depth);
        for i in (0..self.log_vertex_count).rev() {
            let up = unsafe { *self.parents_exp.get_unchecked(i).get_unchecked(u) };
            let vp = unsafe { *self.parents_exp.get_unchecked(i).get_unchecked(u) };
            if up != vp {
                u = up;
                v = vp;
            }
        }

        if u == v {
            u
        } else {
            unsafe { *self.parents_exp.get_unchecked(0).get_unchecked(u) }
        }
    }
}
