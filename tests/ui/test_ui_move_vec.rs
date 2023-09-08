use mutslices::MutSlice;

fn main() {
        let vec = Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let mut mut_slice = MutSlice::vec_into(vec);
        let ranges = [(0, 5), (5, 10), (10, 15)];
        let [ref mut one, ref mut two, ref mut three] = *mut_slice.slices_mut(&ranges) else { panic!(); };
        one.swap_with_slice(two);
        two.swap_with_slice(three);
        three.swap_with_slice(one);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

