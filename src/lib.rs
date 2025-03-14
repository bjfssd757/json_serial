// Tests start in 58 line

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
                    json.push_str(&format!("\"{}\": {}, ", stringify!($field_name), self.$field_name.to_json()));
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
                    let key = kv.next().expect("Fail on next() in key. impl_from_json").trim().trim_matches('"').to_string();
                    let value = kv.next().expect("Fail on next() in value. impl_from_json").trim().trim_matches('"').parse().unwrap();
                    map.insert(key, value);
                }

                $struct_name {
                    $(
                        $field_name: <$field_type>::from_json(map.get(stringify!($field_name)).expect("Fail in macro on parse!").as_str()),
                    )*
                }
            }
        }
    };
}

impl ToJson for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToJson for Vec<String> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<i8> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<i16> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<i32> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<i64> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<i128> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<u8> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<u16> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<u32> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<u64> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<u128> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<f32> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<f64> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<isize> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for Vec<usize> {
    fn to_json(&self) -> String {
        let json_value: Vec<String> = self.iter().map(|v| v.to_json()).collect();
        format!("[{}]", json_value.join(", "))
    }
}

impl ToJson for i8 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for i16 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for i32 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for i64 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for i128 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for u8 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for u16 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for u32 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for u64 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for u128 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for f32 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for f64 {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for isize {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for usize {
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}



impl FromJson for String {
    fn from_json(json: &str) -> Self { json.to_string() }
}

impl FromJson for Vec<i8> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().expect("fail on parse in Vec<i8>"))
            .collect()
    }
}

impl FromJson for Vec<i16> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<i32> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<i64> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<i128> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<u8> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<u16> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<u32> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<u64> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<u128> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<isize> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<usize> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<f32> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<f64> {
    fn from_json(json: &str) -> Self {
        let json = json.trim_matches(|c| c == '{' || c == '}');
        json.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl FromJson for Vec<String> {
    fn from_json(s: &str) -> Self {
        s.trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|v| v.trim().to_string())
            .collect()
    }
}

impl FromJson for i8 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for i16 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for i32 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for i64 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for i128 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for u8 {
    fn from_json(s: &str) -> Self {
        match s.parse() {
            Ok(v) => return v,
            Err(e) => panic!("{:?}", e),
        }
    }
}

impl FromJson for u16 {
    fn from_json(s: &str) -> Self {
        s.parse().expect("Fail on parse u16!")
    }
}

impl FromJson for u32 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for u64 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for u128 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for isize {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for usize {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for f32 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl FromJson for f64 {
    fn from_json(s: &str) -> Self {
        s.parse().unwrap()
    }
}



#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    homes: Vec<i8>,
} // as example for struct

#[derive(Debug)]
struct Address {
    city: String,
    street: String,
    number: u16,
} // as example for struct

impl_to_json!(Address, city, street, number);
impl_to_json!(Person, name, age, address, homes);
impl_from_json!(Address, city: String, street: String, number: u16);
impl_from_json!(Person, name: String, age: u8, address: Address, homes: Vec<i8>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_macro() {
        unsafe {env::set_var("RUST_BACKTRACE", "full");}

        let _address = Address {
            city: "Moscow".to_string(),
            street: "Gorky Park".to_string(),
            number: 42,
        }; // as example for values in struct
        let person = Person {
            name: "Bob".to_string(),
            age: 30,
            address: Address {
                city: "London".to_string(),
                street: "Street".to_string(),
                number: 32,
            },
            homes: vec![1, 2, 3],
        }; // as example for values in struct

        assert_eq!(person.to_json(), "{\"name\": \"Bob\", \"age\": 30, \"address\": {\"city\": \"London\", \"street\": \"Street\", \"number\": 32}, \"homes\": [1, 2, 3]}");
        //assert_eq!(address.to_json(), "{\"city\": \"Moscow\", \"street\": \"Gorky Park\", \"number\": 42}");
    }
    
    #[test]
    fn test_from_json() {
        unsafe {env::set_var("RUST_BACKTRACE", "1");}

        let address = Address {
            city: "Moscow".to_string(),
            street: "Gorky Park".to_string(),
            number: 42,
        };
        let person = Person {
            name: "Bob".to_string(),
            age: 30,
            address: Address {
                    city: "Some".to_string(),
                    street: "Some Street".to_string(),
                    number: 54
                },
            homes: vec![2, 3, 4],
        };

        let json_person = person.to_json();
        let json_address = address.to_json();

        let struct_person = Person::from_json(&json_person);
        let struct_address = Address::from_json(&json_address);

        assert_eq!(struct_person.name, "Bob");
        assert_eq!(struct_address.city, "Some");
        assert_eq!(struct_person.age, 30);
        assert_eq!(struct_person.address.number, 54);
        assert_eq!(struct_person.homes, vec![2, 3, 4]);
    }
}
