use std::string::ToString;
use std::cmp::Eq;

pub struct Assignment {
    value : ;

}

// Implementing ToString trait
impl ToString for Assignment {
    fn to_string(&self) -> String {
        format!("{:?}",to_string(value))
    }
}

impl PartialEq for Assignment {
    fn eq(&self, other: &Assignment) -> bool {
        self.value == other.value
    }
}
