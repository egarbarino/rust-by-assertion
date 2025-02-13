fn main() {
  
}
//
// ## Control Flow
//
#[cfg(test)]
mod tests {
    //
    // ### If-Then-Else
    //
    #[test]
    fn test_if() {
        /* Simple If */
        let mut r = '-';
        if 3 > 2 {
            r = 'a';
        }
        assert_eq!(r,'a');

        /* If Else */
        let mut r = '-';
        if 2 > 3 {
            r = 'a';
        } else {
            r = 'b';
        }
        assert_eq!(r,'b');

        /* If-Then-Else */
         let mut r = '-';
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
    // ### If Expressions
    //
    #[test]
    fn test_if_expressions() {
        assert_eq!(if 3 > 2 { true } else { false }, true);
    }

    //
    // ### Loop
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
    }
    #[test]
    fn test_loop_with_labels() {
        let mut results = [0;3];
        let mut index = 0;
        'top_loop : loop {
            'ineer_loop : loop {
                results[index] += (2 + index) * 2;
                if results[index] % 4 == 0 {
                    break 'ineer_loop; /* implied */
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
    // ### While
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
    // ### For
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
    #[test]
    fn test_for_just_index() {
        let numbers = [1,2,3];
        let mut r = 0;
        for index in 0..numbers.len() {
            r = r + &numbers[index];
        }
        assert_eq!(r,6);
    }
    #[test]
    fn test_for_range() {
        let mut r = 0;
        for value in 1..=3 {
            r = r + value;
        }
        assert_eq!(r,6);
    }
    #[test]
    fn test_for_range_step() {
        let mut r = 0;
        for value in (1..=8).step_by(2) {
            r = r + value;
        }
        assert_eq!(r,16);
    }
}
