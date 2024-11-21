use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
 struct Event {
    name: String,
    #[serde (serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String
}
fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
{
    serializer.serialize_str(&format!("This is date: {}", date))
}
fn deserialize_date<'d, D: Deserializer<'d>>(deserializer: D) -> Result<String, D::Error>
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("date", "event date"))
}
fn main() {
    let event = Event {
        name: String::from("Circus"),
        date: String::from("2024-14-11")
    };
    let json = serde_json::to_string(&event).expect("error");
    dbg!(&json);
    let desserialized_json: Event = serde_json::from_str(&json).expect("error");
    dbg!(desserialized_json);
}