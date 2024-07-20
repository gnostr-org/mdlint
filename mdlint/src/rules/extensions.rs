use crate::ruleset::RuleResultDetails;

pub trait VecExt {
    fn to_option(self) -> Option<Vec<RuleResultDetails>>;
}

impl VecExt for Vec<RuleResultDetails> {
    fn to_option(self) -> Option<Vec<RuleResultDetails>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
