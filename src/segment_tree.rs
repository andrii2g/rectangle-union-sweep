use crate::geometry::length;

#[derive(Clone, Default)]
struct Node {
    count: usize,
    length: u128,
}

pub(crate) struct SegmentTree<'a> {
    ys: &'a [i64],
    nodes: Vec<Node>,
    pub visits: usize,
    pub max_depth: usize,
}

impl<'a> SegmentTree<'a> {
    pub fn new(ys: &'a [i64]) -> Self {
        Self {
            ys,
            nodes: vec![Node::default(); 4 * ys.len()],
            visits: 0,
            max_depth: 0,
        }
    }

    pub fn covered(&self) -> u128 {
        self.nodes[1].length
    }

    pub fn update(&mut self, start: usize, end: usize, entering: bool) {
        self.update_node(1, 0, self.ys.len() - 1, (start, end), entering, 0);
    }

    fn update_node(
        &mut self,
        index: usize,
        left: usize,
        right: usize,
        range: (usize, usize),
        entering: bool,
        depth: usize,
    ) {
        self.visits += 1;
        self.max_depth = self.max_depth.max(depth);
        if range.0 <= left && right <= range.1 {
            if entering {
                self.nodes[index].count += 1;
            } else {
                self.nodes[index].count -= 1;
            }
        } else {
            let mid = (left + right) / 2;
            if range.0 < mid {
                self.update_node(index * 2, left, mid, range, entering, depth + 1);
            }
            if range.1 > mid {
                self.update_node(index * 2 + 1, mid, right, range, entering, depth + 1);
            }
        }
        self.nodes[index].length = if self.nodes[index].count > 0 {
            length(self.ys[left], self.ys[right])
        } else if right - left == 1 {
            0
        } else {
            self.nodes[index * 2].length + self.nodes[index * 2 + 1].length
        };
    }

    pub fn intervals(&self) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        self.collect(1, 0, self.ys.len() - 1, &mut result);
        result
    }

    fn collect(&self, index: usize, left: usize, right: usize, result: &mut Vec<(i64, i64)>) {
        if self.nodes[index].length == 0 {
            return;
        }
        if self.nodes[index].count > 0 {
            let start = self.ys[left];
            let end = self.ys[right];
            if let Some(last) = result.last_mut()
                && last.1 == start
            {
                last.1 = end;
                return;
            }
            result.push((start, end));
        } else if right - left > 1 {
            let mid = (left + right) / 2;
            self.collect(index * 2, left, mid, result);
            self.collect(index * 2 + 1, mid, right, result);
        }
    }
}
