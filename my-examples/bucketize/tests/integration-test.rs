extern crate bucketize;
use bucketize::Bucketizer;

#[test]
fn multiple_buckets_open_ends() {
    let bucketizer = Bucketizer::new()
        .bucket(Some(0.0), Some(1.0), 0.5)
        .bucket(Some(1.0), None, 1.0);
    assert_eq!(bucketizer.bucketize(0.3), Some(0.5));
    assert_eq!(bucketizer.bucketize(999.99), Some(1.0));
    assert_eq!(bucketizer.bucketize(-999.99), None);
}
