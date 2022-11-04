use log::{error, info, trace};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug)]
struct JulesPrisonneerDumb {
    id: usize,
    visited_at_least_once: bool,
    total_prisonneers: usize,
}
#[derive(Debug)]
struct JulesPrisonneerClever {
    id: usize,
    visited_at_least_once: bool,
    people_who_visited: HashSet<usize>,
    total_prisonneers: usize,
}
#[derive(Debug)]
struct JulesPrisonneerCleverActuallyDumb {
    id: usize,
    visited_at_least_once: bool,
    people_who_visited: HashSet<usize>,
    total_prisonneers: usize,
}

trait Prisonneer {
    fn new(id: usize, total_prisonneers: usize) -> Self;
    fn id(&self) -> usize;
    fn visit_room(&mut self, room: &mut Room, day: usize) -> bool;
    fn has_visited(&self) -> bool;
}

impl Prisonneer for JulesPrisonneerClever {
    fn id(&self) -> usize {
        self.id
    }
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
        self.people_who_visited.insert(self.id);
        if room.lever == LeverState::Up {
            self.people_who_visited
                .insert((day - 1) % self.total_prisonneers);
        }
        let expected_prisonneer = day % self.total_prisonneers;

        room.lever = if expected_prisonneer == self.id
            || self.people_who_visited.contains(&expected_prisonneer)
        {
            LeverState::Up
        } else {
            LeverState::Down
        };
        self.people_who_visited.len() == self.total_prisonneers
    }
    fn has_visited(&self) -> bool {
        self.visited_at_least_once
    }
}
impl Prisonneer for JulesPrisonneerCleverActuallyDumb {
    fn id(&self) -> usize {
        self.id
    }
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
        self.people_who_visited.insert(self.id);

        // cycle 0
        // 0 1 2 3 4 5 6 7 8 9   day
        // _ _ _ _ _ _ ' ' ' '   lever
        // - - - - - - + + + +   knowledge
        // 4 6 2 8 4 3 0 5 4 7   prisonneer
        // cycle 1
        // 0 1 2 3 4 5 6 7 8 9   day
        // _ _ _ _ _ _ ' ' ' '   lever
        // - - - - - - + + + +   knowledge
        // 3 2 2 0 9 8 1 5 0 4   prisonneer
        let current_cycle = (day / (1 * self.total_prisonneers)) % self.total_prisonneers;

        if day % self.total_prisonneers == 0 {
            if room.lever == LeverState::Up {
                self.people_who_visited
                    .insert((current_cycle - 1) % self.total_prisonneers);
            }
            room.lever = LeverState::Down;
        }
        if self.id == current_cycle || self.people_who_visited.contains(&current_cycle) {
            room.lever = LeverState::Up;
        }

        self.people_who_visited.len() == self.total_prisonneers
    }
    fn has_visited(&self) -> bool {
        self.visited_at_least_once
    }
}

impl Prisonneer for JulesPrisonneerDumb {
    fn id(&self) -> usize {
        self.id
    }
    fn new(id: usize, total_prisonneers: usize) -> Self {
        Self {
            visited_at_least_once: false,
            total_prisonneers,
            id,
        }
    }
    /// the prisonneer visits the room
    /// return true if he knows that everyone already visited the room
    fn visit_room(&mut self, _room: &mut Room, day: usize) -> bool {
        // push the lever up if prisonneer is picked on "his" day or any of visited prisonneers (day % prisonneer_count == id)
        // else push down
        self.visited_at_least_once = true;

        day > self.total_prisonneers * 25
    }
    fn has_visited(&self) -> bool {
        self.visited_at_least_once
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

fn run_the_simulation<T: Prisonneer + Debug>(
    rng: &mut ThreadRng,
    prisonneer_count: usize,
) -> (usize, bool) {
    let mut room = Room::new();
    let mut prisonneers = (0..prisonneer_count)
        .map(|id| T::new(id, prisonneer_count))
        .collect::<Vec<_>>();
    let mut day = 0;
    trace!("day={}", day);
    trace!("prisonneers={:?}", prisonneers);
    trace!("room={:?}", room);
    loop {
        let picked_prisonneer = prisonneers.choose_mut(rng).expect("No prisonneer found");
        if picked_prisonneer.visit_room(&mut room, day) {
            break;
        }
        day += 1;
        trace!("day={}", day);
        trace!("picked_prisonneer={:?}", picked_prisonneer);
        trace!("prisonneers={:?}", prisonneers);
        trace!("room={:?}", room);
    }
    (day, !prisonneers.iter().any(|p| !p.has_visited()))
}

/*
 * - State of lever before passage at each passage
 * - State of lever after passage at each passage
 * - Day of each passage
 * - Day number
 * - Total number of prisonneer
 * - Id of prisonneer
 * - Passage count (including no passage)
 * - State of lever
 * - No one is already certain
 * -
 */
fn main() {
    env_logger::init();
    const SIMULATION_COUNT: usize = 1000;
    const PRISONNEER_COUNT: usize = 100;
    let mut rng = rand::thread_rng();
    let mut total_days = 0.;
    for _ in 0..SIMULATION_COUNT {
        trace!("Running a simulation");
        let (days, succeeded) =
            run_the_simulation::<JulesPrisonneerClever>(&mut rng, PRISONNEER_COUNT);
        if !succeeded {
            error!("Simulation failed...");
            break;
        }
        total_days += days as f64;
    }
    let average_days = total_days / SIMULATION_COUNT as f64;
    info!(
        "Average days: {average_days:.0} ({:.2} years)",
        average_days / 365.
    );
}
