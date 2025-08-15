use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Div;
use ndarray::Array1;


pub struct Ecg {
    pub r_pos: Vec<i32>,
    pub intervals: Vec<i32>,
    pub fintervals: Vec<i32>,
    pub clean_intervals: Vec<i32>,
    pub chars: Vec<char>,
}

impl Ecg {
    pub fn new() -> Ecg {
        Ecg {
            r_pos: vec![],
            intervals: vec![],
            fintervals: vec![],
            clean_intervals: vec![],
            chars: vec![],
        }
    }

    pub fn parse_B_txt(&mut self) {
        let path_b = "C:\\EcgVar\\B.txt";
        let file = File::open(&path_b).unwrap();
        let reader = BufReader::new(&file);
        let mut line = String::new();
        // for res_line in reader.lines() {
        for (i, res_line) in reader.lines().enumerate() {
            line = match res_line {
                Ok(val) => val,
                Err(_err) => continue,
            };
            if line.contains(';') {
                let split_line: Vec<&str> = line.split(';').collect();
                if split_line.len() == 3 {
                    let end_line: Vec<&str> = split_line[2].split(':').collect();
                    if end_line.len() == 2 {
                        self.r_pos.push(split_line[0].parse::<i32>().unwrap());
                        self.intervals.push(split_line[1].parse::<i32>().unwrap());
                        let char_in_line = end_line[0].chars().nth(0);
                        let char_end = char_in_line.unwrap_or_else(|| "A".chars().next().unwrap());
                        if char_end == 'A' {
                            println!("{} {:?}", i, end_line);
                        }
                        self.chars.push(char_end);
                    }
                }
            }
        }
    }

    pub fn get_fintervals(&mut self) {
        let mut max_diff = 0;
        let mut mean_intervals = 0;
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

                if diff0 >= diff1 {
                    max_diff = diff0;
                } else {
                    max_diff = diff1;
                }
                mean_intervals = (&self.intervals[i - 3]
                    + &self.intervals[i - 2]
                    + &self.intervals[i - 1]
                    + &self.intervals[i + 2]
                    + &self.intervals[i + 3])
                    / 5;
                if (self.chars[i] == 'V') && (self.chars[i + 1] == 'V') && (max_diff > 10000) {   // max_diff > 100
                    self.fintervals.push(
                        mean_intervals
                            + ((&self.intervals[i] - mean_intervals) as f64 * 0.2) as i32,
                    );
                    step = 1;
                } else if (self.chars[i] == 'V') && (self.chars[i + 1] != 'V') && (max_diff > 4000) {  // max_diff > 40
                    self.fintervals.push(
                        mean_intervals
                            + ((&self.intervals[i] - mean_intervals) as f64 * 0.2) as i32,
                    );
                    self.fintervals.push(
                        mean_intervals
                            + ((&self.intervals[i + 1] - mean_intervals) as f64 * 0.2) as i32,
                    );
                    step = 2;
                } else if self.chars[i] == 'V' {    //  else if self.chars[i] == 'N'
                    let tf: &Vec<i32> = &self.intervals[i - 1..i + 3].to_vec();
                    let tf: Vec<f32> = tf.iter().map(|x| *x as f32).collect();
                    let ref_t: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0] * 1.3, tf[0],];
                    let ref_t0: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0], tf[0] * 0.6,];
                    let ref_t1: Vec<f32> = vec![tf[0], tf[0] * 1.35, tf[0], tf[0] * 1.35,];

                    let coef_cor = get_coef_cor(&ref_t, &tf);
                    let coef_cor0 = get_coef_cor(&ref_t0, &tf);
                    let coef_cor1 = get_coef_cor(&ref_t1, &tf);
                    let vec_cor : Vec<f32> = vec![coef_cor, coef_cor0, coef_cor1];
                    let max_cor = vec_cor.iter().fold(f32::MIN, |acc, &x| acc.max(x));
                    let diff21 = tf[2] + tf[0] - 2.0 * tf[1];
                    if (max_cor > trs) || (diff21 > 160.0) {
                        self.fintervals.push(
                            mean_intervals
                                + ((&self.intervals[i] - mean_intervals) as f64 * 0.2) as i32,
                        );
                        self.fintervals.push(
                            mean_intervals
                                + ((&self.intervals[i + 1] - mean_intervals) as f64 * 0.2) as i32,
                        );
                        step = 2;
                    } else {
                        self.fintervals.push(
                            mean_intervals
                                + ((&self.intervals[i] - mean_intervals) as f64 * 0.2) as i32,
                        );
                        step = 1;
                    }

                } else {
                    self.fintervals.push(self.intervals[i]);
                    step = 1;
                }
            } else {
                self.fintervals.push(self.intervals[i]);
                step = 1;
            }
        }
    }
}

pub fn get_diff_intervals(intervals: &Vec<i32>, step_diff: usize) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let len = intervals.len();

    if step_diff >= len {
        return out;
    }

    for i in step_diff..len {
        let temp = (intervals[i] - intervals[i - step_diff]).abs();
        out.push(temp);
    }
    out[0] = out[1];
    let val = out[0];
    for i in 0..step_diff {
        out.insert(0, val);
    }
    out
}

pub fn get_coef_cor(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    let arr_x:Array1<f32> = Array1::from_vec(x.clone());
    let arr_y: Array1<f32> = Array1::from_vec(y.clone());
    let mean_x = &arr_x.mean().unwrap();
    let mean_y = &arr_y.mean().unwrap();
    let arr_xy = &arr_x * &arr_y;
    let mean_xy = Array1::from(arr_xy).mean().unwrap();
    let std_x = &arr_x.std(0.0);
    let std_y = &arr_y.std(0.0);
    let std_xy = std_x * std_y;
    if std_xy != 0.0 {
        (mean_xy - mean_x * mean_y) / std_xy
    } else { 0.0 }
}

// pub fn step_moving_average_i32(data: &Vec<i32>, window_size: usize) -> Vec<i32> {
//     let mut out: Vec<i32> = vec![];
//     let hf_w_size = window_size / 2;
//     for i in (0..data.len() - hf_w_size).step_by(hf_w_size) {
//         let buff = &data[i..i + hf_w_size];
//         let mean_buff = buff.iter().sum::<i32>() / buff.len() as i32;
//         for i in 0..hf_w_size {
//             out.push(mean_buff);
//         }
//     }
//     let len_data = data.len();
//     let len_out = out.len();
//     let diff_len = len_data - len_out;
//     let last_out = *out.last().unwrap();
//     for i in 0..diff_len {
//         out.push(last_out);
//     }
//     out
// }

pub fn step_moving_average_i32(data: &Vec<i32>, window_size: usize) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    // let hf_w_size = window_size / 2;
    for i in (0..data.len() - window_size).step_by(window_size) {
        let buff = &data[i..i + window_size];
        let mean_buff = buff.iter().sum::<i32>() / buff.len() as i32;
        for i in 0..window_size {
            out.push(mean_buff);
        }
    }
    let len_data = data.len();
    let len_out = out.len();
    let diff_len = len_data - len_out;
    let last_out = *out.last().unwrap();
    for i in 0..diff_len {
        out.push(last_out);
    }
    out
}