use euclidean::Region;

#[test]
fn corners() {
    let region: Region = [4.5, 6.5, 8.5, 10.0].into();
    
    assert_eq!(region[0], 4.5); // x
    assert_eq!(region[1], 6.5); // y
    assert_eq!(region[2], 8.5); // width
    assert_eq!(region[3], 10.0); // height
    
    // top-left
    assert_eq!(region.top_left(), [4.5, 6.5].into());
    
    // top-right
    assert_eq!(region.top_right(), [13.0, 6.5].into());
    
    // bottom-left
    assert_eq!(region.bottom_left(), [4.5, 16.5].into());
    
    // bottom-right
    assert_eq!(region.bottom_right(), [13.0, 16.5].into());
}