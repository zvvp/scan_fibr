use crate::my_lib::get_coef_cor;
use crate::my_lib::{step_moving_average, moving_average};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use native_dialog::{DialogBuilder, MessageLevel};

pub struct TimeParam {
    pub r_pos: Vec<i32>,
    pub intervals: Vec<f32>,
    pub inds_min_diff: Vec<usize>,
    pub clear_intervals: Vec<f32>,
    pub threshold: Vec<f32>,
    pub chars: Vec<char>,
    pub forms: Vec<usize>
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
            forms: vec![],
        };
        time_param.parse_b_txt();
        time_param.get_inds_min_diff();
        time_param.get_clear_intervals();
        time_param.get_threshold();
        time_param
    }

    fn parse_b_txt(&mut self) {
        let path_b = "c:\\EcgVar\\B.txt";
        let file = File::open(&path_b);
        let file = match file {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    DialogBuilder::message()
                        .set_level(MessageLevel::Error)
                        .set_title("Ошибка")
                        .set_text("В директории c:\\EcgVar\\ отсутствует файл B.txt ")
                        .alert()
                        .show()
                        .unwrap();
                    return;
                },
                other_error => panic!("{:?}", other_error),
            },
        };
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
                        self.r_pos.push(split_line[0].parse::<i32>().unwrap());
                        let mut interval = split_line[1].parse::<f32>().unwrap();
                        if interval > 450.0 {
                            interval = 200.0;
                        }
                        self.intervals.push(interval);
                        self.forms.push(end_line[1].parse::<usize>().unwrap());
                        let char_in_line = end_line[0].chars().nth(0);
                        let char_end = char_in_line.unwrap_or_else(|| "A".chars().next().unwrap());
                        self.chars.push(char_end);
                    }
                }
            }
        }
    }

    fn get_clear_intervals(&mut self) {
        // let mut max_diff: f32 = 0.0;
        // let mut mean_intervals: f32 = 0.0;
        // let trs = 0.99; // 0.99
        let mut intervals = self.intervals.clone();
        let mut step: usize = 1;
        for i in 0..self.chars.len() {
            if step == 2 {
                step = 1;
                continue;
            }
            if (i > 3) && (i < intervals.len() - 4) {
                // let diff0 = (&self.intervals[i] - &self.intervals[i - 1]).abs();
                // let diff1 = (&self.intervals[i + 1] - &self.intervals[i]).abs();
                // let max_diff = &self.intervals[i + 1] - &self.intervals[i];
                // let max_diff = if diff0 >= diff1 {
                //     diff0
                // } else {
                //     diff1
                // };
                let mean_intervals = (intervals[i - 3]
                    + intervals[i - 2]
                    + intervals[i - 1]
                    + intervals[i + 2]
                    + intervals[i + 3])
                    / 5.0;
                let mut diff_mean = intervals[i] - mean_intervals;
                if diff_mean == 0.0 {
                    diff_mean = 0.1;
                }
                let sign_diff_mean = diff_mean / diff_mean.abs();
                let dev_mean = sign_diff_mean * (diff_mean.abs().sqrt()) * 0.4;
                if (self.chars[i] == 'V') && (self.chars[i + 1] == 'V') {//&& (max_diff > 100.0) {
                    self.clear_intervals.push(mean_intervals + dev_mean);  // * 0.2
                    intervals[i] = mean_intervals + dev_mean;
                    step = 1;
                } else if (self.chars[i] == 'V') && (self.chars[i + 1] != 'V') {//&& (max_diff > 40.0) {
                    // max_diff > 40
                    self.clear_intervals.push(mean_intervals + dev_mean);  // * 0.2
                    self.clear_intervals.push(mean_intervals - dev_mean);  // * 0.2
                    intervals[i] = mean_intervals + dev_mean;
                    intervals[i + 1] = mean_intervals - dev_mean;
                    step = 2;
                } else if (self.chars[i] != 'V') && (self.forms[i] == 1) {     //self.chars[i] != 'V'
                    let tf = intervals[i - 1..i + 4].to_vec();
                    let sum_interval = intervals[i] + intervals[i + 1];
                    let half_sum = sum_interval / 2.0;
                    let diff_intervals = (intervals[i - 1] - half_sum).abs();
                    let diff21 = tf[2] - tf[1];
                    let diff23 = tf[2] - tf[3];
                    let diff02 = (tf[0] - tf[2]).abs();
                    let diff24 = (tf[2] - tf[4]).abs();
                    let diff13 = (tf[1] - tf[3]).abs();

                    // if i == 1710 {
                    //     println!("i = {}", i);
                    //     println!("{} {} {} {}", tf[0], tf[1], tf[2], tf[3]);
                    //     println!("sum_interval = {}", sum_interval);
                    //     println!("half_sum = {}", half_sum);
                    //     println!("diff_intervals = {}", diff_intervals);
                    //     println!("diff21 = {}", diff21);
                    //     println!("diff23 = {}", diff23);
                    // }
                    if (tf[1] < intervals[i-1]) && (tf[2] > intervals[i-1]) && (diff21 > 65.0) && (diff23 > 10.0) {//&& (diff_intervals < diff21 * 0.3) {
                        self.clear_intervals.push(mean_intervals + dev_mean);
                        self.clear_intervals.push(mean_intervals - dev_mean);
                        intervals[i] = mean_intervals + dev_mean;
                        intervals[i + 1] = mean_intervals - dev_mean;
                        step = 2;
                    } else if sum_interval < intervals[i - 1] * 1.1 {
                        self.clear_intervals.push(mean_intervals + dev_mean);
                        self.clear_intervals.push(mean_intervals + dev_mean);
                        intervals[i] = mean_intervals + dev_mean;
                        intervals[i + 1] = mean_intervals + dev_mean;
                        step = 2;
                    // } else if (diff02 < 10.0) && (diff24 < 10.0) && (diff13 < 10.0) && (diff21 > 50.0) {
                    //     self.clear_intervals.push(mean_intervals + dev_mean);
                    //     self.clear_intervals.push(mean_intervals + dev_mean);
                    //     step = 2;
                    } else if (diff24 < 10.0) && (diff13 < 10.0) && (diff21 > 70.0) {
                        self.clear_intervals.push(mean_intervals + dev_mean);
                        self.clear_intervals.push(mean_intervals - dev_mean);
                        intervals[i] = mean_intervals + dev_mean;
                        intervals[i + 1] = mean_intervals - dev_mean;
                        step = 2;
                    } else {
                        self.clear_intervals.push(intervals[i]);
                        step = 1;
                    }

                    // let ref_t: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0] * 1.3, tf[0]];
                    // let ref_t0: Vec<f32> = vec![tf[0], tf[0] * 0.6, tf[0], tf[0] * 0.6];
                    // let ref_t1: Vec<f32> = vec![tf[0], tf[0] * 1.35, tf[0], tf[0] * 1.35];
                    // let coef_cor = get_coef_cor(&ref_t, &tf);
                    // let coef_cor0 = get_coef_cor(&ref_t0, &tf);
                    // let coef_cor1 = get_coef_cor(&ref_t1, &tf);
                    // let vec_cor: Vec<f32> = vec![coef_cor, coef_cor0, coef_cor1];
                    // let max_cor = vec_cor.iter().fold(f32::MIN, |acc, &x| acc.max(x));
                    // let diff21 = tf[2] + tf[0] - 2.0 * tf[1];
                    // if (max_cor > trs) || (diff21 > 160.0) {
                    //     self.clear_intervals
                    //         .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                    //     self.clear_intervals.push(
                    //         mean_intervals + ((&self.intervals[i + 1] - mean_intervals) * 0.2),
                    //     );
                    //     step = 2;
                    // } else {
                    //     self.clear_intervals
                    //         .push(mean_intervals + ((&self.intervals[i] - mean_intervals) * 0.2));
                    //     step = 1;
                    // }
                } else {
                    self.clear_intervals.push(intervals[i]);
                    step = 1;
                }
            } else {
                self.clear_intervals.push(intervals[i]);
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
        Параметры:
        intervals: Vec<f32>.
        Возвращает:
        массив индексов Vec<usize>, где абсолютная разница
        между последовательными интервалами меньше или равна 3.
        */
        for i in 1..self.intervals.len() {
            let min_diff = &self.intervals[i] * 0.02;
            if (self.chars[i] == 'N') && (self.chars[i - 1] == 'N')  && (self.forms[i] != 0) && (self.forms[i] == self.forms[i - 1]) {
                let diff = (self.intervals[i] - self.intervals[i - 1]).abs();
                if diff <= min_diff {
                    self.inds_min_diff.push(i);
                }
            }
        }
    }
}
