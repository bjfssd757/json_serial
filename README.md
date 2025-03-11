# json_serial
This library will help you work with Json format in Rust

## Documentation

> [!WARNING]
> This library is in early development stage. Be careful when using it and stay turned for updates!

[Serialize into json](#Serialize)\
[Deserialize from json](#Deserialize)

### Serialize

> [!WARNING]
> All structures require ```#[derive(Debug)]```

To serialize a structure (struct) into json format (you will get a String as output) you need to use the ```impl_to_json!()``` macro for your structure, passing it the structure and all the elements of the structure:
```Rust
impl_to_json!(Struct_name, value1, value2, value3 ...);
```
After that you need to specify the values for this structure. For example:
```Rust
let address = Address {
    city: "City".to_string(),
    street: "Street".to_string(),
    number: 1,
}; // as example for values in struct
let person = Person {
    name: "Bob".to_string(),
    age: 30,
    address: "address".to_string(),
}; // as example for values in struct

// when struct:

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
``` 
Then you can apply the ```.to_json()``` function to the structure with the values.

### Deserialize

> [!WARNING]
> All structures require ```#[derive(Debug)]```

To deserialize data from json format to a structure (struct) you need to apply the macro ```impl_from_json!()``` to the structure that will repeat the json body. For example:\
Json:
```Json
{
	"name": "Bob",
	"age": 30,
	"address": "some address"
}
```
Rust:
```Rust
#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
	address: String,
}

impl_from_json!(Person, name: String, age: u8, address: String);

let json = ... // there should be json in &str format here
let without_json = Person::from_json(&json);

// USING

let name = without_json.name; // String
let age = without_json.age; // u8
let address = without_json.address; // String
```
> [!WARNING]
> value must be passed along with the data type!


Full example with using two macro:
```Rust
#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
	address: String,
}

impl_to_json!(Person, name, age, address);

let person = Person {
	name: "Bob".to_string(),
	age: 30,
	address: "address".to_string(),
}

let json = person.to_json();

impl_from_json!(Person, name: String, age: u8, address: String);

let without_json = Person::from_json(&json);
```