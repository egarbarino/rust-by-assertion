fn main() {
    
}

#[cfg(test)]
mod tests {

    //
    // # Data Types
    //
    // ## Scalar Types
    //
    // A scalar type represents a single value. 
    //
    // ### Booleans
    //
    // Booleans use the `true` and `false` literals and can be reversed
    // using `!`.
    //
    #[test]
    fn test_booleans() {
        assert_eq!(true, !false);
        assert_eq!(false, !true);
        let t : bool = true;
        assert_eq!(t, true);
    }
    //
    // ### Integers
    //
    // If integers are unsigned, their type is prefixed with `u`,
    // otherwise, they are prefixed with `i` (as in 'regular' integer).
    // After the `u` or `i` prefix, their size in bits is indicated,
    // ranging from 8 to 128.
    //
    // The `usize` type represents the architecture-dependant integer type
    // that is valid for addressing purposes. In contemporary computers,
    // this is equivalent to `u64`.
    //
    #[test]
    fn test_unsigned_integers() {
        assert_eq!(u8::MIN, 0);
        assert_eq!(u8::MAX, 255);
        assert_eq!(u16::MIN, 0);
        assert_eq!(u16::MAX, 65535);
        assert_eq!(u32::MIN, 0);
        assert_eq!(u32::MAX, 4294967295);
        assert_eq!(u64::MIN, 0);
        assert_eq!(u64::MAX, 18446744073709551615);
        assert_eq!(usize::MIN, 0); /* Arch dependant */
        assert_eq!(usize::MAX, 18446744073709551615); /* Arch dependant */
        assert_eq!(u128::MIN, 0);
        assert_eq!(u128::MAX, 340282366920938463463374607431768211455);
    }
    #[test]
    fn test_signed_integers() {
        assert_eq!(i8::MIN, -128);
        assert_eq!(i8::MAX, 127);
        assert_eq!(i16::MIN, -32768);
        assert_eq!(i16::MAX, 32767);
        assert_eq!(i32::MIN, -2147483648);
        assert_eq!(i32::MAX, 2147483647);
        assert_eq!(i64::MIN, -9223372036854775808);
        assert_eq!(i64::MAX, 9223372036854775807);
        assert_eq!(isize::MIN, -9223372036854775808); /* Arch dependant */
        assert_eq!(isize::MAX, 9223372036854775807); /* Arch dependant */
        assert_eq!(i128::MIN, -170141183460469231731687303715884105728);
        assert_eq!(i128::MAX, 170141183460469231731687303715884105727);
    }
    //
    // Integer literals may be expressed as follows:
    //
    #[test]
    fn test_integer_literals() {
        assert_eq!(2500, 2_500);      /* Thousands separeted by _ */
        assert_eq!(255, 0xff);        /* Hexadecimal              */
        assert_eq!(255, 0b11111111);  /* Binary                   */
        assert_eq!(65, b'A');         /* Character code           */
    }
    //
    // Regular operators are implemented as in most languages,
    // except for power.
    //
    #[test]
    fn test_integer_operators(){
        assert_eq!(3 + 2, 5);
        assert_eq!(3 - 2, 1);
        assert_eq!(3 * 2, 6);
        assert_eq!(6 / 3, 2);
        assert_eq!(5 % 3, 2);             /* modulo */
        assert_eq!((2 as u32).pow(3), 8); /* power  */
    }
    //
    // Unlike other languages, Rust treats the exceeding of
    // an integer's size as an overflow. If a wrapping behaviour
    // is desired, the `wrapping_add` method could be used, instead.
    //
    #[test]
    fn test_integer_wrapping() {
        let mut a : u8 = 255;
        /* a += 1; panics */
        a = a.wrapping_add(1);
        assert_eq!(a, 0);
    }
    //
    // ### Floats
    //
    // Rust implements floats using the IEEE 754 standard.
    //
    #[test]
    fn test_floats() {
        assert_eq!(f32::MIN, -3.4028235e38);
        assert_eq!(f32::MAX, 3.4028235e38);
        assert_eq!(f64::MIN, -1.7976931348623157e308);
        assert_eq!(f64::MAX, 1.7976931348623157e308);   
    }
    //
    // Operators behave like in most conventional languages,
    // except for power which requires the `powf` function.
    //
    #[test]
    fn test_float_operators(){
        assert_eq!(0.7 + 0.3, 1.0);
        assert_eq!(1.5 - 0.5, 1.0);
        assert_eq!(2.5 * 3.0, 7.5);
        assert_eq!(6.0 / 2.5, 2.4);
        assert_eq!((1.5 as f32).powf(2.0), 2.25);
    }
    //
    // ### Characters
    //
    // Characters in Rust aren't bytes. They use a 32-bit
    // integer-like type which can encode unicode characters.
    //
    #[test]
    fn test_characters() {
        let c1 = 'A';
        let c2: char = 'A';
        assert_eq!(c1, c2);
        assert_eq!(c1 as u32, 65);
        assert_eq!(char::MIN, '\0');
        assert_eq!(char::MIN as u32, 0);
        assert_eq!(char::MAX, '\u{10ffff}');
        assert_eq!(char::MAX as u32, 1114111);
        assert_eq!('😊' as u32, 0x1F60A);
    }

    //
    // ## Tuples
    //
    // Tuples have a fixed length. Each component may be of a different type.
    // Tuples are both constructed and deconstructed in Rust 
    // using the `(c1, c2, ...)` 
    // notation like in many other languages.
    //
    // Components may be extracted individually using the
    // `tuple.index` notation, starting from zero.
    //
    #[test]
    fn test_tuples() {

        /* Tuple construction with explicit type information */
        let t : (u8, char, bool) = (255,'A',true); 

        /* Query components by coordinate index */
        assert_eq!(t.0, 255);
        assert_eq!(t.1, 'A');
        assert_eq!(t.2, true);

        /* Deconstruction: assign components to variables */
        let (x,y,z) = t; 
        assert_eq!(x, 255);
        assert_eq!(y, 'A');
        assert_eq!(z, true);

        /* Deconstruction: using wild card _ to ignore components */
        let (_,_,last) = t; 
        assert_eq!(last, true);

        /* Construction of Unit tuple: zero components */
        let unit = (); 
        assert_eq!(unit, ());
    }
    //
    // ## Arrays
    //
    // Arrays have a fixed length and their elements
    // have the same type as in most statically-typed
    // languages. The length is checked using the `len`
    // method, while elements are obtained using the
    // standard `array[element_index]` notation.
    //
    #[test]
    fn test_arrays() {
        /* Type information is optional */
        let a = [1,2,3];
        let a_typed : [u8; 3] = [1,2,3];
        assert_eq!(a, a_typed);
        assert_eq!(a.len(), 3);

        /* Obtaining elements by index */
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 2);
        assert_eq!(a[2], 3);
        /* assert_eq!(a[3], 3); won't compile */

        /* Filling an array with repeated values */
        let a_repeat = [0; 5];
        assert_eq!(a_repeat, [0,0,0,0,0]);

    }
    //
    // ## Slices
    //
    // A slice in Rust, similarly to Go, acts as a 'view' upon an array. Such
    // a view consists of a start and end indices. A slice, as the name suggests,
    // allow manipulating subsets of an array without the need of having to
    // allocate a new one. 
    //
    // Assuming we have an array in hand, obtaining a slice involves addressing
    // it by reference and specifying the desired 
    // range: `&array[start_index..end_index+1]`.
    //
    //
    // ### Array slices
    //
    #[test]
    fn test_slice_regular_arrays() {

        /* Index            0 1 2 3 4 */
        let a : [u8 ; 5] = [1,2,3,4,5];

        let len = a.len(); 
        assert_eq!(len,5);

        assert_eq!(&a[0..len],[1,2,3,4,5]); /* From 0 to the 5-1 (4) */  
        assert_eq!(&a[0..],[1,2,3,4,5]);    /* From 0 to the end of the array */
        assert_eq!(&a[..len],[1,2,3,4,5]);  /* From the start to 5-1 (4) */ 
        assert_eq!(&a[0..=4],[1,2,3,4,5]);  /* From 0 to 4 */
        assert_eq!(&a[..3],[1,2,3]);        /* From the start to 3-1 (2) */
        assert_eq!(&a[0..3],[1,2,3]);       /* From 0 to 3-1 (2) */
        assert_eq!(&a[3..],[4,5]);          /* From 3 to the end of the array */
        assert_eq!(&a[3..len],[4,5]);       /* From 3 to 5-1 (4) */
        assert_eq!(&a[3..5],[4,5]);         /* From 3 to 5-1 (4) */
        assert_eq!(&a[3..=4],[4,5]);        /* From 3 to 4 */
        assert_eq!(return_array_slice(&a),[1,2,3]);
    }
    //
    // Note that the input is a fixed-size array but the output
    // is a size-independent slice 
    //
    fn return_array_slice(a : &[u8 ; 5]) -> &[u8] {
        &a[0..3]
    }
    //
    // ### String slices
    //
    // There isn't much special about string aliases, except
    // that the string slice type is `&str` rather than `&String`
    //
    #[test]
    fn test_slice_string() {

        /* Index              0123456789 */
        /*                              10 */
        let s = String::from("Hello world");

        let len = s.len();
        assert_eq!(len, 11);

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
    //
    // Note that the slice for a string is `&str` rather than `&String`
    //
    fn return_string_slice(s : &String) -> &str {
        &s[0..5]
    }

    //
    // # Variables
    //
    // ## Constants
    //
    // Constants are inlined whenever they are evaluated rather than
    // being stored in a single memory location. 
    //
    const MILLENNIUM_BUG : u16 = 2000;

    #[test]
    fn test_constants() {
        assert_eq!(MILLENNIUM_BUG, 2000);
    }
    //
    // ## Immutable and Mutable Variables
    //
    // Variables are declared using the `let` keyword and are 
    // immutable by default unless the `mut` modifier is included.
    //
    #[test]
    fn test_variables() {
        let _a : u8 = 9;       /* Immutable by default */
        /* _a += 1; won't compile, variable is immutable */
        let mut b : u8 = 9;    /* Made mutable by adding 'mut' */
        b += 1;
        assert_eq!(b, 10)
    }

    //
    // ## Static variables
    //
    // Static variables are stored in a fixed memory location and
    // referenced accordingly whenever evaluated.
    //
    static BEST_COMMODORE_COMPUTER : u8 = 64;

    /* Unsafe; avoid this if possible */
    static mut INITIAL_TEMPERATURE : i8 = -5;
    
    #[test]
    fn test_static_variables() {
        assert_eq!(BEST_COMMODORE_COMPUTER, 64);
       
        /* Use Mutex mechanism or similar to be safe */
        unsafe {
            INITIAL_TEMPERATURE += 7;
            assert_eq!(INITIAL_TEMPERATURE, 2);
        }
    }
    //
    // ## Variable Scope
    //
    // As in most C-like languages, curly braces introduce a new scope.
    //
    #[test]
    fn test_variable_scope_1() {
        let x = 1;
        assert_eq!(x, 1);
        {
            /* This is a different x */
            let x = 2;
            assert_eq!(x, 2);
        }
        /* This is the original x */
        assert_eq!(x, 1);
    }
    //
    // Unlike, say, pure functional languages, variable names may be
    // reused within the same scope. Please note that it is the variable
    // name that is being reused, rather than the type it can accommodate.
    //
    #[test]
    fn test_variable_scope_2() {
        let asterisks = ['*';5];         /* Here asterisks is an array */
        let asterisks = asterisks.len(); /* Here asterisks is an integer */
        assert_eq!(asterisks,5);
    }
}
