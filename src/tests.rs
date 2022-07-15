use super::*;

#[test]
fn test_compute_cycles() {
    let cycle_set = CycleSet::compute(1);
    assert_eq!(cycle_set.cycles, vec![vec![0]]);

    let cycle_set = CycleSet::compute(10);
    assert_eq!(
        cycle_set.cycles,
        vec![
            vec![0],
            vec![
                0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7, 8, 5, 3, 8,
                1, 9, 0, 9, 9, 8, 7, 5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3, 2, 5,
                7, 2, 9, 1
            ],
            vec![0, 2, 2, 4, 6, 0, 6, 6, 2, 8, 0, 8, 8, 6, 4, 0, 4, 4, 8, 2],
            vec![0, 5, 5],
            vec![1, 3, 4, 7, 1, 8, 9, 7, 6, 3, 9, 2],
            vec![2, 6, 8, 4]
        ]
    );
    assert_eq!(cycle_set.pairs.len(), 100);

    let cycle_set = CycleSet::compute(123);
    assert_eq!(cycle_set.pairs.len(), 15_129);
}
