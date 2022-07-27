use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer, Deserialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

struct Points {
    frames: BTreeMap<String, Vec<f32>>,
}

// Serializes as a map in which the values are all unit.
impl Serialize for Points {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.frames.iter().map(|(key, value)| (key, value)))
    }
}

struct Data {
    id: String,
    points: Points,
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut data = serializer.serialize_struct("Data", 3)?;
        data.serialize_field("id", &self.id)?;
        data.serialize_field("points", &self.points)?;
        data.end()
    }
}

fn main() {
    let mut frames = BTreeMap::new();
    for frame in 1..=3_usize {
        let frame = frame.to_string();
        let frame = format!("{:0>4}", frame);
        let data = vec![1.2,3.2_f32];
        frames.insert(frame, data);
    }
    let points = Points { frames };

    let mut data = Data{id:"3".to_string(), points};


    let out = serde_json::to_string(&data).unwrap();
    println!("{}", out);
    let mut file = File::create("./test.json").expect("msg");
    file.write_all(out.as_bytes()).expect("msg");
}
