pub trait ToJson {
    fn to_json(&self) -> String;
}

pub trait FromJson: Sized {
    fn from_json(json: &str) -> Self;
}

#[macro_export]
macro_rules! impl_to_json {
    ($struct_name:ident, $( $field_name:ident ),* ) => {
        impl ToJson for $struct_name {
            fn to_json(&self) -> String {
                let mut json = String::new();
                json.push('{');
                $(
                    json.push_str(&format!("{:?}: {:?}, ", stringify!($field_name), self.$field_name));
                )*
                json.pop();
                json.pop();
                json.push('}');
                json
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_json {
    ($struct_name:ident, $( $field_name:ident : $field_type:ty ),* ) => {
        impl FromJson for $struct_name {
            fn from_json(json: &str) -> Self {
                use std::collections::HashMap;

                let json = json.trim_matches(|c| c == '{' || c == '}');
                let mut map: HashMap<String, String> = HashMap::new();

                for pair in json.split(',') {
                    let mut kv = pair.split(':');
                    let key = kv.next().unwrap().trim().trim_matches('"').to_string();
                    let value = kv.next().unwrap().trim().trim_matches('"').parse().unwrap();
                    map.insert(key, value);
                }

                $struct_name {
                    $(
                        $field_name: map.get(stringify!($field_name))
                            .unwrap().parse::<$field_type>().unwrap(),
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
            address: String,
        }
        #[derive(Debug)]
        struct Address {
            city: String,
            street: String,
            number: u16,
        }

        impl_to_json!(Address, city, street, number);

        impl_to_json!(Person, name, age, address);

        let address = Address {
            city: "Moscow".to_string(),
            street: "Gorky Street".to_string(),
            number: 42,
        };
        let person = Person {
            name: "Bob".to_string(),
            age: 30,
            address: "address".to_string(),
        };

        assert_eq!(person.to_json(), "{\"name\": \"Bob\", \"age\": 30, \"address\": \"address\"}");
        assert_eq!(address.to_json(), "{\"city\": \"Moscow\", \"street\": \"Gorky Street\", \"number\": 42}");
    }

    #[test]
    fn test_from_json() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
            address: String,
        }
        #[derive(Debug)]
        struct Address {
            city: String,
            street: String,
            number: u16,
        }

        impl_from_json!(Address, city: String, street: String, number: u16);
        impl_to_json!(Address, city, street, number);

        impl_to_json!(Person, name, age, address);
        impl_from_json!(Person, name: String, age: u8, address: String);

        let address = Address {
            city: "Moscow".to_string(),
            street: "Gorky Street".to_string(),
            number: 42,
        };
        let person = Person {
            name: "Bob".to_string(),
            age: 30,
            address: "address".to_string(),
        };

        let json_person = person.to_json();
        let json_address = address.to_json();

        let struct_person = Person::from_json(&json_person);
        let struct_address = Address::from_json(&json_address);

        assert_eq!(struct_person.name, "Bob");
        assert_eq!(struct_address.city, "Moscow");
    }
}
