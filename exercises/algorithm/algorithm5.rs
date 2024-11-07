use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // 创建一个新的图，n 是节点数量
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加一条边
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // BFS 遍历，返回节点访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![]; // 用来存储访问的顺序
        let mut visited = vec![false; self.adj.len()]; // 记录节点是否被访问
        let mut queue = VecDeque::new(); // 队列，用于BFS

        // 初始化：从 start 节点开始
        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 记录当前访问的节点

            // 遍历该节点的所有邻接节点
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记该节点已访问
                    queue.push_back(neighbor); // 将未访问的邻居节点加入队列
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

