use crate::lib::action::Action;
use crate::lib::local::Local;
use crate::lib::state::{neighbours, Board, State};
use bit_set::BitSet;
use serde::export::fmt::Display;
use serde::export::Formatter;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use tungstenite::accept;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct LucidAction {
    pub covered: BitSet<usize>,
    pub value: u32,
}

#[derive(Clone)]
pub struct LucidState<'a> {
    pub actions: Vec<LucidAction>,
    pub score: u32,
    pub data: &'a ActionData,
    pub wildcards: BitSet<usize>,
}

pub type ActionData = HashMap<BitSet<usize>, BitSet<usize>>;

pub trait CreateActions {
    fn create() -> Self;
}

impl CreateActions for ActionData {
    fn create() -> Self {
        let mut data = HashMap::new();

        fn get_action(data: &mut ActionData, key: BitSet<usize>) -> &mut BitSet<usize> {
            data.entry(key).or_insert_with(Default::default)
        }

        fn find_extensions(data: &mut ActionData, action: Vec<usize>) {
            let last = *action.last().unwrap();
            let local = neighbours(last);
            for neighbour in local {
                if !action.contains(&neighbour) {
                    let mut branch = action.clone();
                    branch.push(neighbour);
                    find_extensions(data, branch);
                }
            }
            let ends = get_action(data, action.into_iter().collect());
            ends.insert(last);
        }

        for i in 0..25 {
            find_extensions(&mut data, vec![i]);
        }
        data
    }
}

impl<'a> LucidState<'a> {
    pub fn create(state: &State, data: &'a ActionData) -> Self {
        let mut actions = Vec::new();
        for action in state.actions() {
            let new_action = LucidAction {
                value: state.board[action[0]],
                covered: action.into_iter().collect(),
            };
            if !actions
                .iter()
                .any(|a: &LucidAction| a.covered == new_action.covered)
            {
                actions.push(new_action);
            }
        }
        LucidState {
            actions,
            score: state.score,
            data,
            wildcards: Default::default(),
        }
    }

    fn set_tile(&mut self, index: usize, value: u32) {
        let mut local_actions = Vec::new();
        local_actions.push(LucidAction {
            covered: vec![index].into_iter().collect(),
            value,
        });
        for n in neighbours(index) {
            let mut local_local_actions = Vec::new();
            for action in &self.actions {
                if self.data.get(&action.covered).unwrap().contains(n) {
                    for local_action in &local_actions {
                        if action.value == local_action.value
                            && action.covered.is_disjoint(&local_action.covered)
                            && self.data.get(&local_action.covered).unwrap().contains(n)
                        {
                            let mut new_action = action.clone();
                            new_action.covered.union_with(&local_action.covered);
                            local_local_actions.push(new_action)
                        }
                    }
                }
            }
            local_actions.extend(local_local_actions);
        }
        self.actions.extend(local_actions);
    }

    fn next(&self, covered: &BitSet<usize>, end: usize, value: u32) -> Self {
        let mut new_state = self.clone();
        let new_value = covered.len() as u32 * value;
        new_state.score += new_value;
        new_state
            .actions
            .retain(|old| old.covered.is_disjoint(&covered));

        new_state.set_tile(end, new_value);
        new_state.wildcards.remove(end);
        for index in covered {
            if index != end {
                for v in 1..4 {
                    new_state.set_tile(index, v)
                }
                new_state.wildcards.insert(index);
            }
        }
        new_state
    }

    fn action_value(
        &self,
        action: &LucidAction,
        visited: &mut HashMap<Vec<LucidAction>, u32>,
    ) -> u32 {
        if action
            .covered
            .iter()
            .filter(|pos| self.wildcards.contains(*pos))
            .count()
            > 0
        {
            return self.score;
        };

        let mut max_value = 0;
        for end in self.data.get(&action.covered).unwrap() {
            max_value = self
                .next(&action.covered, end, action.value)
                .state_value(visited)
                .max(max_value)
        }
        max_value
    }

    fn state_value(&self, visited: &mut HashMap<Vec<LucidAction>, u32>) -> u32 {
        let key = self.actions.clone();
        if let Some(value) = visited.get(&key) {
            return value.clone();
        }
        let mut value_actions = BTreeMap::new();
        for action in &self.actions {
            if action.covered.len() > 1 {
                value_actions
                    .entry(Reverse(self.action_value(action, visited)))
                    .or_insert_with(Vec::new)
                    .push(action)
            }
        }
        let mut wildcard_index = HashMap::new();
        for (i, wildcard) in self.wildcards.iter().enumerate() {
            wildcard_index.insert(wildcard, i);
        }

        let mut total = 0;
        'spot: for i in 1..3usize.pow(self.wildcards.len() as u32) {
            for (Reverse(value), action_list) in &value_actions {
                'action: for action in action_list {
                    for index in &action.covered {
                        if let Some(&j) = wildcard_index.get(&index) {
                            let x = (i as u32 / 3u32.pow(j as u32)) % 3;
                            if x != action.value {
                                continue 'action;
                            }
                        }
                    }
                    total += *value;
                    continue 'spot;
                }
            }
            total += self.score;
        }
        let value = total / 3f32.powf(self.wildcards.len() as f32);
        visited.insert(key, value);
        value
    }

    pub fn best_action(&self) -> Action {
        let mut best: Option<(u32, &BitSet<usize>, usize)> = None;
        let mut visited = HashMap::new();
        for action in &self.actions {
            println!("action done");
            if action.covered.len() > 1 {
                for end in self.data.get(&action.covered).unwrap() {
                    let value = self
                        .next(&action.covered, end, action.value)
                        .state_value(&mut visited);
                    if best.is_none() || value > best.unwrap().0 {
                        best = Some((value, &action.covered, end));
                    }
                }
            }
        }
        let (_, covered, end) = best.unwrap();

        fn find_action(covered: &BitSet<usize>, action: Vec<usize>) -> Option<Vec<usize>> {
            if action.len() == covered.len() {
                return Some(action);
            }
            for n in neighbours(*action.last().unwrap()) {
                if covered.contains(n) && !action.contains(&n) {
                    let new_action = action.clone();
                    let res = find_action(covered, new_action);
                    if res.is_some() {
                        return res;
                    }
                }
            }
            return None;
        }

        let mut action = find_action(covered, vec![end]).unwrap();
        action.reverse();
        action
    }

    pub fn show_actions(&self) {
        for action in &self.actions {
            println!("{:?}", action.covered)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let local = Local::new(123);
        let data = ActionData::create();
        assert_eq!(
            data.get(&[0, 1, 2, 5].to_vec().into_iter().collect()),
            Some(&[2, 5].to_vec().into_iter().collect())
        );
        local.state.show_board();
        LucidState::create(&local.state, &data).show_actions();
    }
}
