use rill_protocol::io::provider::StreamType;

pub fn provider_type() -> StreamType {
    "system::provider".into()
}

pub mod proclist;
