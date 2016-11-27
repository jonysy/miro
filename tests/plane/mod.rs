use euclidean::Region;

#[test]
fn corners() {
    let region: Region = [4.5, 6.5, 8.5, 1.0].into();
    
    assert_eq!(region[0], 4.5);
    assert_eq!(region[1], 6.5);
    assert_eq!(region[2], 8.5);
    assert_eq!(region[3], 1.0);
}