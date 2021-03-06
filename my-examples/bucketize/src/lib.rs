//! this creates a crate level documentation

/// This block documents the following structure
///
/// # Example
/// ```
/// use bucketize::Bucketizer;
///
/// let b = Bucketizer::new()
///     .bucket(Some(10.0), Some(20.0), 15.0)
///     .bucket(Some(5.0), Some(10.0), 7.5)
///     .bucket(None, Some(4.0), 0.0);
///
/// assert_eq!(b.bucketize(12.34), Some(15.0));
/// assert_eq!(b.bucketize(9999.99), None);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    /// Create a new `Bucketizer`
    pub fn new() -> Self {
        Bucketizer {
            buckets: Vec::new()
        }
    }

    pub fn bucket(self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        let mut new = self;
        new.buckets.push((min, max, value));
        new
    }

    pub fn bucketize(&self, input: f64) -> Option<f64> {
        for bucket in &self.buckets {
            match *bucket {
                (None, None, value) => return Some(value),
                (Some(min), None, value) => {
                    if input >= min { return Some(value) }
                },
                (None, Some(max), value) => {
                    if input < max { return Some(value) }
                },
                (Some(min), Some(max), value) => {
                    if input >= min && input < max { return Some(value)}
                }
            }
        }

        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(0.7), Some(0.5));
    }

   #[test]
    fn single_bucket_end_values() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }

    #[test]
    fn multiple_buckets_closed_ends() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(-1.0), Some(0.0), -0.5)
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(-0.7), Some(-0.5));
        assert_eq!(bucketizer.bucketize(999.99), None);
    }

    #[test]
    fn multiple_buckets_open_ends() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5)
            .bucket(Some(1.0), None, 1.0);
        assert_eq!(bucketizer.bucketize(0.3), Some(0.5));
        assert_eq!(bucketizer.bucketize(999.99), Some(1.0));
        assert_eq!(bucketizer.bucketize(-999.99), None);
    }
}
