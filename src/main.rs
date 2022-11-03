use log::trace;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::HashSet;

#[derive(Debug)]
struct Prisonneer {
    id: usize,
    visited_at_least_once: bool,
    people_who_visited: HashSet<usize>,
    total_prisonneers: usize,
}

impl Prisonneer {
    fn new(id: usize, total_prisonneers: usize) -> Self {
        Self {
            id,
            visited_at_least_once: false,
            people_who_visited: HashSet::new(),
            total_prisonneers,
        }
    }
    /// the prisonneer visits the room
    /// return true if he knows that everyone already visited the room
    fn visit_room(&mut self, room: &mut Room, day: usize) -> bool {
        // push the lever up if prisonneer is picked on "his" day or any of visited prisonneers (day % prisonneer_count == id)
        // else push down
        self.visited_at_least_once = true;
        if room.lever == LeverState::Up {
            self.people_who_visited.insert(day - 1);
        }

        room.lever = if day % self.total_prisonneers == self.id {
            LeverState::Up
        } else {
            LeverState::Down
        };
        self.people_who_visited.len() == self.total_prisonneers - 1
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
enum LeverState {
    Up,
    #[default]
    Down,
}

#[derive(Default, Debug)]
struct Room {
    lever: LeverState,
}

impl Room {
    fn new() -> Self {
        Default::default()
    }
}

fn run_the_simulation(rng: &mut ThreadRng, prisonneer_count: usize) -> (usize, bool) {
    let mut room = Room::new();
    let mut prisonneers = (0..prisonneer_count)
        .map(|id| Prisonneer::new(id, prisonneer_count))
        .collect::<Vec<_>>();
    let mut day = 1;
    loop {
        let picked_prisonneer = prisonneers.choose_mut(rng).expect("No prisonneer found");
        if picked_prisonneer.visit_room(&mut room, day) {
            break;
        }
        trace!("room={:?}", room);
        trace!("prisonneers={:?}", prisonneers);
        day += 1;
    }
    (day, !prisonneers.iter().any(|p| !p.visited_at_least_once))
}

fn main() {
    env_logger::init();
    const SIMULATION_COUNT: usize = 1000;
    const PRISONNEER_COUNT: usize = 100;
    let mut rng = rand::thread_rng();
    let mut total_days = 0.;
    for _ in 0..SIMULATION_COUNT {
        let (days, succeeded) = run_the_simulation(&mut rng, PRISONNEER_COUNT);
        if !succeeded {
            println!("Simulation failed...");
            break;
        }
        total_days += days as f64;
    }
    let average_days = total_days / SIMULATION_COUNT as f64;
    println!(
        "Average days: {average_days:.0} ({:.2} years)",
        average_days / 365.
    );
}
