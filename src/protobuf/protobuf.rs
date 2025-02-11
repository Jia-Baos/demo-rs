use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub age: u32,
}

pub fn protobuf_demo(){
     // 创建一个 Person 实例
     let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // 序列化
    let mut buffer = Vec::new();
    person.encode(&mut buffer).unwrap();
    println!("Serialized: {:?}", buffer);

    // 反序列化
    let deserialized_person = Person::decode(buffer.as_slice()).unwrap();
    println!("Deserialized: {} is {} years old", deserialized_person.name, deserialized_person.age);
}