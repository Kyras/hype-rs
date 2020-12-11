use smallvec::SmallVec;

const FEATURE_ARRAY_SIZE: usize = 10;

pub type FeatureType = f32;
pub type FeatureArray = [FeatureType; FEATURE_ARRAY_SIZE];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FeatureVector(smallvec::SmallVec<FeatureArray>);

impl FeatureVector {
    pub fn empty() -> Self {
        Self(smallvec::SmallVec::new())
    }

    pub(crate) fn from_smallvec(vec: SmallVec<FeatureArray>) -> Self {
        Self(vec)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(smallvec::SmallVec::with_capacity(capacity))
    }

    pub fn push(&mut self, value: FeatureType) {
        self.0.push(value)
    }

    pub fn extend<I: IntoIterator<Item=FeatureType>>(&mut self, values: I) {
        self.0.extend(values)
    }
}

impl IntoIterator for FeatureVector {
    type Item = FeatureType;
    type IntoIter = smallvec::IntoIter<FeatureArray>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}