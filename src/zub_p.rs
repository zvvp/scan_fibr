use crate::my_lib::{find_local_max, find_local_min, my_filtfilt, Lead
                    , median_filter, find_max};
use crate::time_param::TimeParam;

pub struct Zubp {
    pub mean_amp_p: f32,
    pub presence_pr: Vec<f32>,
    pub intervals_pr: Vec<f32>,
    pub inds_pr: Vec<f32>,
}

impl Zubp {
    pub fn new(r_pos_len: &usize) -> Zubp {
        Zubp {
            mean_amp_p: 0.0,
            presence_pr: vec![0.0; *r_pos_len],
            intervals_pr: vec![],
            inds_pr: vec![],
        }
    }
    pub fn get_mean_amp_pos(&mut self, num: u8, time_param: &TimeParam) {
        /*
            Заполняет структуру Zubp
         */
        let lead = Lead::new(num);
        let mut vec_amp_p: Vec<f32> = vec![];
        for ind in &time_param.inds_min_diff {
            let len_pr = time_param.intervals[*ind] * 0.45;
            let start = (time_param.r_pos[*ind] - len_pr) as usize;
            let stop = (time_param.r_pos[*ind] - 10.0) as usize;
            let fragment = lead.lead[start..stop].to_vec();
            let (amp_p, ind_p) = self.get_amp_ind_p(&fragment);
            if (amp_p < 1.0) && (amp_p > 0.001) {
                let pr = len_pr - ind_p as f32;
                vec_amp_p.push(amp_p);
                self.intervals_pr.push(pr);
                self.inds_pr.push(ind_p as f32);
            }
        }
        if vec_amp_p.is_empty() {
            self.mean_amp_p = 0.0;
        } else {
            let sum: f32 = vec_amp_p.iter().sum();
            let count = vec_amp_p.len() as f32;
            self.mean_amp_p = sum / count;
        }
        self.intervals_pr = median_filter(&self.intervals_pr, 19);
        self.interp_pr();
    }

    fn get_amp_ind_p(&mut self, fragment: &Vec<f32>) -> (f32, usize) {
        /*
            Возвращает амп. P и его индекс в фрагменте(PR)
            для вычисления mean_amp_p и mean_PR
         */
        let b: Vec<f32> = vec![0.02785977, 0.05571953, 0.02785977];
        let a: Vec<f32> = vec![1.0, -1.47548044, 0.58691951];
        let bi: Vec<f32> = vec![0.03634612, 0.03634612];
        let ai: Vec<f32> = vec![1.0, -0.92730777];
        let mut amp_p: f32 = 0.0;
        let mut ind_p: usize = 0;
        let mut fragment = my_filtfilt(&b, &a, &fragment);
        let isoline = my_filtfilt(&bi, &ai, &fragment);
        for i in 0..fragment.len() {
            fragment[i] = fragment[i] - isoline[i];
        }
        let mut over_fragment: Vec<f32> = vec![0.0; fragment.len()];
        for i in 0..fragment.len() {
            if fragment[i] > 0.0 {
                over_fragment[i] = fragment[i];
            }
        }
        let mut under_fragment: Vec<f32> = vec![0.0; fragment.len()];
        for i in 0..fragment.len() {
            if fragment[i] < 0.0 {
                under_fragment[i] = fragment[i];
            }
        }
        // ind_max - массив индексов лок максимумов больше 0
        // vec_max - массив значений лок максимумов больше 0
        let (ind_max, vec_max) = find_local_max(&over_fragment);
        // ind_min - массив индексов лок минимумов меньше 0
        // vec_min - массив значений лок минимумов меньше 0
        let (ind_min, _vec_min) = find_local_min(&under_fragment);
        let len_ind_min = ind_min.len();
        let mut ind_loc_extrem = ind_min;
        if ind_max.len() >= 1 {
            let max_ind_p = find_max(&ind_max, &vec_max);
            ind_p = max_ind_p.1;
            if len_ind_min > 0 {
                ind_loc_extrem.push(ind_p); // массив индексов лок минимумов и ind_p
                //Сортируем массив индексов
                ind_loc_extrem.sort_by(|a, b| a.partial_cmp(b).unwrap());
                // Массив значений лок максимумов и лок минимумов
                let loc_extrem: Vec<f32> = ind_loc_extrem.iter()
                    .filter_map(|&index| fragment.get(index))
                    .cloned().collect();
                let (_max_val, max_index) = find_max(&ind_loc_extrem, &loc_extrem);
                if max_index == 0 {
                    amp_p = loc_extrem[0];
                } else if max_index == loc_extrem.len() - 1 {
                    amp_p = loc_extrem[loc_extrem.len() - 1];
                } else {
                    let side1 = loc_extrem[max_index] - loc_extrem[max_index - 1];
                    let side2 = loc_extrem[max_index] - loc_extrem[max_index + 1];
                    if side1 < side2 {
                        amp_p = side1;
                    } else {
                        amp_p = side2;
                    }
                }
            } else {
                amp_p = fragment[ind_p];
            }
        }
        (amp_p, ind_p)
    }

    // fn find_p1(&self, fragment: &Vec<f32>) -> f32 {
    //     /*
    //         Возвращает 1.0, если P зубец найден 0.0, если нет
    //      */
    //     let mut pzub: f32 = 0.0;
    //     let mut amp_pzub: f32 = 0.0;
    //     let mut amp_pzub1: f32 = 0.0;
    //     let mut amp_pzub2: f32 = 0.0;
    //     let locminmax = LocMinMax::new(fragment);
    //     /*println!("arr_loc {:?}", &locminmax.arr_loc);
    //     println!("diff_loc {:?}", &locminmax.diff_loc);
    //     println!("ind_max {:?}", &locminmax.ind_max);
    //     println!("ind_min {:?}", &locminmax.ind_min);*/
    //     if locminmax.ind_max.len() == 1 {
    //         if locminmax.ind_min.len() == 0 {
    //             amp_pzub = find_min(&locminmax.diff_loc);
    //         } else if locminmax.ind_min.len() == 1 {
    //             if locminmax.ind_max[0] < locminmax.ind_min[0] {
    //                 amp_pzub = find_min(&locminmax.diff_loc[..2].to_vec());
    //             } else if locminmax.ind_max[0] > locminmax.ind_min[0] {
    //                 amp_pzub = find_min(&locminmax.diff_loc[1..3].to_vec());
    //             }
    //         } else if locminmax.ind_min.len() == 2 {
    //             amp_pzub = find_min(&locminmax.diff_loc[1..3].to_vec());
    //         }
    //     } else if locminmax.ind_max.len() == 2 {
    //         if locminmax.ind_min.len() == 1 {
    //             amp_pzub1 = find_min(&locminmax.diff_loc[..2].to_vec());
    //             amp_pzub2 = find_min(&locminmax.diff_loc[2..].to_vec());
    //         } else if locminmax.ind_min.len() == 2 {
    //             if locminmax.ind_max[0] < locminmax.ind_min[0] {
    //                 amp_pzub1 = find_min(&locminmax.diff_loc[..2].to_vec());
    //                 amp_pzub2 = find_min(&locminmax.diff_loc[2..4].to_vec());
    //             } else if locminmax.ind_max[0] > locminmax.ind_min[0] {
    //                 amp_pzub1 = find_min(&locminmax.diff_loc[1..3].to_vec());
    //                 amp_pzub2 = find_min(&locminmax.diff_loc[3..].to_vec());
    //             }
    //         } else if locminmax.ind_min.len() == 3 {
    //             amp_pzub1 = find_min(&locminmax.diff_loc[1..3].to_vec());
    //             amp_pzub2 = find_min(&locminmax.diff_loc[3..5].to_vec());
    //         }
    //         if (amp_pzub1 > amp_pzub2) && (amp_pzub1 / amp_pzub2 > 1.1) {   // > 10.0
    //             amp_pzub = amp_pzub1;
    //         } else if (amp_pzub2 > amp_pzub1) && (amp_pzub2 / amp_pzub1 > 1.1) {   // > 10.0
    //             amp_pzub = amp_pzub2;
    //         }
    //     }
    //     if (amp_pzub > self.mean_amp_p * 0.005) && (amp_pzub > 0.0005) {
    //     // if (amp_pzub > self.mean_amp_p * 0.05) && (amp_pzub > 0.0035) {
    //         pzub = 1.0;
    //     }
    //     pzub
    // }

    pub fn get_p_in_lead(&mut self, num: u8, time_param: &TimeParam) -> Vec<f32> {
        let lead = Lead::new(num);
        let mut p: Vec<f32> = vec![0.0; time_param.r_pos.len()];
        for i in 4..time_param.r_pos.len() {
            let start = (time_param.r_pos[i] - self.presence_pr[i] - 15.0) as usize;
            let stop = if (self.presence_pr[i] - 15.0) > 10.0 {
                (time_param.r_pos[i] - self.presence_pr[i] + 15.0) as usize
            } else {
                (time_param.r_pos[i] - 10.0) as usize
            };
            let fragment = lead.lead[start..stop].to_vec();
            if time_param.chars[i] != 'N' {
                p[i] = 1.0;
            } else {
                let (amp_p, _ind_p) = self.get_amp_ind_p(&fragment);
                if amp_p > self.mean_amp_p * 0.12 {
                    p[i] = 1.0;
                }
            }
        }
        p
    }

    fn interp_line(&mut self, ind_start: usize, ind_stop: usize, val_start: f32, val_stop: f32) {
        let diff_val = val_stop - val_start;
        let count_step = ind_stop - ind_start;
        if count_step > 0 {
            let step_val = diff_val / count_step as f32;
            for i in ind_start..ind_stop {
                self.presence_pr[i] = val_start + step_val * (i as f32);
            }
        }
    }
    fn interp_pr(&mut self) {
        for i in 1..self.inds_pr.len() {
            let ind_start = self.inds_pr[i - 1] as usize;
            let ind_stop = self.inds_pr[i] as usize;
            let val_start = self.intervals_pr[i - 1];
            let val_stop = self.intervals_pr[i];
            self.interp_line(ind_start, ind_stop, val_start, val_stop);
        }
        self.interp_line(0, self.inds_pr[0] as usize, self.intervals_pr[0], self.intervals_pr[0] );
        self.interp_line(self.inds_pr.len(), self.presence_pr.len(), self.intervals_pr[self.inds_pr.len() - 1], self.intervals_pr[self.inds_pr.len() - 1]);
    }
}
