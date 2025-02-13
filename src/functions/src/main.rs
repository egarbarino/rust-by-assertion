fn main() {
    
}

#[cfg(test)]
mod tests {
    //
    // ## Functions
    //
    fn empty_function() {
    }
    fn multiply(x : i32, y : i32) -> i32 {
        x * y
    }
    fn multiply_same_1(x : i32, y : i32) -> i32 {
        return x * y;
    }
    fn multiply_same_2(x : i32, y : i32) -> i32 {
        let result = {
            x * y
        };
        result
    }
    #[test]
    fn test_simple_functions() {
        assert_eq!(empty_function(),());
        assert_eq!(multiply(2,3), 6);
        assert_eq!(multiply_same_1(2,3), 6);
        assert_eq!(multiply_same_2(2,3), 6);        
    }
}
