use serde_json::{Serializer, Deserializer};
use std::sync::{Arc, Mutex};

pub struct EndpointFace {
    pub uri: &'static str,
    pub requirements: &'static [&'static str],
    pub handler: fn(data: serde_json::Value) -> Result<(), ()>,
}


impl EndpointFace {
    /// Checks for required fields in json data
    ///
    /// self
    /// The parsed json data
    pub fn check(self, data: serde_json::Value) -> Result<(), ()> {
        for i in self.requirements {
            if data[i] == serde_json::Value::Null {
                return Err(());
            }
        }

        if (self.handler)(data).is_err() {
            return Err(());
        }

        Ok(())
    }
}
