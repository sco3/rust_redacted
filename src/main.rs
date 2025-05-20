use serde::{Serialize, Serializer};
use serde_json;

//#[derive(Serialize)]
struct Secret {
    s_value: String,
}

fn fmt(s: &str) -> String {
    return format!("{}<redacted>", s);
}

impl Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let redacted = match self.s_value.len() {
            0 => fmt(""),
            n if n <= 4 => fmt(""),
            _ => fmt(&self.s_value[..4]),
        };
        serializer.serialize_str(&redacted)
    }
}

fn main() {
    let s = Secret {
        s_value: "asd".to_string(),
    };
    println!("{}", serde_json::to_string(&s).unwrap());
    
    let s_long = Secret {
        s_value: "asdf-1234".to_string(),
    };
    println! ("{}",serde_json::to_string( &s_long).unwrap());
}
