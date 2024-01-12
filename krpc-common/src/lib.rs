#[derive(Debug)]
pub struct KrpcMsg {
    pub unique_identifier: String,
    pub version: String,
    pub class_name: String,
    pub method_name: String,
    pub data: String,
}

impl KrpcMsg {
    pub fn new_empty() -> KrpcMsg {
        return KrpcMsg {
            unique_identifier: "".to_string(),
            version: "".to_string(),
            class_name: "".to_string(),
            method_name: "".to_string(),
            data: "".to_string(),
        };
    }

    pub fn new(
        unique_identifier: String,
        version: String,
        class_name: String,
        method_name: String,
        data: String,
    ) -> KrpcMsg {
        return KrpcMsg {
            unique_identifier,
            version,
            class_name,
            method_name,
            data
        };
    }
}