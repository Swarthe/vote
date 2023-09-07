use vote::{Procedure, Person, PersonList, Motion};
use vote::procedure::{Prototype, Proposal, Petition, Referendum};

use rand::Rng;

use chrono::Duration;

const POPULATION_SIZE: u64 = 21;
const DEVELOPER_COUNT: u64 = 4;
const VOTE_CHANCE: f64 = 0.8;
const PROPOSAL_SECS: i64 = 4;

type Result<T> = std::result::Result<T, ()>;

fn main() -> Result<()> {
    println!();

    let persons = build_population();
    let motion = build_motion(&persons);

    let prototype = build_prototype(motion);
    let proposal = build_proposal(prototype, &persons)?;
    let petition = build_petition(proposal);
    let referendum = build_referendum(petition, &persons)?;

    pass_motion(referendum, &persons)
}

fn build_population() -> PersonList {
    use rnglib::{RNG, Language};

    let rng = RNG::from(&Language::Roman);

    let persons = (0..POPULATION_SIZE).map(|_| Person {
        name: rng.generate_short() + " " + &rng.generate_name()
    }).collect();

    print!("--- The population of Exampletown ({POPULATION_SIZE})\n\n");
    print!("{persons}\n\n");
    pause_short();

    persons
}

fn build_motion(persons: &PersonList) -> Motion {
    let motion = Motion {
        title: "Construction of a new monument in Exampletown",
        description: "Exampletown is too empty. A monument must be built.",
        developers: persons.rand_choices(DEVELOPER_COUNT).into(),
        electors: persons.ids().collect()
    };

    print!("--- The motion\n\n");
    print!("{motion}\n\n");
    pause_long();

    print!("--- The developers of the motion ({DEVELOPER_COUNT})\n\n");
    motion.developers.iter().for_each(|id| println!("{}", persons[*id].name));
    print!("\n");
    pause_short();

    motion
}

fn build_prototype(motion: Motion) -> Procedure<Prototype> {
    let prototype = Procedure::begin(motion);

    print!("--- Stage 1: Prototype\n");
    print!("--- The developers publicly refine the motion.\n");
    print!("--- Then, they vote to propose the completed motion.\n\n");
    pause_long();

    prototype
}

fn build_proposal(
    mut prototype: Procedure<Prototype>,
    persons: &PersonList
) -> Result<Procedure<Proposal>> {
    let mut rng = rand::thread_rng();

    print!(
        "{} votes for proposal required. Voters:\n\n",
        prototype.motion().dev_count() / 2 + 1
    );

    pause_short();

    for idx in 0..prototype.motion().dev_count() {
        let dev_id = prototype.motion().developers[idx];

        println!("{}", persons[dev_id].name);
        pause_micro();

        if rng.gen_bool(VOTE_CHANCE) {
            prototype.register_proposal_vote(dev_id).unwrap();
        }
    }

    print!("\n{} votes registered for proposal.\n\n", prototype.proposal_votes());
    pause_short();

    let proposal = prototype.into_proposal(Duration::seconds(PROPOSAL_SECS))
        .map_err(|_| println!("Insufficient votes for proposal"))?;

    print!("--- Stage 2: Proposal\n");
    print!("--- The completed motion is subject public debate for a limited time.\n");
    print!("--- Debate end date: {}.\n\n", proposal.end_date());
    pause_long();

    Ok(proposal)
}

fn build_petition(mut proposal: Procedure<Proposal>) -> Procedure<Petition> {
    let petition = loop {
        match proposal.into_petition() {
            Ok(pet) => {
                print!("Proposal stage end date reached.\n\n");
                pause_short();
                break pet;
            }

            Err(pro) => proposal = pro
        }
    };

    print!("--- Stage 3: Petition\n");
    print!("--- The motion is subject to a vote of approval by a subset of the population.\n");
    print!("--- If approved, the motion is shown to the population for a general vote.\n\n");
    pause_long();

    petition
}

fn build_referendum(
    mut petition: Procedure<Petition>,
    persons: &PersonList
) -> Result<Procedure<Referendum>> {
    let mut rng = rand::thread_rng();
    let voter_ids = petition.voter_ids().to_vec();

    print!(
        "{} votes for referendum required. Voters:\n\n",
        voter_ids.len() / 2 + 1
    );

    pause_short();

    for id in voter_ids {
        println!("{}", persons[id].name);
        pause_micro();

        if rng.gen_bool(VOTE_CHANCE) {
            petition.register_approval_vote(id).unwrap();
        }
    }

    print!("\n{} votes registered for referendum.\n\n", petition.votes_for());
    pause_short();

    let referendum = petition.into_referendum()
        .map_err(|_| println!("Insufficient votes for referendum"))?;

    print!("--- Stage 4: Referendum\n");
    print!("--- The motion is subject to a general vote by the population.\n");
    print!("--- If it receives more votes for than against, it is passed.\n\n");
    pause_long();

    Ok(referendum)
}

fn pass_motion(
    mut referendum: Procedure<Referendum>,
    persons: &PersonList
) -> Result<()> {
    let mut rng = rand::thread_rng();

    print!("Voters:\n\n");
    pause_short();

    for id in persons.ids() {
        println!("{}", persons[id].name);
        pause_micro();

        if rng.gen_bool(VOTE_CHANCE) {
            referendum.register_vote_for(id).unwrap();
        } else {
            referendum.register_vote_against(id).unwrap();
        }
    }

    print!("\n{} votes registered for.\n", referendum.votes_for());
    print!("{} votes registered against.\n\n", referendum.votes_against());
    pause_short();

    if let Ok(()) = referendum.pass() {
        print!("--- The motion is passed.\n");
        Ok(())
    } else {
        print!("--- The motion is rejected.\n");
        Err(())
    }
}

fn pause_micro() { sleep_secs(1) }
fn pause_short() { sleep_secs(3) }
fn pause_long()  { sleep_secs(5) }

fn sleep_secs(n: u64) {
    use std::time::Duration;

    std::thread::sleep(Duration::from_secs(n));
}
