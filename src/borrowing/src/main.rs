fn main() {
}
#[cfg(test)]
mod tests {
    //
    // ## Borrowing
    //
    // Borrowing is the process used by the Rust compiler to
    // avoid race conditions and detect when it is safe to
    // dispose of allocated memory without the need for
    // explicit allocation nor deallocation.
    //
    // ### Variable Move
    //
    // Variables that hold pointers are invalidated
    // if assigned to a new variables. In the below
    // example, the contents of s1 have _moved_ to
    // s2.
    //
    #[test]
    fn test_local_variable_move() {
        let s1 = String::from("hello");
        let mut s2 = s1;
        /* s1.push_str(" world") Invalid operation*/
        s2.push_str(" world");
        assert_eq!(s2, "hello world");
    }
    //
    // ### Variable Ownership Passed to Function
    //
    // The ownership of a variable is lost if passed
    // to a function
    //

    #[test]
    fn test_variable_move_to_function_problem() {
        let s1 = String::from("hello");
        check_not_empty(s1);
        /* assert_eq!(s1, "hello"); invalid operation */
    }
    /* Ensure s1 is not empty */
    fn check_not_empty(s : String) {
        assert!(!s.is_empty());
    }
    //
    // One option is return the ownership back 
    //

    #[test]
    fn test_variable_move_to_function_return_solution() {
        let s1 = String::from("hello");
        let s2 = check_not_empty_and_return(s1);
        assert_eq!(s2,"hello");
    }
    /* Ensure s1 is not empty */
    fn check_not_empty_and_return (s : String) -> String {
        assert!(!s.is_empty());
        s
    }
    
    //
    // One other option is to pass a reference. This
    // is what _borrowing_ means.
    //

    #[test]
    fn test_variable_move_to_function_referece_solution() {
        let s1 = String::from("hello");
        check_not_empty_by_reference(&s1);
        assert_eq!(s1,"hello");
    }
    /* Ensure s1 is not empty */
    fn check_not_empty_by_reference (s : &String) {
        assert!(!s.is_empty());
    }

    //
    // However, functions that have borrowed a value,
    // need to explicit about their mutable arguments in
    // order to change them.
    //

    #[test]
    fn test_variable_move_to_function_borrowing_error() {
        let mut s1 = String::from("hello");
        append_world(&mut s1);
        assert_eq!(s1,"hello world");
    }
    /* Mutate variable */
    fn append_world (s : &mut String) {
        s.push_str(" world");
        assert!(!s.is_empty());
    }
    //
    // Also, there can only be one single mutable
    // reference at any time
    //

    #[test]
    fn test_variable_move_to_function_ref_limit_1() {
        let mut s1 = String::from("hello");
        let r1_mutable = &mut s1;
        /* let r2_mutable = &mut s; Invalid */
        assert_eq!(r1_mutable,"hello");
    }
    //
    // But there is no limit is the references are 
    // immutable
    //
    #[test]
    fn test_variable_move_to_function_ref_limit_2() {
        let s1 = String::from("hello");
        let r1 = &s1;
        let r2 = &s1;
        assert_eq!(r1,"hello");
        assert_eq!(r2,"hello");

    }
    //
    // ### Clone
    //
    // The contents held by a variable may be cloned
    // to 'avoid' the borrower's restriction. 
    //
    #[test]
    fn test_clone() {
        let mut s1 = String::from("hello");
        let mut s2 = s1.clone();
        s1.push_str(" world");
        s2.push_str(" world");
        assert_eq!(s1, "hello world");
        assert_eq!(s2, "hello world");
    }    
}