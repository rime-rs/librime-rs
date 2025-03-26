use fst::automaton::Str;

pub enum ValueType {
    Null,
    Scalar,
    List,
    Map,
}

struct ConfigItem {
    r#type: ValueType,
}

impl Default for ConfigItem {
    fn default() -> Self {
        Self {
            r#type: ValueType::Null,
        }
    }
}

trait Config {
    fn r#type() -> ValueType;
    fn empty(&self) -> bool;
}

pub struct ConfigValue {
    item: ConfigItem,
    value: String,
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self {
            item: ConfigItem {
                r#type: ValueType::Scalar,
            },
            value: String::new(),
        }
    }
}
impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        todo!()
    }
}
impl From<i32> for ConfigValue {
    fn from(value: i32) -> Self {
        todo!()
    }
}
impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        todo!()
    }
}
impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        todo!()
    }
}
impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl ConfigValue {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    // schalar value accessors
    pub(crate) fn get_bool(value: &bool) -> bool {
        todo!()
    }
    pub fn get_int(value: &i32) -> bool {
        todo!()
    }
    pub(crate) fn get_double(value: &f64) -> bool {
        todo!()
    }
    pub fn get_string(value: &str) -> bool {
        todo!()
    }
    // pub(crate) fn set_bool();
    // pub(crate) fn set_int();
    // pub(crate) fn set_double();
    // pub(crate) fn set_string_from_str();
    // pub(crate) fn set_string();

    pub(crate) fn str_(&self) -> &str {
        &self.value
    }

    pub(crate) fn empty(&self) -> &bool {
        todo!();
    }
}
