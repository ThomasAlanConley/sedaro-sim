use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Range;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    time: f64,
    time_step: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

fn propagate(agent_id: &str, universe: &HashMap<String, State>) -> State {
    let state = universe.get(agent_id).unwrap();
    let mut time = state.time;
    let mut time_step = state.time_step;
    let mut x = state.x;
    let mut y = state.y;
    let mut vx = state.vx;
    let mut vy = state.vy;

    match agent_id {
        "Planet" => {
            x += vx * time_step;
            y += vy * time_step;
        }
        "Satellite" => {
            let planet = universe.get("Planet").unwrap();
            let px = planet.x;
            let py = planet.y;
            let dx = px - x;
            let dy = py - y;
            let distance_squared = dx * dx + dy * dy;
            let distance_cubed = distance_squared.powf(3.0 / 2.0);
            let fx = dx / distance_cubed;
            let fy = dy / distance_cubed;
            vx += fx * time_step;
            vy += fy * time_step;
            x += vx * time_step;
            y += vy * time_step;
        }
        _ => {}
    }

    time += time_step;
    time_step = 0.01 + rand::thread_rng().gen::<f64>() * 0.09;

    State {
        time,
        time_step,
        x,
        y,
        vx,
        vy,
    }
}

#[derive(Default, Debug)]
struct QRangeStore {
    store: Vec<(f64, f64, HashMap<String, State>)>,
}

impl QRangeStore {
    fn new() -> Self {
        Self { store: vec![] }
    }

    fn set(&mut self, range: Range<f64>, value: HashMap<String, State>) {
        if range.start >= range.end {
            panic!("Invalid Range.");
        }
        self.store.push((range.start, range.end, value));
    }

    fn get(&self, key: f64) -> Result<Vec<&HashMap<String, State>>, &str> {
        let ret: Vec<&HashMap<String, State>> = self
            .store
            .iter()
            .filter(|&&(l, h, _)| l <= key && key < h)
            .map(|&(_, _, ref v)| v)
            .collect();
        if ret.is_empty() {
            Err("Not found.")
        } else {
            Ok(ret)
        }
    }
}

fn read(t: f64, store: &QRangeStore) -> HashMap<String, State> {
    match store.get(t) {
        Ok(data) => data.iter().fold(HashMap::new(), |acc, &map| {
            map.iter().fold(acc, |mut acc, (k, v)| {
                acc.insert(k.clone(), v.clone());
                acc
            })
        }),
        Err(_) => HashMap::new(),
    }
}

fn main() {
    let init = HashMap::from([
        (
            "Planet".to_string(),
            State {
                time: 0.0,
                time_step: 0.01,
                x: 0.0,
                y: 0.1,
                vx: 0.1,
                vy: 0.0,
            },
        ),
        (
            "Satellite".to_string(),
            State {
                time: 0.0,
                time_step: 0.01,
                x: 0.0,
                y: 1.0,
                vx: 1.0,
                vy: 0.0,
            },
        ),
    ]);

    let mut store = QRangeStore::new();
    store.set(-999999999.0..0.0, init.clone());

    let mut times: HashMap<String, f64> = init
        .iter()
        .map(|(agent_id, state)| (agent_id.clone(), state.time))
        .collect();

    for _ in 0..1000 {
        for agent_id in init.keys() {
            let t = *times.get(agent_id).unwrap();
            let universe = read(t - 0.001, &store);
            if universe.keys().collect::<Vec<_>>() == init.keys().collect::<Vec<_>>() {
                let new_state = propagate(agent_id, &universe);
                let mut new_entry = HashMap::new();
                new_entry.insert(agent_id.clone(), new_state.clone());
                store.set(t..new_state.time, new_entry);
                times.insert(agent_id.clone(), new_state.time);
            }
        }
    }

    let json_data = serde_json::to_string_pretty(&store.store).unwrap();
    let mut file = File::create("../sedaro-nano/app/public/data.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}
