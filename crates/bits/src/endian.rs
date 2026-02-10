// use crate::bit_slice::{BitLike, BitSliceExt};
//
// pub struct LeSlice<T>([T]);
//
// impl<T: BitLike> BitSliceExt for LeSlice<T> {
//     type Bit = T;
//     type Iter<'a> = ()
//     where
//         Self: 'a;
//
//     fn len(&self) -> usize {
//         todo!()
//     }
//
//     fn is_empty(&self) -> bool {
//         todo!()
//     }
//
//     fn get_opt(&self, idx: usize) -> Option<Self::Bit> {
//         todo!()
//     }
//
//     fn iter(&self) -> Self::Iter<'_> {
//         todo!()
//     }
// }
//
// pub struct BeSlice<T>([T]);
//
// pub struct NeSlice<T>([T]);
