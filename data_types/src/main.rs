fn main() {
    
}

#[cfg(test)]
mod tests {

    //
    // ## Scalar Types
    //
    // Scalar type represents a single value. 
    //
    // ### Booleans
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

    #[test]
    fn test_integer_literals() {
        assert_eq!(2500, 2_500);
        assert_eq!(255, 0xff);
        assert_eq!(255, 0b11111111);
        assert_eq!(65, b'A');
    }
    #[test]
    fn test_integer_operators(){
        assert_eq!(3 + 2, 5);
        assert_eq!(3 - 2, 1);
        assert_eq!(3 * 2, 6);
        assert_eq!(6 / 3, 2);
        assert_eq!(5 % 3, 2);
        assert_eq!((2 as u32).pow(3), 8);
    }
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
    #[test]
    fn test_floats() {
        assert_eq!(f32::MIN, -3.4028235e38);
        assert_eq!(f32::MAX, 3.4028235e38);
        assert_eq!(f64::MIN, -1.7976931348623157e308);
        assert_eq!(f64::MAX, 1.7976931348623157e308);   
    }
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
    // Tuples have a fixed length. Each component
    // may be of a different type
    //
    #[test]
    fn test_tuples() {
        let t : (u8, char, bool) = (255,'A',true);
        assert_eq!(t.0, 255);
        assert_eq!(t.1, 'A');
        assert_eq!(t.2, true);
        let (x,y,z) = t;
        assert_eq!(x, 255);
        assert_eq!(y, 'A');
        assert_eq!(z, true);
        let (_,_,last) = t;
        assert_eq!(last, true);
        let unit = ();
        assert_eq!(unit, ());
    }
    //
    // ## Arrays
    //
    // Arrays are fixed length and their elements
    // have the same type
    //
    #[test]
    fn test_arrays() {
        let a = [1,2,3];
        let a_typed : [i32; 3] = [1,2,3];
        assert_eq!(a, a_typed);
        assert_eq!(a.len(), 3);
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 2);
        assert_eq!(a[2], 3);
        /* assert_eq!(a[3], 3); won't compile */
        let a_repeat = [0; 5];
        assert_eq!(a_repeat, [0,0,0,0,0]);

    }
    //
    // ## Variables
    //

    //
    // ### Immutable and Mutable Variables
    //
    #[test]
    fn test_variables() {
        let _a : u8 = 9;
        /* _a += 1; won't compile, variable is immutable */
        let mut b : u8 = 9;
        b += 1;
        assert_eq!(b, 10)
    }
    //
    // ### Constants
    //
    // They are inlined whenever they are evaluated
    //
    #[test]
    fn test_constants() {
        const MILLENNIUM_BUG : u16 = 2000;
        assert_eq!(MILLENNIUM_BUG, 2000);
    }
    //
    // ### Static variables
    //
    // They are stored in a fixed memory location and
    // referenced accordingly whenever evaluated.
    //
    static BEST_COMMODORE_COMPUTER : u8 = 64;
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
    // ### Variable Scope
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
    #[test]
    fn test_variable_scope_2() {
        let asterisks = ['*';5];
        /* The asterisks variable replaces the old one above */
        let asterisks = asterisks.len();
        assert_eq!(asterisks,5);
    }
}
