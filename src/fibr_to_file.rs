use crate::pacient::Pacient;
use crate::fibr::Fibr;
use crate::time_param::TimeParam;
use crate::my_lib::{get_coef_p, get_coef_fibr};
use crate::disp::get_coef_disp;
use std::fs::File;
use std::io::BufWriter;
// use std::fs::OpenOptions;
use std::io::Write;



pub fn fibr_to_f_txt() {
    let time_param = TimeParam::new();

    let trs = &time_param.threshold;

    let r_pos = &time_param.r_pos;

    let coef_p = get_coef_p(&time_param);

    let coef_disp = get_coef_disp(&time_param);

    let coef_fibr = get_coef_fibr(&coef_p, &coef_disp, &time_param);

    let mask: Vec<usize> = vec![1; coef_fibr.len()];
    let pacient = Pacient::new();
    let start_time_in_samples = pacient.start_time_in_samples;
    let fibr = Fibr::new(&coef_fibr, &trs, &r_pos, start_time_in_samples, &mask);

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

    let file = File::create("c:\\EcgVar\\F.txt").expect("Не удалось создать файл");
    let mut writer = BufWriter::new(file);
    let text = text.as_bytes();
    writer.write_all(text).expect("Не удалось записать в файл");
}