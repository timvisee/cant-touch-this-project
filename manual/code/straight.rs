#[test]
fn straight() {
    let points = PointTrace::new(vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(5.0, 5.0, 0.0),
    ]);

    let expected = RotTrace::new(vec![RotPoint::new(0f64, 2f64.sqrt())]);

    assert_eq!(points.to_rot_trace(false), expected);
}
