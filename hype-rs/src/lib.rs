pub use hype_derive::AsFeatureVector;
pub mod feature_vector;

pub mod prelude {
    use super::*;
    pub use feature_vector::FeatureVector;
    pub use feature_vector::AsFeatureVector;
    pub use hype_derive::AsFeatureVector;
}