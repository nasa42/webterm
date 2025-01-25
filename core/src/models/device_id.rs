#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceId {
    device_name: String,
    device_subname: String,
}

impl DeviceId {
    pub fn new(device_name: String, device_subname: String) -> Self {
        Self {
            device_name,
            device_subname,
        }
    }

    pub fn name(&self) -> &str {
        &self.device_name
    }

    pub fn subname(&self) -> &str {
        &self.device_subname
    }
}
