use home;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::app::Penalty;

#[derive(Debug)]
pub struct Times {
    pub file_path: PathBuf,
    pub times: Value,
}

impl Times {
    pub fn new() -> Self {
        // get home directory
        let home = home::home_dir().unwrap();
        let file_name = "times.json";
        // create path to file
        let mut path = PathBuf::new();
        path.push(home);
        path.push(".cargo/bin/cube-times");
        path.push(file_name);
        // If the file doesn't exist, create it.
        if !path.exists() {
            // create the folder first
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut file = File::create(&path).unwrap();

            // create the base json
            let json = r#"{
                "bests": {
                    "single": {
                        "time": 0,
                        "scramble": ""
                    },
                    "ao5": {
                        "time": 0,
                        "times": []
                    },
                    "ao12": {
                        "time": 0,
                        "times": []
                    }
                },
                "times": []
            }"#;

            // write the json to the file
            file.write_all(json.as_bytes()).unwrap();

            // close the file
            drop(file);
        }

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open: {}", why),
            Ok(file) => file,
        };

        // read the file to a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let times: Value = serde_json::from_str(&contents).unwrap();

        Self {
            file_path: path,
            times,
        }
    }

    pub fn save_to_file(&self) {
        // write times to the file name, overrwrite old file
        let mut file = File::create(&self.file_path).unwrap();

        let json = serde_json::to_string_pretty(&self.times).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        drop(file);
    }

    pub fn del_time(&mut self, index: usize) {
        // delete time from times
        if index >= self.num_times() {
            return;
        }
        let deleted_time = self.times["times"].as_array_mut().unwrap().remove(index);
        self.update_after_delete(deleted_time["time"].as_f64().unwrap());
    }

    pub fn del_last(&mut self) {
        // delete last time from times
        if self.num_times() == 0 {
            return;
        }
        self.del_time(self.num_times() - 1);
    }

    pub fn add_time(&mut self, time: f64, scramble: String, penalty: Penalty) {
        // add time to times

        let object = json!({
            "time": time,
            "scramble": scramble,
            "penalty": penalty.to_string(),
        });

        self.times["times"].as_array_mut().unwrap().push(object);
        self.update_after_add();
    }

    pub fn num_times(&self) -> usize {
        // get number of times
        self.times["times"].as_array().unwrap().len()
    }

    pub fn update_after_delete(&mut self, time: f64) {
        if self.num_times() == 0 {
            // if there are no times, clear all best times
            self.times["bests"]["single"]["time"] = Value::from(0.0);
            self.times["bests"]["single"]["scramble"] = Value::from("");
            self.times["bests"]["ao5"]["time"] = Value::from(0.0);
            self.times["bests"]["ao5"]["times"]
                .as_array_mut()
                .unwrap()
                .clear();
            self.times["bests"]["ao12"]["time"] = Value::from(0.0);
            self.times["bests"]["ao12"]["times"]
                .as_array_mut()
                .unwrap()
                .clear();
        }

        if time == self.times["bests"]["single"]["time"].as_f64().unwrap() {
            // if the deleted time was the best single, update best single
            let mut best_single_time = -1.0;
            let mut best_single_scramble = self.times["bests"]["single"]["scramble"].to_string();

            for time in self.times["times"].as_array().unwrap() {
                if best_single_time == -1.0 || time["time"].as_f64().unwrap() < best_single_time {
                    best_single_time = time["time"].as_f64().unwrap();
                    best_single_scramble = time["scramble"].as_str().unwrap().to_string();
                }
            }

            self.times["bests"]["single"]["time"] = Value::from(best_single_time);
            self.times["bests"]["single"]["scramble"] = Value::from(best_single_scramble);
        }

        // if the deleted time was the best ao5, update best ao5
        // recalculate all ao5s, and find the best one
        let mut best_ao5_time = -1.0;
        let mut best_ao5_times: Vec<Value> = vec![];
        for i in 0..self.num_times() {
            self.times["times"][i]["ao5"] = Value::from(self.calc_ao5(i));
            if i > 4
                && (best_ao5_time == -1.0
                    || self.times["times"][i]["ao5"].as_f64().unwrap() < best_ao5_time)
            {
                best_ao5_time = self.times["times"][i]["ao5"].as_f64().unwrap();
                best_ao5_times = vec![
                    self.times["times"][i - 4].clone(),
                    self.times["times"][i - 3].clone(),
                    self.times["times"][i - 2].clone(),
                    self.times["times"][i - 1].clone(),
                    self.times["times"][i].clone(),
                ];
            }
        }
        self.times["bests"]["ao5"]["time"] = Value::from(best_ao5_time);
        self.times["bests"]["ao5"]["times"] = Value::from(best_ao5_times);

        // update ao12
        let mut best_ao12_time = -1.0;
        let mut best_ao12_times: Vec<Value> = vec![];
        for i in 0..self.num_times() {
            self.times["times"][i]["ao12"] = Value::from(self.calc_ao12(i));
            if i > 11
                && (best_ao12_time == -1.0
                    || self.times["times"][i]["ao12"].as_f64().unwrap() < best_ao12_time)
            {
                best_ao12_time = self.times["times"][i]["ao12"].as_f64().unwrap();
                best_ao12_times = vec![
                    self.times["times"][i - 11].clone(),
                    self.times["times"][i - 10].clone(),
                    self.times["times"][i - 9].clone(),
                    self.times["times"][i - 8].clone(),
                    self.times["times"][i - 7].clone(),
                    self.times["times"][i - 6].clone(),
                    self.times["times"][i - 5].clone(),
                    self.times["times"][i - 4].clone(),
                    self.times["times"][i - 3].clone(),
                    self.times["times"][i - 2].clone(),
                    self.times["times"][i - 1].clone(),
                    self.times["times"][i].clone(),
                ];
            }
        }

        self.times["bests"]["ao12"]["time"] = Value::from(best_ao12_time);
        self.times["bests"]["ao12"]["times"] = Value::from(best_ao12_times);
    }

    pub fn update_after_add(&mut self) {
        // SINGLES
        // check if the new time is the best single
        if (self.num_times() == 1)
            || (self.times["times"][self.num_times() - 1]["time"]
                .as_f64()
                .unwrap()
                < self.times["bests"]["single"]["time"].as_f64().unwrap())
        {
            self.times["bests"]["single"]["time"] =
                self.times["times"][self.num_times() - 1]["time"].clone();
            self.times["bests"]["single"]["scramble"] =
                self.times["times"][self.num_times() - 1]["scramble"].clone();
        }

        // AO5
        for i in 0..self.num_times() {
            self.times["times"][i]["ao5"] = Value::from(self.calc_ao5(i));
        }

        // check if the new time is the best ao5
        if (self.num_times() >= 5)
            && (self.times["times"][self.num_times() - 1]["ao5"]
                .as_f64()
                .unwrap()
                < self.times["bests"]["ao5"]["time"].as_f64().unwrap()
                || self.times["bests"]["ao5"]["time"].as_f64().unwrap() == 0.0)
        {
            self.times["bests"]["ao5"]["time"] =
                self.times["times"][self.num_times() - 1]["ao5"].clone();
            self.times["bests"]["ao5"]["times"] = Value::from(vec![
                self.times["times"][self.num_times() - 5].clone(),
                self.times["times"][self.num_times() - 4].clone(),
                self.times["times"][self.num_times() - 3].clone(),
                self.times["times"][self.num_times() - 2].clone(),
                self.times["times"][self.num_times() - 1].clone(),
            ]);
        }

        // AO12
        for i in 0..self.num_times() {
            self.times["times"][i]["ao12"] = Value::from(self.calc_ao12(i));
        }

        // check if the new time is the best ao12
        if (self.num_times() >= 12)
            && (self.times["times"][self.num_times() - 1]["ao12"]
                .as_f64()
                .unwrap()
                < self.times["bests"]["ao12"]["time"].as_f64().unwrap()
                || self.times["bests"]["ao12"]["time"].as_f64().unwrap() == 0.0)
        {
            self.times["bests"]["ao12"]["time"] =
                self.times["times"][self.num_times() - 1]["ao12"].clone();
            // use a loop to make a vector of the last 12 times
            let mut last_12_best: Vec<Value> = vec![];
            for i in self.num_times() - 12..self.num_times() {
                last_12_best.push(self.times["times"][i].clone());
            }
            self.times["bests"]["ao12"]["times"] = Value::from(last_12_best);
        }
    }

    pub fn calc_ao5(&self, index: usize) -> f64 {
        // calculate ao5 for a given index
        if index < 4 {
            return 0.0;
        }

        // Calculate ao5 by cutting out best and worst times and averaging the rest
        // count number of DNFs
        let mut dnf_count = 0;

        let mut times = vec![];
        for i in index - 4..index + 1 {
            times.push(self.times["times"][i]["time"].as_f64().unwrap());
            // check for dnf
            if self.times["times"][i]["penalty"].as_str().unwrap() == "DNF" {
                dnf_count += 1;
            }
        }

        // if more than 1 dnf, ao5 is dnf
        match dnf_count {
            0 => {
                // no dnf, sort and remove best and worst
                times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                times.remove(0);
                times.pop();
            }
            1 => {
                // one dnf remove the best time
                times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                times.remove(0);

                // find the dnf and remove it
                for i in 0..times.len() {
                    if self.times["times"][i]["penalty"].as_str().unwrap() == "DNF" {
                        times.remove(i);
                        break;
                    }
                }
            }
            _ => {
                // more than one dnf, ao5 is dnf
                return -1.0;
            }
        }

        let mut sum = 0.0;
        for time in times {
            sum += time;
        }
        sum / 3.0
    }

    pub fn calc_ao12(&self, index: usize) -> f64 {
        // calculate ao12 for a given index
        if index < 11 {
            return 0.0;
        }

        let mut dnf_count = 0;

        // Calculate ao12 by cutting out best and worst times and averaging the rest
        let mut times = vec![];
        for i in index - 11..index + 1 {
            times.push(self.times["times"][i]["time"].as_f64().unwrap());
            // check for dnf
            if self.times["times"][i]["penalty"].as_str().unwrap() == "DNF" {
                dnf_count += 1;
            }
        }

        match dnf_count {
            0 => {
                // no dnf, sort and remove best and worst
                times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                times.remove(0);
                times.pop();
            }
            1 => {
                // one dnf remove the best time
                times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                times.remove(0);

                // find the dnf and remove it
                for i in 0..times.len() {
                    if self.times["times"][i]["penalty"].as_str().unwrap() == "DNF" {
                        times.remove(i);
                        break;
                    }
                }
            }
            _ => {
                // more than one dnf, ao12 is dnf
                return -1.0;
            }
        }

        let mut sum = 0.0;
        for time in times {
            sum += time;
        }
        sum / 10.0
    }

    pub fn reset(&mut self) {
        // reset times
        self.times["times"].as_array_mut().unwrap().clear();
        self.reset_bests();
    }

    pub fn reset_bests(&mut self) {
        self.times["bests"]["single"]["time"] = Value::from(0.0);
        self.times["bests"]["single"]["scramble"] = Value::from("");
        self.times["bests"]["ao5"]["time"] = Value::from(0.0);
        self.times["bests"]["ao5"]["times"]
            .as_array_mut()
            .unwrap()
            .clear();
        self.times["bests"]["ao12"]["time"] = Value::from(0.0);
        self.times["bests"]["ao12"]["times"]
            .as_array_mut()
            .unwrap()
            .clear();
    }

    pub fn currents(&self) -> Vec<String> {
        let mut currents = vec![];

        if self.num_times() == 0 {
            currents.push("NA".to_string());
            currents.push("NA".to_string());
            currents.push("NA".to_string());
            return currents;
        }

        if self.times["times"][self.num_times() - 1]["penalty"]
            .as_str()
            .unwrap()
            == "DNF"
        {
            currents.push("DNF".to_string());
        } else {
            currents.push(format!(
                "{:.3}",
                self.times["times"][self.num_times() - 1]["time"]
                    .as_f64()
                    .unwrap()
            ));
        }

        if self.num_times() >= 5 {
            currents.push(format!(
                "{:.3}",
                self.times["times"][self.num_times() - 1]["ao5"]
                    .as_f64()
                    .unwrap()
            ));
        } else {
            currents.push("NA".to_string());
        }

        if self.num_times() >= 12 {
            currents.push(format!(
                "{:.3}",
                self.times["times"][self.num_times() - 1]["ao12"]
                    .as_f64()
                    .unwrap()
            ));
        } else {
            currents.push("NA".to_string());
        }
        currents
    }

    pub fn bests(&self) -> Vec<String> {
        if self.num_times() == 0 {
            return vec!["NA".to_string(), "NA".to_string(), "NA".to_string()];
        }

        let mut bests = vec![];

        bests.push(format!(
            "{:.3}",
            self.times["bests"]["single"]["time"].as_f64().unwrap()
        ));

        if self.num_times() >= 5 {
            bests.push(format!(
                "{:.3}",
                self.times["bests"]["ao5"]["time"].as_f64().unwrap()
            ));
        } else {
            bests.push("NA".to_string());
        }

        if self.num_times() >= 12 {
            bests.push(format!(
                "{:.3}",
                self.times["bests"]["ao12"]["time"].as_f64().unwrap()
            ));
        } else {
            bests.push("NA".to_string());
        }

        bests
    }

    pub fn toggle_penalty(&mut self, penalty: Penalty) {
        // toggle penalty for the last time
        if self.num_times() == 0 {
            return;
        }

        let index = self.num_times() - 1;

        match penalty {
            Penalty::DNF => match self.times["times"][index]["penalty"].as_str().unwrap() {
                "DNF" => {
                    self.times["times"][index]["penalty"] = Value::from("");
                }
                "+2" => {
                    self.times["times"][index]["penalty"] = Value::from("DNF");
                    self.times["times"][index]["time"] =
                        Value::from(self.times["times"][index]["time"].as_f64().unwrap() - 2.0);
                }
                _ => {
                    self.times["times"][index]["penalty"] = Value::from("DNF");
                }
            },
            Penalty::PlusTwo => match self.times["times"][index]["penalty"].as_str().unwrap() {
                "+2" => {
                    self.times["times"][index]["penalty"] = Value::from("");
                    self.times["times"][index]["time"] =
                        Value::from(self.times["times"][index]["time"].as_f64().unwrap() - 2.0);
                }
                "DNF" => {
                    self.times["times"][index]["penalty"] = Value::from("+2");
                    self.times["times"][index]["time"] =
                        Value::from(self.times["times"][index]["time"].as_f64().unwrap() + 2.0);
                }
                _ => {
                    self.times["times"][index]["penalty"] = Value::from("+2");
                    self.times["times"][index]["time"] =
                        Value::from(self.times["times"][index]["time"].as_f64().unwrap() + 2.0);
                }
            },
            Penalty::None => {}
        }

        // recalculate all a05 and ao12
        for i in 0..self.num_times() {
            self.times["times"][i]["ao5"] = Value::from(self.calc_ao5(i));
            self.times["times"][i]["ao12"] = Value::from(self.calc_ao12(i));
        }
    }

    pub fn display_time(&self, time: f64) -> String {
        let time_string;

        if time == -1.0 {
            time_string = "DNF".to_string();
        } else if time == 0.0 {
            time_string = "NA".to_string();
        } else {
            time_string = format!("{:.3}", time);
        }

        return time_string;
    }
}
