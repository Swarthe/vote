use crate::{Motion, PersonId};

use chrono::{Duration, Utc};

type DateTime = chrono::DateTime<chrono::Utc>;

/// an electoral procedure for passing motions
///
/// ## development
///
/// in the development of new motions, open source software development
/// practices are taken as inspiration. development of the motion can be made in
/// public before proposal, either by one individual or by a group of
/// developpers of equal influence. decisions like adding/removing
/// developpers and proposing/retiring the motion are done by majority vote
/// among the developpers.
///
/// developpers are expected to take census and polls of interested populations
/// to determine the development and precise parameters of the motion.
/// ultimately, the motion will be subjected to the general approval of the
/// population, so transparent and bilateral development essential to serving
/// its wishes.
///
/// the motion remains public starting from the initial public development
/// stage, although it is only subject to general vote in the last stage
///
/// in order to be proposed, an absolute majority of the developers must vote to
/// approve it in a specific state.
///
/// ## majority
///
/// a majority is obtained when a plurality of the electorate votes to approve
/// the motion, as opposed to rejecting it. ties are impossible as an even split
/// does not equate to a plurality - in that case, the motion is rejected.
///
/// in order to obviate the need for a quorum, a preliminary "petition
/// stage" is used where a motion is shown to a limited, random set of electors
/// (of which a majority must vote to approve the motion for it to proceed), the
/// size of which can be adjusted to control the number of motions approved for
/// general votes. thus, a minority cannot through abstention prevent a majority
/// from imposing its will (the no-show paradox and strategic voting in general
/// is de-incentivised)
///
/// this functions as a sort of filter, and divides the selection of motions
/// across the electorate for higher efficiency. the use of large and random
/// approval groups ensures that all motions have a fair chance of approval,
/// regardless of the prominence of the proponents.
///
/// ## secret ballot
///
/// every vote, approval, signature... can be done in total secrecy, without the
/// identity of the voter being known during or after the fact. this is to
/// ensure that all voters can express their will independently and without
/// external influence, interference, or intimidation.
pub struct Procedure<St: ProcedureStage> {
    motion: Motion,
    stage: St
}

// realistically, voters/approvers... would be stored in DB
// secure mechanisms will be used to ensure vote secrecy when taken
//
// TODO: const generic enums for procedure state when available

/// typestate for electoral procedure
///
/// sealed trait
pub trait ProcedureStage: sealed::Sealed {}

/// developpment until majority of developpers vote to propose
///
/// any developpers can at any time start proposal vote for particular instance
/// of a motion - only one may be active at a time, until it expires or majority
/// is reached
///
/// minimum requiered number of votes to propose is  the number of
/// developpers / 2 + 1
pub struct Prototype {
    /// all voters are developers, listed in the motion
    have_voted: Vec<PersonId>,
    proposal_votes: u64
}

/// development is frozen and public debate until certain date is reached, set
/// by developers and probably subject to minimums in most cases
///
/// developers can vote at any time to return motion to prototype state
///
/// parties for and against the motion engage in fair debate, such that the
/// electorate is educated before making a decision
pub struct Proposal {
    end_date: DateTime
}

/// shown to a limited set of random individuals from the electorate for
/// approval or denial. voters decide whether the motion is worthy of
/// consideration or not, and are encouraged like with the general election to
/// vote sincerely in accordance with the accepted principles
///
/// if absolute majority of electorate approves, motion is selected for vote
pub struct Petition {
    voter_ids: Vec<PersonId>,
    have_voted: Vec<PersonId>,
    approval_votes: u64
}

/// motion is carried when there are more votes for than votes against
pub struct Referendum {
    have_voted: Vec<PersonId>,
    /// votes for adoption.
    votes_for: u64,
    /// votes against adoption.
    votes_against: u64,
}

impl ProcedureStage for Prototype {}
impl ProcedureStage for Proposal {}
impl ProcedureStage for Petition {}
impl ProcedureStage for Referendum {}

impl<St: ProcedureStage> Procedure<St> {
    pub fn motion(&self) -> &Motion {
        &self.motion
    }
}

impl Procedure<Prototype> {
    pub fn begin(motion: Motion) -> Self {
        Self { motion, stage: Prototype {
            have_voted: Vec::new(),
            proposal_votes: 0
        }}
    }

    pub fn proposal_votes(&self) -> u64 {
        self.stage.proposal_votes
    }

    /// error and does nothing if `person_id` has already voted or is not
    /// developper
    pub fn register_proposal_vote(&mut self, person_id: PersonId) -> Result<(), ()> {
        let is_valid = self.motion.developers.contains(&person_id)
            && !self.stage.have_voted.contains(&person_id);

        if is_valid {
            self.stage.proposal_votes += 1;
            self.stage.have_voted.push(person_id);

            Ok(())
        } else {
            Err(())
        }
    }

    /// returns Err(self) unchanged if not enough votes
    pub fn into_proposal(self, prop_time: Duration) -> Result<Procedure<Proposal>, Self> {
        let half = self.motion.developers.len() as u64 / 2;

        if self.stage.proposal_votes > half {
            Ok(Procedure {
                motion: self.motion,
                stage: Proposal { end_date: Utc::now() + prop_time }
            })
        } else {
            Err(self)
        }
    }
}

impl Procedure<Proposal> {
    pub fn end_date(&self) -> DateTime {
        self.stage.end_date
    }

    /// returns Err if proposal end date has not been reached
    pub fn into_petition(self) -> Result<Procedure<Petition>, Self> {
        use rand::seq::SliceRandom;

        if self.stage.end_date <= Utc::now() {
            let petitioner_count = self.motion.electors.len() as f32 * PETITIONER_RATIO;

            let voter_ids = self.motion.electors.choose_multiple(
                &mut rand::thread_rng(),
                petitioner_count as usize
            ).copied().collect::<Vec<_>>();

            Ok(Procedure {
                motion: self.motion,
                stage: Petition {
                    voter_ids,
                    have_voted: Vec::new(),
                    approval_votes: 0
                }
            })
        } else {
            Err(self)
        }
    }
}

/// the size of the petitioner group relative to population
///
/// in reality this would be a dynamic value, inversely proportional to the size
/// of the population
pub const PETITIONER_RATIO: f32 = 0.25;

impl Procedure<Petition> {
    pub fn votes_for(&self) -> u64 {
        self.stage.approval_votes
    }

    pub fn voter_ids(&self) -> &[PersonId] {
        &self.stage.voter_ids
    }

    pub fn register_approval_vote(&mut self, person_id: PersonId) -> Result<(), ()> {
        let is_valid = self.motion.electors.contains(&person_id)
            && !self.stage.have_voted.contains(&person_id);

        if is_valid {
            self.stage.approval_votes += 1;
            self.stage.have_voted.push(person_id);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn into_referendum(self) -> Result<Procedure<Referendum>, Self> {
        let half = self.stage.voter_ids.len() as u64 / 2;

        if self.stage.approval_votes > half {
            Ok(Procedure {
                motion: self.motion,
                stage: Referendum {
                    have_voted: Vec::new(),
                    votes_for: 0,
                    votes_against: 0
                }
            })
        } else {
            Err(self)
        }
    }
}

impl Procedure<Referendum> {
    pub fn votes_for(&self) -> u64 {
        self.stage.votes_for
    }

    pub fn votes_against(&self) -> u64 {
        self.stage.votes_against
    }

    pub fn register_vote_for(&mut self, person_id: PersonId) -> Result<(), ()> {
        let is_valid = self.motion.electors.contains(&person_id)
            && !self.stage.have_voted.contains(&person_id);

        if is_valid {
            self.stage.votes_for += 1;
            self.stage.have_voted.push(person_id);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn register_vote_against(&mut self, person_id: PersonId) -> Result<(), ()> {
        let is_valid = self.motion.electors.contains(&person_id)
            && !self.stage.have_voted.contains(&person_id);

        if is_valid {
            self.stage.votes_against += 1;
            self.stage.have_voted.push(person_id);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pass(self) -> Result<(), Self> {
        if self.stage.votes_for > self.stage.votes_against {
            Ok(())
        } else {
            Err(self)
        }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Prototype {}
    impl Sealed for super::Proposal {}
    impl Sealed for super::Petition {}
    impl Sealed for super::Referendum {}
}
