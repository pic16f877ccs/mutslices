#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
//! This crate provides the ability to create many mutable slices of a vector.
//! Slicers are created from an array of ranges.
//!
//! # Examples
//! 
//! ```
//! use mutslices::MutSlice;
//!
//! let vec = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
//! let mut mut_slice = MutSlice::vec_into(vec);
//! let ranges = [(0, 5), (5, 10), (10, 15)];
//! let [one, two, three] = &mut*mut_slice.slices_mut(&ranges) else { panic!(); };
//!
//! two.swap(2, 3);
//! one.iter_mut().for_each(|elem| *elem = *elem * 3);
//! one[0] = 77;
//! two[1] = 78;
//! one.swap_with_slice(two);
//! two.swap_with_slice(three);
//! three.swap_with_slice(one);
//! let vec_from_slice = mut_slice.into_vec();
//! assert_eq!(vec_from_slice, vec![77, 6, 9, 12, 15, 11, 12, 13, 14, 15, 6, 78, 9, 8, 10]);
//! ```
#[doc = include_str!("../README.md")]
use std::mem::forget;
use std::slice::from_raw_parts_mut;

/// Structure to store the moved vector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutSlice<T> {
    vec: Vec<T>,
}

impl<T> MutSlice<T> {
    /// Moves the vector into the structure.
    ///
    /// # Examples
    /// 
    /// ```
    /// use mutslices::MutSlice;
    ///
    /// let vec = vec![10, 20, 30, 40, 50];
    /// let mut mut_slice = MutSlice::vec_into(vec);
    /// ```
    #[inline]
    pub fn vec_into(mut vec: Vec<T>) -> Self {
        let (ptr, length, capacity) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
        forget(vec);
        Self {
            vec: unsafe { Vec::from_raw_parts(ptr, length, capacity) },
        }
    }

    /// Creates mutable slices.
    ///
    /// # Panics
    ///
    /// Panic if range out of the vector bounds or
    /// ranges overlap or
    /// the end of the range is smaller than the beginning.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutslices::MutSlice;
    ///
    /// let vec = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    /// let mut mut_slice = MutSlice::vec_into(vec);
    /// let ranges = [(0, 3), (3, 5), (5, 10)];
    /// let [ref mut one, ref mut two, ref mut three] = mut_slice.slices_mut(&ranges)[..] else { panic!(); };
    /// one[0] = 11;
    /// two[1] = 55;
    /// three[2] = 88;
    /// assert_eq!(*one, &vec![11, 2, 3]);
    /// assert_eq!(*two, &vec![4, 55]);
    /// assert_eq!(*three, &vec![6, 7, 88, 9, 10]);
    /// ```
    #[inline]
    pub fn slices_mut(&mut self, ranges: &[(usize, usize)]) -> Vec<&mut [T]> {
        let mut vec = Vec::new();
        let mut range_end = 0;
        for range in ranges.iter() {
            let (start, end) = *range;

            if self.vec.len() < end {
                panic!("Index out of range!");
            } else if start < range_end {
                panic!("Ranges should not overlap.");
            } else if start >= end {
                panic!("Range end must be larger ot equal");
            }

            unsafe {
                vec.push(from_raw_parts_mut(
                    self.vec.as_mut_ptr().add(start),
                    end - start,
                ))
            };
            range_end = end;
        }
        vec
    }

    /// Reconstructs a vector from a structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutslices::MutSlice;
    ///
    /// let vec = Vec::from([0, 8, 0, 9, 2, 0, 2, 3]);
    /// let mut mut_slice = MutSlice::vec_into(vec);
    /// let ranges = [(0, 2), (2, 4), (4, 8)];
    /// let [day, month, year] = &mut mut_slice.slices_mut(&ranges)[..] else { panic!(); };
    /// day[1] = 1;
    /// month[1] = 1;
    /// year[3] = 4;
    ///
    /// let rvec = mut_slice.into_vec();
    /// assert_eq!(rvec, vec![0, 1, 0, 1, 2, 0, 2, 4]);
    /// ```
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
        unsafe {
            let (ptr, length, capacity) =
                (self.vec.as_mut_ptr(), self.vec.len(), self.vec.capacity());
            forget(self.vec);
            Vec::from_raw_parts(ptr, length, capacity)
        }
    }
}
