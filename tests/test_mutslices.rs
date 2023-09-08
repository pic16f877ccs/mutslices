use mutslices::MutSlice;
use trybuild::TestCases;

#[test]
#[should_panic]
fn test_range_start_eq_end() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let ranges = [(0, 4), (7, 7), (6, 12)];
    let mut mut_slice = MutSlice::vec_into(vec);
    #[allow(unused_variables)]
        let [slice_one, slice_two, slice_three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
}

#[test]
#[should_panic]
fn test_range_start_greater_end() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let ranges = [(0, 4), (8, 7), (6, 12)];
    let mut mut_slice = MutSlice::vec_into(vec);
    #[allow(unused_variables)]
        let [slice_one, slice_two, slice_three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
}

#[test]
#[should_panic]
fn test_range_overlap() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let ranges = [(0, 4), (4, 7), (6, 12)];
    let mut mut_slice = MutSlice::vec_into(vec);
    #[allow(unused_variables)]
        let [slice_one, slice_two, slice_three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
}

#[test]
#[should_panic]
fn test_range_end_gt_len() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let ranges = [(0, 4), (4, 7), (7, vec.len() + 5)];
    let mut mut_slice = MutSlice::vec_into(vec);
    #[allow(unused_variables)]
        let [slice_one, slice_two, slice_three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
}

#[test]
fn test_range_one_slice() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let ranges = [(0, vec.len())];
    let mut mut_slice = MutSlice::vec_into(vec);
    let [slice_one] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
    assert_eq!(slice_one, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8]);
}

#[test]
fn test_three_slice() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12_u8];
    let cvec = vec.clone();
    let ranges = [(0, 4), (4, 7), (7, 12)];
    let mut mut_slice = MutSlice::vec_into(vec);
    let [ref mut slice_one, ref mut slice_two, ref mut slice_three] = mut_slice.slices_mut(&ranges)[..] else { panic!(); };
    assert_eq!(*slice_one, &cvec[..4]);
    assert_eq!(*slice_two, &cvec[4..7]);
    assert_eq!(*slice_three, &cvec[7..]);
}

#[test]
fn test_three_mut_slice_var_one() {
    let vec = Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut mut_slice = MutSlice::vec_into(vec);
    let ranges = [(0, 5), (5, 10), (10, 15)];
    let [ref mut one, ref mut two, ref mut three] = mut_slice.slices_mut(&ranges)[..] else { panic!(); };
    one.swap_with_slice(two);
    two.swap_with_slice(three);
    three.swap_with_slice(one);
    let vec_from_slice = mut_slice.into_vec();
    assert_eq!(
        vec_from_slice,
        vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10_u8]
    );
}

#[test]
fn test_three_mut_slice_var_two() {
    let vec = Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut mut_slice = MutSlice::vec_into(vec);
    let ranges = [(0, 5), (5, 10), (10, 15)];
    let [one, two, three] = &mut mut_slice.slices_mut(&ranges)[..] else { panic!(); };
    one.swap_with_slice(two);
    two.swap_with_slice(three);
    three.swap_with_slice(one);
    let vec_from_slice = mut_slice.into_vec();
    assert_eq!(
        vec_from_slice,
        vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10_u8]
    );
}

#[test]
fn test_three_mut_slice_var_three() {
    let vec = Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut mut_slice = MutSlice::vec_into(vec);
    let ranges = [(0, 5), (5, 10), (10, 15)];
    let [one, two, three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
    one.swap_with_slice(two);
    two.swap_with_slice(three);
    three.swap_with_slice(one);
    let vec_from_slice = mut_slice.into_vec();
    assert_eq!(
        vec_from_slice,
        vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10_u8]
    );
}

#[test]
fn test_three_mut_slice_var_four() {
    let vec = Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut mut_slice = MutSlice::vec_into(vec);
    let ranges = [(0, 5), (5, 10), (10, 15)];
    let [ref mut one, ref mut two, ref mut three] = *mut_slice.slices_mut(&ranges) else { panic!(); };
    one.swap_with_slice(two);
    two.swap_with_slice(three);
    three.swap_with_slice(one);
    let vec_from_slice = mut_slice.into_vec();
    assert_eq!(
        vec_from_slice,
        vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10_u8]
    );
}

#[test]
fn test_clone_eq() {
    let vec = vec![10, 20, 30, 40, 50];
    let mut_slice = MutSlice::vec_into(vec);
    let mut_slice_cloned = mut_slice.clone();
    assert_eq!(mut_slice, mut_slice_cloned);
}

#[test]
#[ignore = "vec_move"]
fn test_vec_move() {
    let test_ui = TestCases::new();
    test_ui.compile_fail("tests/ui/*.rs");
}
