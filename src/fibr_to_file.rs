use crate::pacient::Pacient;
use crate::fibr::Fibr;
use crate::time_param::TimeParam;
use crate::my_lib::{get_coef_p, get_coef_fibr};
use crate::disp::get_coef_disp;
use std::fs::File;
use std::io::BufWriter;
// use std::fs::OpenOptions;
use std::io::Write;
use ndarray_npy::write_npy;
use ndarray::{Array, Array1};


pub fn fibr_to_f_txt() {
    let time_param = TimeParam::new();

    // let clear_intervals = time_param.clear_intervals.clone();

    // let data_i64: Vec<i64> = clear_intervals.iter().map(|&x| x as i64).collect();
    // let array: Array1<i64> = Array::from_vec(data_i64);
    // write_npy("clear_intervals.npy", &array);

    let trs = time_param.threshold.clone();

    let data_i64: Vec<i64> = trs.iter().map(|&x| x as i64).collect();
    let array: Array1<i64> = Array::from_vec(data_i64);
    let _ = write_npy("trs.npy", &array);

    let r_pos = &time_param.r_pos;
    let coef_p = get_coef_p(&time_param);

    let data_f64: Vec<f64> = coef_p.iter().map(|&x| x as f64).collect();
    let array: Array1<f64> = Array::from_vec(data_f64);
    let _ = write_npy("coef_p.npy", &array);

    let coef_disp = get_coef_disp(&time_param);

    let data_f64: Vec<f64> = coef_disp.iter().map(|&x| x as f64).collect();
    let array: Array1<f64> = Array::from_vec(data_f64);
    let _ = write_npy("coef_disp.npy", &array);

    let coef_fibr = get_coef_fibr(&coef_p, &coef_disp, &time_param);

    let data_f64: Vec<f64> = coef_fibr.iter().map(|&x| x as f64).collect();
    let array: Array1<f64> = Array::from_vec(data_f64);
    let _ = write_npy("coef_fibr.npy", &array);

    let mask: Vec<usize> = vec![1; coef_fibr.len()];
    let pacient = Pacient::new();
    let start_time_in_samples = pacient.start_time_in_samples;
    let fibr = Fibr::new(&coef_fibr, &trs, &r_pos, start_time_in_samples, &mask);
    println!("{:?}", fibr);

    let mut text = pacient.file_name.clone();
    text.push_str("\n\n");
    let (h, m, s) = pacient.start_time.clone();
    let start_time = format!("Начало записи {}:{}:{}\n\n", h, m, s);
    text.push_str(&start_time);

    for i in 0..fibr.start_ind_arr.len() {
        let (d1, h1, m1, s1) = pacient.get_time_from_samples(fibr.start_ind_arr[i] as u32);
        let (d2, h2, m2, s2) = pacient.get_time_from_samples(fibr.stop_ind_arr[i] as u32);
        let (d3, h3, m3, s3) = pacient.get_time_from_samples(fibr.diff_stop_start[i] as u32);
        let h3 = d3 * 24 + h3;
        let episod = format!("с {}д {}:{}:{} по {}д {}:{}:{}  (длит. эпизода {}:{}:{})\n", d1 + 1, h1, m1, s1, d2 + 1, h2, m2, s2, h3, m3, s3);
        text.push_str(&episod);
    }

    let num_of_episodes = fibr.start_ind_arr.len();
    let num_of_samples_in_fibr: usize = fibr.diff_stop_start.iter().sum();
    let (d, h, m, s) = pacient.get_time_from_samples(num_of_samples_in_fibr as u32);
    let h = d * 24 + h;
    let sum_time_fibr = format!("\nВсего эпизодов фибриляции: {}\nОбщее время эпизодов: {}:{}:{}\n\n", num_of_episodes, h, m, s);
    text.push_str(&sum_time_fibr);

    let total_time = format!("Общее время записи: {}:{}:{}", pacient.total_time.0, pacient.total_time.1, pacient.total_time.2);
    text.push_str(&total_time);

    let file = File::create("F.txt").expect("Не удалось создать файл");
    let mut writer = BufWriter::new(file);
    let text = text.as_bytes();
    writer.write_all(text).expect("Не удалось записать в файл");
}