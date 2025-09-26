use crate::my_lib::get_coef_cor;
use crate::my_lib::{step_moving_average, moving_average};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct TimeParam {
    pub r_pos: Vec<f32>,
    pub intervals: Vec<f32>,
    pub inds_min_diff: Vec<usize>,
    pub clear_intervals: Vec<f32>,
    pub threshold: Vec<f32>,
    pub chars: Vec<char>,
}

impl TimeParam {
    pub fn new() -> TimeParam {
        let mut time_param = TimeParam {
            r_pos: vec![],
            intervals: vec![],
            inds_min_diff: vec![],
            clear_intervals: vec![],
            threshold: vec![],
            chars: vec![],
        };
        time_param.parse_b_txt();
        time_param.get_inds_min_diff();
        time_param.get_clear_intervals();
        time_param.get_threshold();
        time_param
    }

    fn parse_b_txt(&mut self) {
        let path_b = "B.txt";
        let file = File::open(&path_b).unwrap();
        let reader = BufReader::new(&file);
        // let mut line = String::new();
        // for res_line in reader.lines() {
        for (_i, res_line) in reader.lines().enumerate() {
            let line: String = match res_line {
                Ok(val) => val,
                Err(_err) => continue,
            };
            if line.contains(';') {
                let split_line: Vec<&str> = line.split(';').collect();
                if split_line.len() == 3 {
                    let end_line: Vec<&str> = split_line[2].split(':').collect();
                    if end_line.len() == 2 {
                        self.r_pos.push(split_line[0].parse::<f32>().unwrap());
                        self.intervals.push(split_line[1].parse::<f32>().unwrap());
                        let char_in_line = end_line[0].chars().nth(0);
                        let char_end = char_in_line.unwrap_or_else(|| "A".chars().next().unwrap());
                        // if char_end == 'A' {
                        //     println!("{} {:?}", i, end_line);
                        // }
                        self.chars.push(char_end);
                    }
                }
            }
        }
    }

    fn get_clear_intervals(&mut self) {
        // let mut max_diff: f32 = 0.0;
        // let mut mean_intervals: f32 = 0.0;
        let trs = 0.99;
        let mut step: usize = 1;

        for i in 0..self.chars.len() {
            if step == 2 {
                step = 1;
                continue;
            }
            if (i > 3) && (i < self.intervals.len() - 3) {
                let diff0 = &self.intervals[i] - &self.intervals[i - 1];
                let diff1 = &self.intervals[i + 1] - &self.intervals[i];

                let max_diff = if diff0 >= diff1 {
                    diff0
                } else {
                    diff1
                };
                let mean_intervals = (&self.intervals[i - 3]
                    + &self.intervals[i - 2]
                    + &self.intervals[i - 1]
                    + &self.intervals[i + 2]
                    + &self.intervals[i + 3])
                    / 5.0;
                if (self.chars[i] == 'V') && (self.chars[i + 1] == 'V') && (max_diff > 100.0) {
                    // max_diff > 100
                    self.clear_intervals
                        .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                    step = 1;
                } else if (self.chars[i] == 'V') && (self.chars[i + 1] != 'V') && (max_diff > 40.0)
                {
                    // max_diff > 40
                    self.clear_intervals
                        .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                    self.clear_intervals
                        .push(mean_intervals + ((&self.intervals[i + 1] - mean_intervals) * 0.2));
                    step = 2;
                } else if (self.chars[i] != 'V') && (max_diff > 100.0) {
                    //  else if self.chars[i] == 'N'
                    let tf: &Vec<f32> = &self.intervals[i - 1..i + 3].to_vec();
                    // let tf: Vec<f32> = tf.iter().map(|x| *x as f32).collect();
                    let ref_t: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0] * 1.3, tf[0]];
                    let ref_t0: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0], tf[0] * 0.6];
                    let ref_t1: Vec<f32> = vec![tf[0], tf[0] * 1.35, tf[0], tf[0] * 1.35];

                    let coef_cor = get_coef_cor(&ref_t, &tf);
                    let coef_cor0 = get_coef_cor(&ref_t0, &tf);
                    let coef_cor1 = get_coef_cor(&ref_t1, &tf);
                    let vec_cor: Vec<f32> = vec![coef_cor, coef_cor0, coef_cor1];
                    let max_cor = vec_cor.iter().fold(f32::MIN, |acc, &x| acc.max(x));
                    let diff21 = tf[2] + tf[0] - 2.0 * tf[1];
                    if (max_cor > trs) || (diff21 > 160.0) {
                        self.clear_intervals
                            .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                        self.clear_intervals.push(
                            mean_intervals + ((&self.intervals[i + 1] - mean_intervals) * 0.2),
                        );
                        step = 2;
                    } else {
                        self.clear_intervals
                            .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                        step = 1;
                    }
                } else {
                    self.clear_intervals.push(self.intervals[i]);
                    step = 1;
                }
            } else {
                self.clear_intervals.push(self.intervals[i]);
                step = 1;
            }
        }
    }

    fn get_threshold(&mut self) {
        self.threshold = step_moving_average(&self.clear_intervals, 2);
        self.threshold = step_moving_average(&self.threshold, 3);
        self.threshold = step_moving_average(&self.threshold, 2);
        self.threshold = moving_average(&self.threshold, 12);
    }

    fn get_inds_min_diff(&mut self) {
        /*
        Возвращает индексы интервалов, где абсолютная разница между
        последовательными интервалами меньше или равна 3.
        Параметры:
        intervals: Vec<f32>.
        Возвращает:
        массив индексов Vec<usize>, где абсолютная разница
        между последовательными интервалами меньше или равна 3.
        */
        for i in 1..self.intervals.len() - 1 {
            if (self.chars[i] == 'N') && (self.chars[i - 1] == 'N') && (self.chars[i + 1] == 'N') {
                let diff = (self.intervals[i] - self.intervals[i + 1]).abs();
                if diff <= 3.0 {
                    self.inds_min_diff.push(i);
                }
            }
        }
    }
}
