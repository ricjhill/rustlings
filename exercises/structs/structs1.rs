// structs1.rs
// Address all the TODOs to make the tests pass!



struct ColorClassicStruct<'a> {
    name: &'a str,
    hex:&'a str,
    // TODO: Something goes here

}

struct ColorTupleStruct( String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let name ="green";
        let hex =  "#00FF00";
         let green =  ColorClassicStruct { name, hex};


        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let colour :String = "green".to_string();
        let hex: String =  "#00FF00".to_string();
        let green = ColorTupleStruct(colour, hex);
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
