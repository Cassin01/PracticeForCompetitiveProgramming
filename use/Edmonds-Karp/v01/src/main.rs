fn bfs(s: usize, t: usize, capacity: &Vec<Vec<i64>>) -> Option::<(i64, Vec<i64>)> {
    let parent = vec![-1; capacity.len()];
    parent[s] = -2;
    let mut q = Vec::new();
    q.push((s, i64::max_value()));

    while !(q.is_empty()) {
        let (cur, flow) = q.pop()?;

        for next in adj
    }
    None
}

fn max_flow(s: usize, t: usize, parent: Vec<i64>) -> (i64, Vec<i64>) {
    let mut max_flow = 0;
    let mut parent: Vec<usize> = Vec::new();
    let mut capacity = vec![vec![0; 10]; 10];

    loop {
        if let Some((flow, parent)) = bfs(s, t, &capacity) {
            max_flow += flow;
            let cur = t;
            while cur != s {
                let prev = parent[cur];
                capacity[prev][cur] -= flow;
                capacity[cur][prev] += flow;
                cur = prev;
            }
        } else {
            break;
        }
    }
    max_flow
}
