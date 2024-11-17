use crate::avl::AvlTree;

#[test]
fn lnr_mut() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }
    assert_eq!(7, avl.depth());
    assert_eq!(true, avl.contains(9));
    assert_eq!(false, avl.contains(100));
    avl.iter_lnr_mut().for_each(|f| *f += 2);
    assert_eq!(false, avl.contains(1));
    assert_eq!(true, avl.contains(10));
}

#[test]
fn lnr_order() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }
    avl.iter_lnr_mut().for_each(|f| *f += 1);
    assert_eq!(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        avl.into_iter_lnr().collect::<Vec<_>>().as_slice()
    );
}

#[test]
fn nrl_order() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }
    assert_eq!(
        [3, 7, 8, 9, 5, 6, 4, 1, 2, 0],
        avl.into_iter_nrl().collect::<Vec<_>>().as_slice()
    );
}

#[test]
fn rln_order() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }
    assert_eq!(
        [3, 1, 0, 2, 7, 5, 4, 6, 8, 9],
        avl.into_iter_rln().collect::<Vec<_>>().as_slice()
    );
}

#[test]
fn depth() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }
    assert_eq!(4, avl.depth());
    for i in 10..100 {
        avl.insert(i);
    }
    assert_eq!(7, avl.depth());
}
