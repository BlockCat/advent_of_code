use aoc_2021::stopwatch;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::ControlFlow,
};

pub fn main() {
    let d1 = stopwatch(|| {
        println!("Ex1: {}", exercise_1(real_input_1()));
    })
    .unwrap();
    let d2 = stopwatch(|| {
        println!("Ex2: {}", exercise_1(real_input_2()));
    });

    println!("D1: {:?}", d1);
    println!("D2: {:?}", d2);
}

fn test_input_1() -> State<2> {
    State {
        hallway: [None; 11],
        room: [
            [Some(Phod(1)), Some(Phod(0))],
            [Some(Phod(2)), Some(Phod(3))],
            [Some(Phod(1)), Some(Phod(2))],
            [Some(Phod(3)), Some(Phod(0))],
        ],
    }
}

fn test_input_2() -> State<4> {
    State {
        hallway: [None; 11],
        room: [
            [Some(Phod(1)), Some(Phod(3)), Some(Phod(3)), Some(Phod(0))],
            [Some(Phod(2)), Some(Phod(2)), Some(Phod(1)), Some(Phod(3))],
            [Some(Phod(1)), Some(Phod(1)), Some(Phod(0)), Some(Phod(2))],
            [Some(Phod(3)), Some(Phod(0)), Some(Phod(2)), Some(Phod(0))],
        ],
    }
}

fn real_input_1() -> State<2> {
    State {
        hallway: [None; 11],
        room: [
            [Some(Phod(2)), Some(Phod(3))],
            [Some(Phod(2)), Some(Phod(0))],
            [Some(Phod(1)), Some(Phod(1))],
            [Some(Phod(3)), Some(Phod(0))],
        ],
    }
}

fn real_input_2() -> State<4> {
    State {
        hallway: [None; 11],
        room: [
            [Some(Phod(2)), Some(Phod(3)), Some(Phod(3)), Some(Phod(3))],
            [Some(Phod(2)), Some(Phod(2)), Some(Phod(1)), Some(Phod(0))],
            [Some(Phod(1)), Some(Phod(1)), Some(Phod(0)), Some(Phod(1))],
            [Some(Phod(3)), Some(Phod(0)), Some(Phod(2)), Some(Phod(0))],
        ],
    }
}

fn exercise_1<const N: usize>(input: State<N>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0usize), input.clone()));

    let mut visited = HashSet::new();
    while let Some((Reverse(cost), status)) = heap.pop() {
        if !visited.insert(status.clone()) {
            continue;
        }
        if status.is_completed() {
            return cost;
        }

        for (added_cost, state) in status.neighbouring_states() {
            heap.push((Reverse(cost + added_cost), state));
        }
    }

    unreachable!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Phod(u8);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State<const N: usize> {
    hallway: [Option<Phod>; 11],
    room: [[Option<Phod>; N]; 4],
}

impl<const N: usize> State<N> {
    fn replace_room(&self, room: usize, pos: usize, v: Option<Phod>) -> Self {
        let mut base_state = self.clone();
        base_state.room[room][pos] = v;
        base_state
    }

    fn replace_hallway(&self, hallway: usize, v: Option<Phod>) -> Self {
        let mut base_state = self.clone();
        base_state.hallway[hallway] = v;
        base_state
    }

    fn neighbouring_states(&self) -> Vec<(usize, State<N>)> {
        [1, 10, 100, 1000]
            .into_iter()
            .enumerate()
            .flat_map(|(phod, move_cost)| self.moves_for(Phod(phod as u8), move_cost))
            .collect()
    }   

    fn moves_for(&self, pod: Phod, move_cost: usize) -> Vec<(usize, State<N>)> {
        let mut candidated = Vec::with_capacity(7);
        // Move to outside of room
        for (room, positions) in self.room.iter().enumerate() {
            for pos in 0..positions.len() {
                if self.room[room][pos] == Some(pod) {
                    // It does not have to move anymore
                    if self.pod_has_to_move(room, pos, pod) {
                        candidated.extend(find_from_pos(self, room, pos, pod, move_cost));
                    }
                    break;
                } else if self.room[room][pos].is_some() {
                    break;
                }
            }
        }

        // Move pods in hallway
        for h in 0..11 {
            if self.hallway[h] != Some(pod) {
                continue;
            }
            let room = pod.0 as usize;
            let base_state = self.replace_hallway(h, None);
            let dest = room_exits_to(room);
            let (moves, reachable) = hallway_is_reachable(self, h, dest);
            if reachable {
                if !self.can_enter_room(room, pod) {
                    continue;
                }
                // Try to fill in bottom
                for pos in (0..N).rev() {
                    let moves = moves + pos + 1;
                    if self.room[room][pos].is_none() {
                        candidated.push((
                            moves * move_cost,
                            base_state.replace_room(room, pos, Some(pod)),
                        ));
                        break;
                    }
                }
            }
        }

        candidated
    }

    fn is_completed(&self) -> bool {
        (0..4).all(|x| self.is_room_complete(x))
    }

    fn is_room_complete(&self, room: usize) -> bool {
        self.room[room].iter().all(|x| x == &Some(Phod(room as u8)))
    }

    fn can_enter_room(&self, room: usize, pod: Phod) -> bool {
        if room as u8 != pod.0 {
            return false;
        }
        self.room[room].iter().all(|x| match x {
            Some(p) => p == &pod,
            None => true,
        })
    }

    fn pod_has_to_move(&self, room: usize, pos: usize, pod: Phod) -> bool {
        room as u8 != pod.0 || !self.room[room].iter().skip(pos).all(|x| x == &Some(pod))
    }
}

fn find_from_pos<const N: usize>(
    state: &State<N>,
    room: usize,
    pos: usize,
    pod: Phod,
    move_cost: usize,
) -> Vec<(usize, State<N>)> {
    let mut candidates = Vec::new();
    let exit_pos = room_exits_to(room);
    let mut possible_hallways = Vec::new();

    let base_state = state.replace_room(room, pos, None);

    // try left
    for l in (0..exit_pos).rev() {
        let moves = pos + 1 + exit_pos - l;
        if let ControlFlow::Break(_) = handle_hallway_candidates(
            state,
            l,
            pod,
            &mut candidates,
            moves,
            move_cost,
            &base_state,
            &mut possible_hallways,
        ) {
            break;
        }
    }
    for r in exit_pos + 1..11 {
        let moves = pos + 1 + r - exit_pos;
        if let ControlFlow::Break(_) = handle_hallway_candidates(
            state,
            r,
            pod,
            &mut candidates,
            moves,
            move_cost,
            &base_state,
            &mut possible_hallways,
        ) {
            break;
        }
    }

    for (moves, h) in possible_hallways {
        candidates.push((moves * move_cost, base_state.replace_hallway(h, Some(pod))));
    }

    candidates
}

fn handle_hallway_candidates<const N: usize>(
    state: &State<N>,
    hallway_pos: usize,
    pod: Phod,
    candidates: &mut Vec<(usize, State<N>)>,
    moves: usize,
    move_cost: usize,
    base_state: &State<N>,
    possible_hallways: &mut Vec<(usize, usize)>,
) -> ControlFlow<()> {
    if state.hallway[hallway_pos].is_some() {
        return ControlFlow::Break(());
    } else {
        match hallway_to_room(hallway_pos) {
            Some(room) => {
                if room as u8 == pod.0 {
                    if state.can_enter_room(room, pod) {
                        // Try to fill in bottom
                        for pos in (0..N).rev() {
                            let moves = moves + pos + 1;
                            if state.room[room][pos].is_none() {
                                candidates.push((
                                    moves * move_cost,
                                    base_state.replace_room(room, pos, Some(pod)),
                                ));
                                break;
                            }
                        }
                    }
                }
            }
            None => possible_hallways.push((moves, hallway_pos)),
        };
    }
    ControlFlow::Continue(())
}

fn hallway_is_reachable<const N: usize>(
    state: &State<N>,
    start: usize,
    dest: usize,
) -> (usize, bool) {
    if dest > start {
        (
            dest - start,
            (start + 1..=dest).all(|x| state.hallway[x].is_none()),
        )
    } else if dest < start {
        (
            start - dest,
            (dest..start).all(|x| state.hallway[x].is_none()),
        )
    } else {
        unreachable!()
    }
}

fn room_exits_to(room: usize) -> usize {
    match room {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => unreachable!(),
    }
}

fn hallway_to_room(hallway: usize) -> Option<usize> {
    match hallway {
        2 => Some(0),
        4 => Some(1),
        6 => Some(2),
        8 => Some(3),
        _ => None,
    }
}
