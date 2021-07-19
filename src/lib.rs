use petgraph::algo::toposort;
use petgraph::graph::DiGraph;

/// Produces proposed dates of birth and death such that the collected facts hold true
///
/// # Parameters
/// Let P_1, P_2, ..., P_n be a set of n deceased people.
/// If person P_a died before person P_b was born then `fact_form1` will contain `(a, b)`.
/// If person P_a and P_b's life span overlapped at least partially then `fact_form2` will contain `(a, b)`.
///
/// # Return value
/// If the facts are internally consistent, returns `Some((birth, death))` where
/// `birth[i - 1]` is the proposed birth date and `death[i - 1]` is the proposed death date of person P_i.
/// Returns `None` if the facts are not internally consistent.
///
/// # Complexity
/// `O(m + n)` where `m = fact_form1.len() + fact_form2.len()`.
pub fn proposed_dates(
    fact_form1: &[(u32, u32)],
    fact_form2: &[(u32, u32)],
    n: usize,
) -> Option<(Vec<usize>, Vec<usize>)> {
    enum Label {
        Birth(usize), // Birth(i) corresponds to label B(i)
        Death(usize), // Death(i) corresponds to label D(i)
    }

    let mut g = DiGraph::new();

    // construct g with nodes B(i) and D(i) for each P_i
    let birth_nodes: Vec<_> = (1..=n).map(|i| g.add_node(Label::Birth(i))).collect(); // birth_nodes[i - 1] is the node labeled B(i)
    let death_nodes: Vec<_> = (1..=n).map(|i| g.add_node(Label::Death(i))).collect(); // death_nodes[i - 1] is the node labeled D(i)

    // for each P_i insert (B(i), D(i))
    for i in 1..=n {
        g.add_edge(birth_nodes[i - 1], death_nodes[i - 1], ());
    }

    // for each first fact form (a, b) add edge (D(a), B(b))
    for (i, j) in fact_form1 {
        g.add_edge(
            death_nodes[(i - 1) as usize],
            birth_nodes[(j - 1) as usize],
            (),
        );
    }

    // for each second fact form (a, b) add edges (B(a), D(b)), (B(b), D(a))
    for (i, j) in fact_form2 {
        g.add_edge(
            birth_nodes[(i - 1) as usize],
            death_nodes[(j - 1) as usize],
            (),
        );
        g.add_edge(
            birth_nodes[(j - 1) as usize],
            death_nodes[(i - 1) as usize],
            (),
        );
    }

    toposort(&g, None).ok().map(|topo| {
        let mut proposed_births = vec![0; n];
        let mut proposed_deaths = vec![0; n];

        // iterate in topological order
        for (date, &event) in topo.iter().enumerate() {
            // assign date according to label
            match g[event] {
                Label::Birth(i) => proposed_births[i - 1] = date + 1,
                Label::Death(i) => proposed_deaths[i - 1] = date + 1,
            }
        }

        (proposed_births, proposed_deaths)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposed_dates_are_internally_consistent_1() {
        let fact_form1 = vec![(1, 2), (3, 4), (4, 6)];
        let fact_form2 = vec![(2, 3), (5, 6)];
        let n = 6;

        if let Some((birth, death)) = proposed_dates(&fact_form1, &fact_form2, n) {
            for (a, b) in &fact_form1 {
                // a died before b was born
                assert!(death[(a - 1) as usize] < birth[(b - 1) as usize]);
            }
            for (a, b) in &fact_form2 {
                assert!(
                    // a born before b died and b born before a died
                    birth[(a - 1) as usize] < death[(b - 1) as usize]
                        && birth[(b - 1) as usize] < death[(a - 1) as usize]
                );
            }
            // birth date comes before death date
            for i in 1..=n {
                assert!(birth[i - 1] < death[i - 1]);
            }
        } else {
            panic!("proposed dates not internally consistent");
        }
    }
}
