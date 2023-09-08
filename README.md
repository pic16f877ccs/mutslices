# Mutslices
 This crate provides the ability to create many mutable slices of a vector. Slicers are created from an array of ranges.

## Exsamples

```rust
use mutslices::MutSlice;

let vec = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
let mut mut_slice = MutSlice::vec_into(vec);
let ranges = [(0, 5), (5, 10), (10, 15)];
let [one, two, three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };

two.swap(2, 3);
one.iter_mut().for_each(|elem| *elem = *elem * 3);
one[0] = 77;
two[1] = 78;
one.swap_with_slice(two);
two.swap_with_slice(three);
three.swap_with_slice(one);
let vec_from_slice = mut_slice.into_vec();
assert_eq!(vec_from_slice, vec![77, 6, 9, 12, 15, 11, 12, 13, 14, 15, 6, 78, 9, 8, 10]);
```
