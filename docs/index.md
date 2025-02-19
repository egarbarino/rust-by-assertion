---
title: Rust by Assertion
author: Ernesto Garbarino
date: 2025-02-18
---



# Simple Data Types

## Scalar Types

A scalar type represents a single value. 

### Booleans

Booleans use the `true` and `false` literals and can be negated
using `!`.


``` rust
#[test]
fn test_booleans() {
    assert_eq!(true, !false);
    assert_eq!(false, !true);
    let t : bool = true;
    assert_eq!(t, true);
}
```


### Integers

If integers are unsigned, their type is prefixed with `u`,
otherwise, they are prefixed with `i` (as in 'regular' integer).
After the `u` or `i` prefix, their size in bits is indicated,
ranging from 8 to 128.

The `usize` type represents the architecture-dependant integer type
that is valid for addressing purposes. In contemporary computers,
this is equivalent to `u64`.

Let's look first at unsigned integers...


``` rust
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
```


... and now at signed ones:


``` rust
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
```


Integer literals may be expressed as follows:


``` rust
#[test]
fn test_integer_literals() {
    assert_eq!(2500, 2_500);      /* Thousands separeted by _ */
    assert_eq!(255, 0xff);        /* Hexadecimal              */
    assert_eq!(255, 0b11111111);  /* Binary                   */
    assert_eq!(65, b'A');         /* Character code           */
}
```


Regular operators are implemented as in most languages,
except for power which requires the `pow()` method.


``` rust
#[test]
fn test_integer_operators(){
    assert_eq!(3 + 2, 5);
    assert_eq!(3 - 2, 1);
    assert_eq!(3 * 2, 6);
    assert_eq!(6 / 3, 2);
    assert_eq!(5 % 3, 2);             /* modulo */
    assert_eq!((2 as u32).pow(3), 8); /* power  */
}
```


Unlike other languages, Rust treats the exceeding of
an integer's size as an overflow. If a wrapping behaviour
is desired, the `wrapping_add` method could be used, instead.


``` rust
#[test]
fn test_integer_wrapping() {
    let mut a : u8 = 255;
    /* a += 1; panics */
    a = a.wrapping_add(1);
    assert_eq!(a, 0);
}
```


### Floats

Rust implements floats using the IEEE 754 standard.


``` rust
#[test]
fn test_floats() {
    assert_eq!(f32::MIN, -3.4028235e38);
    assert_eq!(f32::MAX, 3.4028235e38);
    assert_eq!(f64::MIN, -1.7976931348623157e308);
    assert_eq!(f64::MAX, 1.7976931348623157e308);   
}
```


Operators behave like in most conventional languages,
except for power which requires the `powf()` method.


``` rust
#[test]
fn test_float_operators(){
    assert_eq!(0.7 + 0.3, 1.0);
    assert_eq!(1.5 - 0.5, 1.0);
    assert_eq!(2.5 * 3.0, 7.5);
    assert_eq!(6.0 / 2.5, 2.4);
    assert_eq!((1.5 as f32).powf(2.0), 2.25);
}
```


### Characters

Characters in Rust aren't bytes. They use a 32-bit
integer-like type which can encode unicode characters.


``` rust
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

```


## Tuples

Tuples have a fixed length. Each component may be of a different type.
Tuples are both constructed and deconstructed in Rust 
using the `(c1, c2, ...)` 
notation like in many other languages.

Components may be extracted individually using the
`tuple.index` notation, starting from zero.


``` rust
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
```


## Arrays

Arrays have a fixed length and their elements
have the same type as in most statically-typed
languages. The length is checked using the `len`
method, while elements are obtained using the
standard `array[element_index]` notation.


``` rust
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
```


## Slices

A slice in Rust, similarly to Go, acts as a 'view' upon an array. Such
a view consists of start and end indices. A slice, as the name suggests,
allow manipulating subsets of an array without the need of having to
allocate a new one. 

Assuming we have an array in hand, obtaining a slice involves addressing
it by reference and specifying the desired 
range: `&array[start_index..end_index+1]`.


### Array slices


``` rust
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
```


Note that the input is a fixed-size array but the output
is a size-independent slice.


``` rust
fn return_array_slice(a : &[u8 ; 5]) -> &[u8] {
    &a[0..3]
}
```


### String slices

There isn't nothing special about string slices, except
that the string slice type is `&str` rather than `&String`


``` rust
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
```


Note that the slice for a string is `&str` rather than `&String`


``` rust
fn return_string_slice(s : &String) -> &str {
    &s[0..5]
}

```


# Variables

## Constants

Constants are inlined whenever they are evaluated rather than
being stored in a single memory location. 


``` rust
const MILLENNIUM_BUG : u16 = 2000;

#[test]
fn test_constants() {
    assert_eq!(MILLENNIUM_BUG, 2000);
}
```


## Immutable and Mutable Variables

Variables are declared using the `let` keyword and are 
immutable by default unless the `mut` modifier is included.


``` rust
#[test]
fn test_variables() {
    let _a : u8 = 9;       /* Immutable by default */
    /* _a += 1; won't compile, variable is immutable */
    let mut b : u8 = 9;    /* Made mutable by adding 'mut' */
    b += 1;
    assert_eq!(b, 10)
}

```


## Static variables

Static variables are stored in a fixed memory location and
referenced accordingly whenever evaluated.

Let's first declare two static variables, one mutable and the other one immutable...


``` rust
static BEST_COMMODORE_COMPUTER : u8 = 64;

/* Unsafe; avoid this if possible */
static mut INITIAL_TEMPERATURE : i8 = -5;
```


First, we see that they are in scope from within a test function.


``` rust
#[test]
fn test_static_variables() {
    assert_eq!(BEST_COMMODORE_COMPUTER, 64);
   
    /* Don't do this! Use Mutex or similar to be safe */
    unsafe {
        INITIAL_TEMPERATURE += 7;
        assert_eq!(INITIAL_TEMPERATURE, 2);
    }
}
```


We can also prove that `BEST_COMMODORE_COMPUTER` is stored in the same memory location.


``` rust
#[test]
fn test_static_variables_reference() {
    let pointer_1 = std::ptr::addr_of!(BEST_COMMODORE_COMPUTER);
    let pointer_2 = std::ptr::addr_of!(BEST_COMMODORE_COMPUTER);
    assert_eq!(pointer_1,pointer_2)
}
```


## Variable Scope

As in most C-like languages, curly braces introduce a new scope.


``` rust
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
```


Unlike, say, pure functional languages, variable names may be
reused within the same scope. Please note that it is the variable
name that is being reused, rather than the type it can accommodate.


``` rust
#[test]
fn test_variable_scope_2() {
    let asterisks = ['*';5];         /* Here asterisks is an array */
    let asterisks = asterisks.len(); /* Here asterisks is an integer */
    assert_eq!(asterisks,5);
}
```


# Structs


## Struct Definition

Structs consists of one or more components using the `attribute : type` notation. 
The last component may or may not include a comma.


``` rust
struct Spaceship {
    shield: bool,
    name: String,
    fuel: u8, /* Comma is optional */
}
```


## Regular Struct Construction and Query

Structs are constructed using the same `attribute : value` notation.
The last component may or may not include a comma. Components may
be queried using the `struct_value.component_name` notation.


``` rust

#[test]
fn test_struct() {
    let spaceship = Spaceship {
        shield: true,
        name: String::from("Rocinante"),
        fuel : 12, /* Comma is optional */
    };
    assert_eq!(spaceship.shield, true);
    assert_eq!(spaceship.name, "Rocinante");
    assert_eq!(spaceship.fuel, 12);
}
```


## Name-matching Struct Construction

If the variable names match the struct's component
names, there's no need to specify each component.


``` rust
#[test]
fn test_struct_name() {
    let shield = true;
    let name = String::from("Rocinante");
    let fuel : u8 = 12;
    let spaceship = Spaceship {
        shield, /* has to match struct's attribute name exactly */
        name,   /* has to match struct's attribute name exactly */
        fuel    /* has to match struct's attribute name exactly */
    };
    assert_eq!(spaceship.shield, true);
    assert_eq!(spaceship.name, "Rocinante");
    assert_eq!(spaceship.fuel, 12);
}
```


## Updating Mutable Struct

In this case, each attribute must be updated separately


``` rust
#[test]
fn test_struct_update_mutable() {
    let mut spaceship = Spaceship {
        shield: true,
        name: String::from("Rocinante"),
        fuel : 12
    };
    assert_eq!(spaceship.name, "Rocinante");
    spaceship.name =  String::from("The Anubis"); /* Update here! */
    assert_eq!(spaceship.name, "The Anubis");
}    
```


## Updating Immutable Struct

This essentially involves creating a new struct value
based on the value from an existing one, specifying it as
the last component using the `..base_struct` notation.


``` rust
#[test]
fn test_struct_update_immutable() {
    let spaceship1 = Spaceship {
        shield: true,
        name: String::from("Rocinante"),
        fuel : 12
    };
    let spaceship2 = Spaceship {
        name: String::from("The Anubis"),
        ..spaceship1 /* Note the reference to the previous struct here */
    };

    assert_eq!(spaceship1.shield, true);
    assert_eq!(spaceship1.name, "Rocinante");
    assert_eq!(spaceship1.fuel, 12);

    assert_eq!(spaceship2.shield, true);
    assert_eq!(spaceship2.name, "The Anubis");
    assert_eq!(spaceship2.fuel, 12);
}
```


## Composite Structs

A struct may be made up of other structs


``` rust
struct SpacePort {
    name : String,
    docked_spaceship: Spaceship,
}
```

We can refer to the nested components using the dot notation.

``` rust
#[test]
fn test_struct_composite_struct() {   
    let space_port = SpacePort {
        name : String::from("Lovell City"),
        docked_spaceship : Spaceship {
            shield: true,
            name: String::from("Rocinante"),
            fuel : 12
        }
    };
    assert_eq!(space_port.name, "Lovell City");
    assert_eq!(space_port.docked_spaceship.name, "Rocinante");
} 
```


## Tuple Struct 

In a tuple struct, components are positional rather than
having a name. 


``` rust
#[derive(PartialEq)]
#[derive(Debug)]
struct RGB(u8,u8,u8); /* Declaration */
#[test]
fn test_tuple_struct() {
    let purple = RGB(255,0,255); /* Construction */
    assert_eq!(purple.0, 255);   /* Component 0 Query */
    assert_eq!(purple.1, 0);     /* Component 1 Query */
    assert_eq!(purple.2, 255);   /* Component 2 Query */
    assert_eq!(purple, RGB(255,0,255)); /* via PartialEq and Debug */
}
```


## Unit-Like Struct

This struct has no components.


``` rust
#[derive(PartialEq)]
#[derive(Debug)]
struct NoComponents;
#[test]
fn test_unit_struct() {
    let no_components = NoComponents;
    assert_eq!(no_components, NoComponents);
}
```


# Control Flow

Similarly to Go, Rust does not require boolean expressions in control flow statements
to be flanked by parentheses. 

## If Statements

If statements are implemented using `if boolean_expression {...}`. 


``` rust
#[test]
fn test_if() {
    let mut r = 'a';
    if 3 > 2 {
        r = 'b';
    }
    assert_eq!(r,'b');
}
```


If-Else statements are implemented using `if boolean_expression {...} else {...}`. 


``` rust
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
```


If-The-Else statements are implemented using `if boolean_expression {...} else if boolean_expression {...} else {...}`. 


``` rust
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

```


## If Expressions

It is idiomatic in Rust to use `if/else` to formulate expressions rather than
for imperative control flow purposes. 


``` rust
#[test]
fn test_if_expressions() {
    assert_eq!(if 3 > 2 { true } else { false }, true);
}

```


## Infinite Loop Statements

Infinite loops may be implemented using `loop {...}` and exited using `break;`.


``` rust
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
```


## Infinite Loop Expressions

Infinite loops may also be used to formulate expressions using the
`let variable = loop { break return_value };` syntax


``` rust
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
```


Nested loops may be exited using labels. Labels are prefixed with an apostrophe as follows: `'label`:


``` rust
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

```


## While Loop

While loops are implemented using the `while boolean_expression {...}` syntax.


``` rust
#[test]
fn test_while() {
    let mut x = 0;
    while x < 3 {
        x += 1;
    }
    assert_eq!(x, 3);
}
```


## For Loop

The default, simplest _for loop_ iterates over a sequence of elements.
In concrete, it can iterate over iterable types, such as ranges, collections (e.g., arrays, vectors, slices), iterators, etc. 
It is implemented using the `for element in sequence {...}` syntax.

In this example, it iterates over an array:


``` rust
#[test]
fn test_for() {
    let numbers = [1,2,3];
    let mut r = 0;
    for value in numbers {
        r = r + value;
    }
    assert_eq!(r,6);
}
```


## For Loop (Range)

For loops can also be used to iterate over a custom _range_ using the `for value in start_number..end_number+1 {}` syntax.


``` rust
#[test]
fn test_for_just_index() {
    let numbers = [1,2,3];
    let mut r = 0;
    for index in 0..numbers.len() {
        r = r + &numbers[index];
    }
    assert_eq!(r,6);
}
```


In most cases, it is more practical to declare the range as in `start_number..=end_number` (note the `=` sign)


``` rust
#[test]
fn test_for_range() {
    let mut r = 0;
    for value in 1..=3 {
        r = r + value;
    }
    assert_eq!(r,6);
}
```


It is also possible to define a custom step value via the `step_by(number)` method.


``` rust
#[test]
fn test_for_range_step() {
    let mut r = 0;
    for value in (1..=8).step_by(2) {
        r = r + value;
    }
    assert_eq!(r,16);
}
```


## For Loop over Index

Similarly to Python, it is possible to iterate both through the sequence's values and 
each element's ordinal index, at the same time.  


``` rust
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

```

