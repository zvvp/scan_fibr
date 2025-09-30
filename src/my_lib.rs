use native_dialog::{DialogBuilder, MessageLevel};
use ndarray::Array1;
use ndarray_npy::read_npy;
use crate::time_param::TimeParam;
use crate::zub_p::Zubp;

pub struct Lead {
    pub lead: Vec<f32>,
}
impl Lead {
    pub fn new(num: u8) -> Lead {
        let files = glob::glob("clean_lead*.npy").expect("Failed to read files");
        let arr = if files.count() == 3 {
            let current_dir = std::env::current_dir().unwrap();
            let arr: Array1<f32> = match num {
                1 => read_npy(current_dir.join("clean_lead1.npy")).unwrap(),
                2 => read_npy(current_dir.join("clean_lead2.npy")).unwrap(),
                3 => read_npy(current_dir.join("clean_lead3.npy")).unwrap(),
                _ => panic!(),
            };
            arr
        } else {
            DialogBuilder::message()
                .set_level(MessageLevel::Error)
                .set_title("Ошибка")
                .set_text("В текущей директории не найдены файлы: \nclean_lead1.npy \nclean_lead2.npy \nclean_lead3.npy")
                .alert()
                .show()
                .unwrap();
            let vec:Vec<f32> = vec![];
            let arr = Array1::from_vec(vec);
            arr
        };

        Lead {
            lead: arr.to_vec()
        }
    }
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

pub fn step_moving_average(data: &Vec<f32>, window_size: usize) -> Vec<f32> {
    let mut out: Vec<f32> = vec![];
    for i in (0..data.len() - window_size).step_by(window_size) {
        let buff = &data[i..i + window_size];
        let mean_buff = buff.iter().sum::<f32>() / buff.len() as f32;
        for _i in 0..window_size {
            out.push(mean_buff);
        }
    }
    let len_data = data.len();
    let len_out = out.len();
    let diff_len = len_data - len_out;
    let last_out = *out.last().unwrap();
    for _i in 0..diff_len {
        out.push(last_out);
    }
    out
}

pub fn moving_average(data: &Vec<f32>, window_size: usize) -> Vec<f32> {
    let half_win = window_size / 2;
    let begin = &data[0..window_size];
    let mean_begin = begin.iter().sum::<f32>() / begin.len() as f32;
    let mut out: Vec<f32> = vec![mean_begin; half_win];
    for i in half_win..data.len() - half_win {
        let buff = &data[i - half_win..i + half_win];
        let mean_buff = buff.iter().sum::<f32>() / buff.len() as f32;
        out.push(mean_buff);
    }
    let last_out = *out.last().unwrap();
    for _i in 0..half_win {
        out.push(last_out);
    }
    out
}

pub fn truncate_win2(ch: &Vec<f32>, k: f32, win_size: usize) -> Vec<f32> {
    // let in_ch = ch.clone();
    let mut out = ch.clone();
    let half_win = win_size / 2;

    for i in ((half_win)..(&out.len() - half_win)).step_by(half_win / 2) {
        let slice_start = i - half_win;
        let slice_end = i + half_win;
        let buff = &out[slice_start..slice_end].to_vec();

        let mean_buff: f32 = buff.iter().sum::<f32>() / (buff.len() as f32);
        let over_buff: Vec<f32> = buff.iter().filter(|x| **x > mean_buff).cloned().collect();
        if over_buff.len() > 0 {
            let over_mean: f32 = over_buff.iter().sum::<f32>() / (over_buff.len() as f32);
            for j in slice_start..slice_end {
                if out[j] > over_mean {
                    out[j] = (out[j] - over_mean) * k + over_mean;
                }
            }
        }
        let under_buff: Vec<f32> = buff.iter().filter(|x| **x < mean_buff).cloned().collect();
        if under_buff.len() > 0 {
            let under_mean: f32 = under_buff.iter().sum::<f32>() / (under_buff.len() as f32);
            for j in slice_start..slice_end {
                if out[j] < under_mean {
                    out[j] = (out[j] - under_mean) * k + under_mean;
                }
            }
        }
    }
    // Заполнение краевых участков
    for i in 0..half_win {
        out[i] = out[half_win];
    }
    for i in (out.len() - half_win)..out.len() {
        out[i] = out[out.len() - half_win];
    }
    out
}

pub fn my_filtfilt(b: &Vec<f32>, a: &Vec<f32>, ch: &Vec<f32>) -> Vec<f32> {
    let mut temp = ch.to_owned();
    let mut out = ch.to_owned();
    let len_b = b.len();
    let len_a = a.len();
    let len_ch = ch.len();

    for i in len_b - 1..len_ch {
        temp[i] = b[0] * ch[i];
        for j in 1..len_b {
            temp[i] += b[j] * ch[i - j];
        }
        for j in 1..len_a {
            temp[i] -= a[j] * temp[i - j];
        }
    }

    for i in (1..=(len_ch - len_b)).rev() {
        out[i] = b[0] * temp[i];
        for j in 1..len_b {
            out[i] += b[j] * temp[i + j];
        }
        for j in 1..len_a {
            out[i] -= a[j] * out[i + j];
        }
    }
    out
}

pub fn find_local_max(data: &Vec<f32>) -> (Vec<usize>, Vec<f32>) {
    let mut ind_max: Vec<usize> = vec![];
    let mut vec_max: Vec<f32> = vec![];
    for i in 1..data.len() - 1 {

        if data[i] > data[i - 1] && data[i] > data[i + 1] {
            ind_max.push(i);
            vec_max.push(data[i]);
        }
    }
    (ind_max, vec_max)
}

pub struct LocMinMax {
    pub ind_max: Vec<usize>,
    pub ind_min: Vec<usize>,
    pub ind_loc: Vec<usize>,
    pub arr_loc: Vec<f32>,
    pub diff_loc: Vec<f32>,
}

impl LocMinMax {
    pub fn new(data: &Vec<f32>) -> LocMinMax {
        let mut locminmax = LocMinMax {
            ind_max: vec![],
            ind_min: vec![],
            ind_loc: vec![],
            arr_loc: vec![],
            diff_loc: vec![],
        };
        locminmax.arr_loc.push(data[0]);
        for i in 1..data.len() - 1 {
            if data[i] > data[i - 1] && data[i] > data[i + 1] {
                locminmax.ind_max.push(i);
                locminmax.ind_loc.push(i);
                locminmax.arr_loc.push(data[i]);
            } else if data[i] < data[i - 1] && data[i] < data[i + 1] {
                locminmax.ind_min.push(i);
                locminmax.ind_loc.push(i);
                locminmax.arr_loc.push(data[i]);
            }
        }
        locminmax.arr_loc.push(*data.last().unwrap());
        for i in 1..locminmax.arr_loc.len() {
            locminmax.diff_loc.push((locminmax.arr_loc[i] - locminmax.arr_loc[i - 1]).abs());
        }
        locminmax
    }
}
/*
// pub fn find_loc_min_max(data: &Vec<f32>) {
//     let mut ind_max: Vec<usize> = vec![];
//     let mut ind_min: Vec<usize> = vec![];
//     let mut ind_loc: Vec<usize> = vec![];
//     let mut vec_max: Vec<f32> = vec![];
// }
*/
pub fn find_max(ind_max: &Vec<usize>, vec_max: &Vec<f32>) -> (f32, usize) {
    /* Находит в фрагменте индекс максимального локального максимума
    и значение макс. лок. */
    let mut max: f32 = 0.0;
    let mut ind: usize = 0;
    for (i,item) in vec_max.iter().enumerate() {
        if *item > max {
            max = *item;
            ind = i;
        }
    }
    (max, ind_max[ind])
}

pub fn find_min(data: &Vec<f32>) -> f32 {
    /* Находит минимальное значение фрагмента */
    let mut min: f32 = 10.0;
    // let mut indmin: usize = 0;
    for (i,item) in data.iter().enumerate() {
        if *item < min {
            min = *item;
            // indmin = i;
        }
    }
    // (min, indmin)
    min
}

pub fn get_max(data: &Vec<f32>) -> f32 {
    let mut max: f32 = 0.0;
    for (i,item) in data.iter().enumerate() {
        if *item > max {
            max = *item;
        }
    }
    max
}

pub fn get_coef_p(time_param: &TimeParam) -> Vec<f32> {
    let mut zub_p1 = Zubp::new();
    let mut zub_p2 = Zubp::new();
    let mut zub_p3 = Zubp::new();

    let p1 = zub_p1.get_p_in_lead(1, time_param);
    let p2 = zub_p2.get_p_in_lead(2, time_param);
    let p3 = zub_p3.get_p_in_lead(3, time_param);
    // println!("inds_min_diff.len(): {}", time_param.inds_min_diff.len());
    // println!("mean_amp_p1: {}", zub_p1.mean_amp_p);
    // println!("mean_amp_p2: {}", zub_p2.mean_amp_p);
    // println!("mean_amp_p3: {}", zub_p3.mean_amp_p);
    // println!("mean_PR1: {}", zub_p1.mean_pr);
    // println!("mean_PR2: {}", zub_p2.mean_pr);
    // println!("mean_PR3: {}", zub_p3.mean_pr);
    let mut out: Vec<f32> = vec![0.0; time_param.r_pos.len()];
    for i in 4..p1.len() {
        let mut sum_p1 = p1[i - 4] + p1[i - 3] + p1[i - 2] + p1[i - 1] + p1[i];
        let mut sum_p2 = p2[i - 4] + p2[i - 3] + p2[i - 2] + p2[i - 1] + p2[i];
        let mut sum_p3 = p3[i - 4] + p3[i - 3] + p3[i - 2] + p3[i - 1] + p3[i];
        // if sum_p1 == 0.0 {
        //     sum_p1 = (sum_p2 + sum_p3) / 2.0;
        // } else if sum_p2 == 0.0 {
        //     sum_p2 = (sum_p1 + sum_p3) / 2.0;
        // } else if sum_p3 == 0.0 {
        //     sum_p3 = (sum_p1 + sum_p2) / 2.0;
        // }
        if ((sum_p1 < 2.5) && (sum_p2 > 2.5) && (sum_p3 > 2.5)) || ((sum_p1 > 2.5) && (sum_p2 < 2.5) && (sum_p3 < 2.5)) {
            if sum_p1 < 5.0 {
                sum_p1 = (sum_p2 + sum_p3) / 2.0;
            }
        } else if ((sum_p2 < 2.5) && (sum_p1 > 2.5) && (sum_p3 > 2.5)) || ((sum_p2 > 2.5) && (sum_p1 < 2.5) && (sum_p3 < 2.5)) {
            if sum_p2 < 5.0 {
                sum_p2 = (sum_p1 + sum_p3) / 2.0;
            }
        } else if ((sum_p3 < 2.5) && (sum_p1 > 2.5) && (sum_p2 > 2.5)) || ((sum_p3 > 2.5) && (sum_p1 < 2.5) && (sum_p2 < 2.5)) {
            if sum_p3 < 5.0 {
                sum_p3 = (sum_p1 + sum_p2) / 2.0;
            }
        }
        let sum_buf = sum_p1 * sum_p2 * sum_p3 * 0.38;  // * 0.25;
        out[i - 2] = sum_buf;
    }
    let max_out: f32 = out.iter().fold(f32::MIN, |a, b| a.max(*b));
    for i in 0..out.len() {
        out[i] = -(out[i] - max_out);
    }
    let out_len = out.len();
    out[0] = out[2];
    out[0] = out[2];
    out[out_len-2] = out[out_len-3];
    out[out_len-1] = out[out_len-3];
    out = step_moving_average(&out, 20);
    out = moving_average(&out, 40);
    out = moving_average(&out, 20);
    out
}

pub fn get_coef_fibr(coef_p: &Vec<f32>, coef_disp: &Vec<f32>, time_param: &TimeParam) -> Vec<f32> {
    let mut out: Vec<f32> = vec![0.0; coef_p.len()];
    for i in 0..coef_p.len() {
        out[i] = coef_p[i] * coef_disp[i] * (0.5 + 100.0 / time_param.threshold[i]) * 0.9;
    }
    out = truncate_win2(&out, 0.85, 160);
    out = truncate_win2(&out, 0.75, 80);
    // out = step_moving_average(&out, 8);
    // out = moving_average(&out, 12);
    out
}

pub fn median(vec: &mut Vec<f32>) -> f32 {
    // Сортируем вектор
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = vec.len();
    if len % 2 == 0 {
        // Если четное количество элементов
        let mid1 = vec[len / 2 - 1];
        let mid2 = vec[len / 2];
        (mid1 + mid2) / 2.0
    } else {
        // Если нечетное количество элементов
        vec[len / 2]
    }
}
