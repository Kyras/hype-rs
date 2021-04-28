pub mod feature_vector;

pub mod prelude {
    use super::*;

    pub use feature_vector::FeatureVector;
    pub use feature_vector::AsFeatureVector;
    #[cfg(feature = "derive")]
    pub use hype_derive::AsFeatureVector;
}