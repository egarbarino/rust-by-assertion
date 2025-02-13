fn main() {
}
//
// ## Slices
//
#[cfg(test)]
mod tests {
    //
    // ### String slices
    //
    #[test]
    fn test_slice_string() {

        let s = String::from("Hello world");

        let len = s.len();

        assert_eq!(&s[0..len],"Hello world");
        assert_eq!(&s[0..],"Hello world");
        assert_eq!(&s[..len],"Hello world");
        assert_eq!(&s[..5],"Hello");
        assert_eq!(&s[0..5],"Hello");
        assert_eq!(&s[0..=4],"Hello");
        assert_eq!(&s[6..],"world");
        assert_eq!(&s[6..len],"world");
        assert_eq!(&s[6..11],"world");
        assert_eq!(&s[6..=10],"world");
        assert_eq!(return_string_slice(&s),"Hello");
    }
    fn return_string_slice(s : &String) -> &str {
        &s[0..5]
    }
    //
    // ### Array slices
    //
    #[test]
    fn test_slice_regular_arrays() {

        let a = [1,2,3,4,5];

        let len = a.len();

        assert_eq!(&a[0..len],[1,2,3,4,5]);
        assert_eq!(&a[0..],[1,2,3,4,5]);
        assert_eq!(&a[..len],[1,2,3,4,5]);
        assert_eq!(&a[0..=4],[1,2,3,4,5]);
        assert_eq!(&a[..3],[1,2,3]);
        assert_eq!(&a[0..3],[1,2,3]);
        assert_eq!(&a[3..],[4,5]);
        assert_eq!(&a[3..len],[4,5]);
        assert_eq!(&a[3..5],[4,5]);
        assert_eq!(&a[3..=4],[4,5]);
        assert_eq!(return_array_slice(&a),[1,2,3]);
    }
    fn return_array_slice(a : &[u8]) -> &[u8] {
        &a[0..3]
    }
}