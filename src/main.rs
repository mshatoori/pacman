mod world;
mod pac;
mod pellet;
mod utils;

use crate::world::{WorldModel, Position};
use std::collections::{HashSet, VecDeque};
use crate::pellet::Pellet;
use std::time::Instant;

fn nearest_pellet(wm: &WorldModel, start: Position) -> Option<&Pellet> {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while let Some(pos) = to_visit.pop_front() {
        eprintln!("At {:?}", pos);
        visited.insert(pos);
        let neighbors = wm.neighbors(pos);
        eprintln!("Neighbors: {:?}", neighbors);
        for cell in neighbors {
            if !visited.contains(&cell) {
                eprintln!("Will visit: {:?}", cell);
                to_visit.push_back(cell);
            } else {
                eprintln!("Already visited: {:?}", cell);
            }
        }
        if let Some(pellet) = wm.pellet_at(pos) {
            return Some(pellet);
        }
    }
    None
}

/**
 * Grab the pellets as fast as you can!
 **/
fn main() {
    let mut wm = WorldModel::from_input();

    // game loop
    loop {
        let start = Instant::now();
        wm.update_by_input();

        let mut result = Vec::new();

        for pac in wm.get_team_pacs() {
            // Finding closest pellet
            eprintln!("Pac {} at {:?}", pac.id(), pac.pos());
            match nearest_pellet(&wm, pac.pos()) {
                None => {
                    eprintln!("Cant find pellet :(");
                    result.push(format!("MOVE {} {} {}", pac.id(), pac.pos().0, pac.pos().1))
                }
                Some(pellet) => {
                    eprintln!("Going to {:?}", pellet.pos());
                    result.push(format!("MOVE {} {} {}", pac.id(), pellet.pos().0, pellet.pos().1));
                }
            }
        }
        let result = result.join(" | ");
        println!("{}", result);

        let duration = start.elapsed();

        eprintln!("Time elapsed in step is: {:?}", duration);
    }
}
