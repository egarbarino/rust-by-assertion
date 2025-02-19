fn main() {
  
}

#[cfg(test)]
mod tests {
    //
    // # Control Flow
    //
    // Similarly to Go, Rust does not require boolean expressions in control flow statements
    // to be flanked by parentheses. 
    //
    // ## If Statements
    //
    // If statements are implemented using `if boolean_expression {...}`. 
    //
    #[test]
    fn test_if() {
        let mut r = 'a';
        if 3 > 2 {
            r = 'b';
        }
        assert_eq!(r,'b');
    }
    //
    // If-Else statements are implemented using `if boolean_expression {...} else {...}`. 
    //
    #[test]
    fn test_if_else() {
        let r;
        if 2 > 3 {
            r = 'a';
        } else {
            r = 'b';
        }
        assert_eq!(r,'b');
    }
    //
    // If-The-Else statements are implemented using `if boolean_expression {...} else if boolean_expression {...} else {...}`. 
    //
    #[test]
    fn test_if_then_else() {
        let r;
        if 2 > 3 {
            r = 'a';
         } else if 2 > 5 {
            r = 'b';
         } else if 2 > 1 {
            r = 'c';
         } else {
            r = 'x';
         }
         assert_eq!(r,'c');  
    }

    //
    // ## If Expressions
    //
    // It is idiomatic in Rust to use `if/else` to formulate expressions rather than
    // for imperative control flow purposes. 
    //
    #[test]
    fn test_if_expressions() {
        assert_eq!(if 3 > 2 { true } else { false }, true);
    }

    //
    // ## Infinite Loop Statements
    //
    // Infinite loops may be implemented using `loop {...}` and exited using `break;`.
    //
    #[test]
    fn test_loop_1() {
        let mut x = 1;
        loop {
            x = x * 2;
            if x >= 100 {
                break;
            }
        }
        assert_eq!(x, 128);
    }
    //
    // ## Infinite Loop Expressions
    //
    // Infinite loops may also be used to formulate expressions using the
    // `let variable = loop { break return_value };` syntax
    //
    #[test]
    fn test_return_value_from_loop() {
        let mut x = 1;
        let r = loop {
            x = x * 2;
            if x >= 100 {
                break x;
            }
        };
        assert_eq!(x, 128);
        assert_eq!(r, 128);
    }
    //
    // Nested loops may be exited using labels. Labels are prefixed with an apostrophe as follows: `'label`:
    //
    #[test]
    fn test_loop_with_labels() {
        let mut results = [0;3];
        let mut index = 0;
        'top_loop : loop {
            'inner_loop : loop {
                results[index] += (2 + index) * 2;
                if results[index] % 4 == 0 {
                    break 'inner_loop; /* implied */
                }
            }
            index += 1;
            if index >=3 {
                break 'top_loop; /* implied */
            }
        }
        assert_eq!(results[0], 4);
        assert_eq!(results[1], 12);
        assert_eq!(results[2], 8);
    }

    //
    // ## While Loop
    //
    // While loops are implemented using the `while boolean_expression {...}` syntax.
    //
    #[test]
    fn test_while() {
        let mut x = 0;
        while x < 3 {
            x += 1;
        }
        assert_eq!(x, 3);
    }
    //
    // ## For Loop
    //
    // The default, simplest _for loop_ iterates over a sequence of elements.
    // In concrete, it can iterate over iterable types, such as ranges, collections (e.g., arrays, vectors, slices), iterators, etc. 
    // It is implemented using the `for element in sequence {...}` syntax.
    //
    // In this example, it iterates over an array:
    //
    #[test]
    fn test_for() {
        let numbers = [1,2,3];
        let mut r = 0;
        for value in numbers {
            r = r + value;
        }
        assert_eq!(r,6);
    }
    //
    // ## For Loop (Range)
    //
    // For loops can also be used to iterate over a custom _range_ using the `for value in start_number..end_number+1 {}` syntax.
    //
    #[test]
    fn test_for_just_index() {
        let numbers = [1,2,3];
        let mut r = 0;
        for index in 0..numbers.len() {
            r = r + &numbers[index];
        }
        assert_eq!(r,6);
    }
    //
    // In most cases, it is more practical to declare the range as in `start_number..=end_number` (note the `=` sign)
    //
    #[test]
    fn test_for_range() {
        let mut r = 0;
        for value in 1..=3 {
            r = r + value;
        }
        assert_eq!(r,6);
    }
    //
    // It is also possible to define a custom step value via the `step_by(number)` method.
    //
    #[test]
    fn test_for_range_step() {
        let mut r = 0;
        for value in (1..=8).step_by(2) {
            r = r + value;
        }
        assert_eq!(r,16);
    }
    //
    // ## For Loop over Index
    //
    // Similarly to Python, it is possible to iterate both through the sequence's values and 
    // each element's ordinal index, at the same time.  
    //
    #[test]
    fn test_for_index_and_value() {
        let numbers = [1,2,3];
        let mut r = 0;
        for (index, value) in numbers.iter().enumerate() {
            r = r + value;
            assert_eq!(value, &numbers[index]);
        }
        assert_eq!(r,6);
    }

}
