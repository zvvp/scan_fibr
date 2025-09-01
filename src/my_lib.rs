use ndarray::Array1;


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
    let mut min: f32 = 0.0;
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