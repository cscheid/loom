use serde_json::{Map, Value};

//////////////////////////////////////////////////////////////////////////////

pub trait Serializable {
    fn to_json(&self) -> Value;
}

pub fn tagged_object(class: String, object: Map<String, Value>) -> Value
{
    let mut m = Map::new();
    m.insert("class".to_string(), class.into());
    m.insert("value".to_string(), Value::Object(object));
    Value::Object(m)
}

