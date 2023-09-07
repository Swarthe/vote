use std::fmt;

use std::{
    ops::Index,
    fmt::Display,
    iter::FromIterator
};

/// test to make sure that we can fit and index the entire population
///
/// this negates the need to validate conversion tests between `usize` and
/// `PersonId`
const _POPULATION_FITS_USIZE: () = assert!(usize::BITS >= u64::BITS);

/// data pertaining to a single individual, not necessarily unique
pub struct Person {
    pub name: String
}

/// a population, with unique individuals discriminated by an ID
/// (equivalent to the index of the person in the list)
///
/// PersonList and PersonId are opaque to ensure validity
// realistically this info would be stored in a DB
pub struct PersonList(Vec<Person>);

// u64 instead of usize because a person's ID shouldn't depend on computer
// architecture. same with population size
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PersonId(u64);

impl PersonList {
    pub fn len(&self) -> u64 {
        self.0.len() as _
    }

    /// ID of random person in list
    pub fn rand_choice(&self) -> PersonId {
        use rand::Rng;

        let idx = rand::thread_rng().gen_range(0..self.0.len());

        PersonId::from_usize(idx)
    }

    /// `n` unique IDs of people in list
    ///
    /// panics if n > the number of people in the list
    pub fn rand_choices(&self, n: u64) -> Vec<PersonId> {
        use rand::seq::index;

        index::sample(
            &mut rand::thread_rng(),
            self.0.len(),
            n as usize
        ).iter().map(PersonId::from_usize).collect()
    }

    pub fn ids(&self) -> impl Iterator<Item = PersonId> {
        (0..self.0.len())
            .map(PersonId::from_usize)
    }
}

impl Index<PersonId> for PersonList {
    type Output = Person;

    fn index(&self, idx: PersonId) -> &Person {
        // `PersonId` is a valid `usize` index into `PersonList`.
        &self.0[idx.0 as usize]
    }
}

impl Display for PersonList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut person_iter = self.0.iter();

        if let Some(p) = person_iter.next() {
            write!(f, "{}", p.name)?;
        }

        for p in person_iter {
            write!(f, "\n{}", p.name)?;
        }

        Ok(())
    }
}

//impl<'a> IntoIterator for &'a PersonList {
//    type Item = &'a Person;
//    type IntoIter = std::slice::Iter<'a, Person>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        self.0.iter()
//    }
//}

impl FromIterator<Person> for PersonList {
    fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = Person>
    {
        Self(iter.into_iter().collect())
    }
}

impl PersonId {
    /// should only be used when `n` is a valid index into a `PersonList`, or
    /// the result might be an invalid ID
    fn from_usize(n: usize) -> Self {
        PersonId(n as _)
    }
}
