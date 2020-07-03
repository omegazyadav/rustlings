// structs1.rs
// Address all the TODOs to make the tests pass!

// I AM DONE

struct ColorClassicStruct {
    color_name: String,
    color_type: String,
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */ String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let c_name = String::from("green");
        let c_type = String::from("#00FF00");
        let green = ColorClassicStruct{
            color_name: c_name,
            color_type: c_type,
        };

        assert_eq!(green.color_name, "green");
        assert_eq!(green.color_type, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let color_name = String::from("green");
        let color_type = String::from("#00FF00");
        let green = ColorTupleStruct(color_name,color_type);

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
