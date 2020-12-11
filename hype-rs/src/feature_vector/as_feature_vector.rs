use smallvec;
use crate::feature_vector::FeatureVector;

pub trait AsFeatureVector {
    fn feature_size(&self) -> usize;
    fn as_feature_vector(&self) -> FeatureVector;
}

macro_rules! impl_as_feature_for_integral_types {
    ( $( $type:ty ),+ ) => {
    $(
    impl AsFeatureVector for $type {
        #[inline]
        fn feature_size(&self) -> usize {
            1
        }

        fn as_feature_vector(&self) -> FeatureVector {
            FeatureVector::from_smallvec(smallvec::smallvec![*self as f32])
        }
    }
    )+
    };
}

impl_as_feature_for_integral_types!(isize, usize, f32, f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);