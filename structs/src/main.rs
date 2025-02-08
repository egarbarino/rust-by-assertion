fn main() {
    println!("Hello, world!");
}
//
// ## Structs
//
#[cfg(test)]
mod tests {
    //
    // ### Struct Definition
    //
    struct Spaceship {
        shield: bool,
        name: String,
        fuel: u8,
    }
    //
    // ### Regular Struct Construction
    //
    #[test]
    fn test_struct() {
        let spaceship = Spaceship {
            shield: true,
            name: String::from("Rocinante"),
            fuel : 12
        };
        assert_eq!(spaceship.shield, true);
        assert_eq!(spaceship.name, "Rocinante");
        assert_eq!(spaceship.fuel, 12);
    }
    //
    // ### Name-matching Struct Construction
    //
    // If the variable names match the struct's component
    // names, there's no need to specify each component.
    //
    #[test]
    fn test_struct_name() {
        let shield = true;
        let name = String::from("Rocinante");
        let fuel : u8 = 12;
        let spaceship = Spaceship {
            shield,
            name,
            fuel
        };
        assert_eq!(spaceship.shield, true);
        assert_eq!(spaceship.name, "Rocinante");
        assert_eq!(spaceship.fuel, 12);
    }
    //
    // ### Struct Update Syntax
    //
    #[test]
    fn test_struct_update() {
        let spaceship1 = Spaceship {
            shield: true,
            name: String::from("Rocinante"),
            fuel : 12
        };
        let spaceship2 = Spaceship {
            name: String::from("The Anubis"),
            ..spaceship1
        };
        assert_eq!(spaceship1.shield, true);
        assert_eq!(spaceship1.name, "Rocinante");
        assert_eq!(spaceship1.fuel, 12);

        assert_eq!(spaceship2.shield, true);
        assert_eq!(spaceship2.name, "The Anubis");
        assert_eq!(spaceship2.fuel, 12);
    }
    //
    // ### Tupe Struct
    //
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct RGB(u8,u8,u8);
    #[test]
    fn test_tuple_struct() {
        let purple = RGB(255,0,255);
        assert_eq!(purple.0, 255);
        assert_eq!(purple.1, 0);
        assert_eq!(purple.2, 255);
        assert_eq!(purple, RGB(255,0,255)); /* via PartialEq and Debug */
    }
    //
    // ### Unit-Like Struct
    //
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct NoComponents;
    #[test]
    fn test_unit_struct() {
        let no_components = NoComponents;
        assert_eq!(no_components, NoComponents);
    }
}
