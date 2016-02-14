use rustc_serialize::json;

pub trait Output {
    fn start(&self) -> String {
        "".to_string()
    }

    fn chunk(&self, outs: Vec<String>) -> String;
}

pub struct DzenOutput;

impl Output for DzenOutput {
    fn chunk(&self, outs: Vec<String>) -> String {
        outs.join("|")
    }
}

#[derive(RustcEncodable)]
struct I3barHeader {
    pub click_events: bool,
    pub version: i32
}

#[derive(RustcEncodable)]
struct I3barBlock {
    pub full_text: String
}

pub struct I3barOutput;

impl Output for I3barOutput {
    fn start(&self) -> String {
        let header = I3barHeader {
            click_events: false,
            version: 1
        };

        format!("{}\n[\n", json::encode(&header).unwrap())
    }

    fn chunk(&self, outs: Vec<String>) -> String {
        let blocks = outs.iter().map(|o| I3barBlock { full_text: o.to_owned() }).collect::<Vec<_>>();
        format!("{},", json::encode(&blocks).unwrap())
    }
}

