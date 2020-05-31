use crate::lib::action::Action;
use crate::lib::local::Local;
use crate::lib::state::{neighbours, State};
use bit_set::BitSet;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct LucidState<'a> {
    pub actions: HashMap<BitSet, Option<(u32, bool)>>,
    pub score: u32,
    pub data: &'a ActionData,
    pub depth: u32,
}

pub type ActionData = HashMap<BitSet, BitSet>;

pub trait CreateActions {
    fn create() -> Self;
}

impl CreateActions for ActionData {
    fn create() -> Self {
        let mut data = HashMap::new();

        fn get_action(data: &mut ActionData, key: BitSet) -> &mut BitSet {
            data.entry(key).or_insert_with(Default::default)
        }

        fn find_extensions(data: &mut ActionData, action: Vec<usize>) {
            let last = *action.last().unwrap();
            let local = neighbours(last);
            if action.len() < 4 {
                for neighbour in local {
                    if !action.contains(&neighbour) {
                        let mut branch = action.clone();
                        branch.push(neighbour);
                        find_extensions(data, branch);
                    }
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
        let mut actions: HashMap<BitSet, Option<(u32, bool)>> = HashMap::new();
        for action in state.actions() {
            let value = Some((state.board[action[0]], false));
            if action.len() <= 4 {
                actions.insert(action.into_iter().collect(), value);
            }
        }
        LucidState {
            actions,
            score: state.score,
            data,
            depth: 0,
        }
    }

    fn set_tile(&mut self, index: usize, value: Option<(u32, bool)>) {
        let mut local_actions: HashMap<BitSet, Option<(u32, bool)>> = HashMap::new();
        local_actions.insert(vec![index].into_iter().collect(), value);

        for (covered, &value) in &self.actions {
            let mut local_local_actions = HashMap::new();

            for (local_covered, &local_value) in &local_actions {
                let new_covered = covered.union(local_covered).collect();

                if self.data.contains_key(&new_covered) {
                    if let Some((v, a)) = value {
                        if let Some((w, b)) = local_value {
                            if v == w && !(a && b) {
                                local_local_actions.insert(new_covered, Some((v, a || b)));
                            }
                        } else {
                            if (v == 1 || v == 2 || v == 3) && !a {
                                local_local_actions.insert(new_covered, Some((v, true)));
                            }
                        }
                    } else if let Some((w, b)) = local_value {
                        if (w == 1 || w == 2 || w == 3) && !b {
                            local_local_actions.insert(new_covered, Some((w, true)));
                        }
                    }
                }
            }
            local_actions.extend(local_local_actions);
        }
        self.actions.extend(local_actions);
    }

    fn next(&self, covered: &BitSet, value: u32, end: usize) -> Self {
        let mut new_state = self.clone();
        let new_value = covered.len() as u32 * value;
        new_state.score += new_value;
        new_state.actions.retain(|old, _| old.is_disjoint(&covered));

        new_state.set_tile(end, Some((new_value, false)));
        for index in covered {
            if index != end {
                new_state.set_tile(index, None)
            }
        }
        new_state.depth += 1;
        new_state
    }

    fn action_value(
        &self,
        covered: &BitSet,
        value: u32,
        store: &mut HashMap<Vec<(BitSet, Option<(u32, bool)>)>, u32>,
    ) -> u32 {
        let mut max_value = 0;
        for end in &self.data[covered] {
            max_value = self
                .next(covered, value, end)
                .state_value(store)
                .max(max_value)
        }
        max_value
    }

    pub fn create_key(&self) -> Vec<(BitSet, Option<(u32, bool)>)> {
        let mut k: Vec<(BitSet, Option<(u32, bool)>)> = self
            .actions
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        k.sort_by_key(|(k, v)| k.clone());
        return k;
    }

    fn state_value(&self, store: &mut HashMap<Vec<(BitSet, Option<(u32, bool)>)>, u32>) -> u32 {
        if self.depth > 3 {
            let mut total = self.score;
            for (covered, value) in &self.actions {
                if let Some((v, false)) = value {
                    total += covered.len() as u32 * v;
                }
            }
            return total;
        }
        let key = self.create_key();
        if store.contains_key(&key) {
            return store[&key];
        }

        let mut value_actions = BTreeMap::new();
        let mut wildcard_possibilities: HashMap<usize, BitSet> = HashMap::new();
        for (covered, value) in &self.actions {
            if covered.len() > 1 {
                value_actions
                    .entry(Reverse(self.action_value(covered, value.unwrap().0, store)))
                    .or_insert_with(Vec::new)
                    .push((covered, value.unwrap()))
            } else if value.is_none() {
                wildcard_possibilities.insert(
                    covered.iter().next().unwrap(),
                    vec![1, 2, 3].into_iter().collect(),
                );
            }
        }

        let mut total = 0;
        let mut done = false;
        // let mut test = 0;
        'value: for (Reverse(score), action_list) in value_actions {
            for (covered, value) in action_list {
                let mut p = 1;
                done = true;
                for (&wildcard, possibilities) in &mut wildcard_possibilities {
                    if covered.contains(wildcard) {
                        done = false;
                        if possibilities.contains(value.0 as usize) {
                            possibilities.remove(value.0 as usize);
                        } else {
                            p = 0
                        }
                    } else {
                        p *= possibilities.len() as u32;
                    }
                }
                total += p * score;
                // test += p;
                if done {
                    break 'value;
                }
            }
        }
        if !done {
            let p: u32 = wildcard_possibilities
                .values()
                .map(|p| p.len() as u32)
                .product();
            total += p * self.score;
            // test += p;
        }
        // assert_eq!(test, 3u32.pow(wildcard_possibilities.len() as u32));
        let score = total / 3u32.pow(wildcard_possibilities.len() as u32);
        store.insert(key, score);
        score
    }

    pub fn best_action(&self) -> Action {
        let mut best: Option<(u32, &BitSet, usize)> = None;
        let mut store = HashMap::new();
        for (covered, value) in &self.actions {
            if covered.len() > 1 {
                for end in &self.data[covered] {
                    let value = self
                        .next(covered, value.unwrap().0, end)
                        .state_value(&mut store);
                    if best.is_none() || value > best.unwrap().0 {
                        best = Some((value, covered, end));
                    }
                }
            }
        }
        let (_, covered, end) = best.unwrap();

        fn find_action(covered: &BitSet, action: Vec<usize>) -> Option<Vec<usize>> {
            if action.len() == covered.len() {
                return Some(action);
            }
            for n in neighbours(*action.last().unwrap()) {
                if covered.contains(n) && !action.contains(&n) {
                    let mut new_action = action.clone();
                    new_action.push(n);
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
            println!("{:?}", action.0)
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
            data.get(&vec![0, 1, 2, 5].into_iter().collect()),
            Some(&vec![2, 5].into_iter().collect())
        );
        local.state.show_board();

        let key1 = LucidState::create(&local.state, &data)
            .next(&vec![0, 1, 2].into_iter().collect(), 1, 2)
            .next(&vec![12, 7].into_iter().collect(), 3, 7)
            .create_key();

        let key2 = LucidState::create(&local.state, &data)
            .next(&vec![12, 7].into_iter().collect(), 3, 7)
            .next(&vec![0, 1, 2].into_iter().collect(), 1, 2)
            .create_key();

        assert_eq!(key1, key2);
    }
}
