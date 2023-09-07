use crate::PersonId;

use std::fmt;

pub struct Motion {
    pub title: &'static str,
    pub description: &'static str,
    /// 0 contributors - anonymous motions are possible
    pub developers: Vec<PersonId>,
    /// the group of people who may be affected by the motion, and who can
    /// therefore vote on it
    pub electors: Vec<PersonId>
}

impl Motion {
    pub fn dev_count(&self) -> usize {
        self.developers.len()

    }
    pub fn elector_count(&self) -> usize {
        self.electors.len()
    }
}

impl fmt::Display for Motion {
    // doesn't display developers or electorate
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.title)?;
        f.write_str("\n\n")?;
        f.write_str(self.description)
    }
}
