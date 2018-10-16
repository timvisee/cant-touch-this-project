#[test]
fn corner() {
    let points = PointTrace::new(vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 5.0, 0.0),
        Point3::new(5.0, 5.0, 0.0),
        Point3::new(5.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
    ]);

    let expected = RotTrace::new(vec![RotPoint::from_degrees(-90.0, 5.0); 3]);

    assert_eq!(points.to_rot_trace(false), expected);
    assert_eq!(
        points.to_last_rot_point(),
        Some(RotPoint::from_degrees(-90.0, 5.0))
    );
}
