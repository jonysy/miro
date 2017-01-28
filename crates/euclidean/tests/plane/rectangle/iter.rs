use euclidean::Region;

#[test]
fn iter() {
    let region: Region<_> = [5, 7, 9, 10].into();
    
    let mut iter = region.iter();
    
    for y in 7..10 {
        for x in 5..9 {
            assert_eq!((x, y), iter.next().unwrap());
        }
    }
}