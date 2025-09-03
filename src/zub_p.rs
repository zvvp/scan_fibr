use std::cmp::min;
use crate::my_lib::{find_local_max, find_max, find_min, my_filtfilt, Lead, LocMinMax};
use crate::time_param::TimeParam;

pub struct Zubp {
    pub mean_amp_p: f32,
    pub mean_PR: f32,
}

impl Zubp {
    pub fn new() -> Zubp {
        Zubp {
            mean_amp_p: 0.0,
            mean_PR: 0.0,
        }
    }
    fn get_mean_amp_pos(&mut self, lead: &Lead, time_param: &TimeParam) {
        /*
            Вычисляет ср. амп. P зубца и ср. расстояние PR для отведения lead
         */
        let bl: Vec<f32> = vec![0.00259189, 0.00777566, 0.00777566, 0.00259189];
        let al: Vec<f32> = vec![1.0, -2.3989593, 1.96548122, -0.54578683];
        let mut sum_amp_p: f32 = 0.0;
        let mut sum_PR: f32 = 0.0;
        let mut len_sum: f32 = 1.0;
        for ind in &time_param.inds_min_diff {
            if time_param.chars[*ind] == 'N' {
                let len_pr = time_param.intervals[*ind] * 0.36 + 5.0;
                let start = (time_param.r_pos[*ind] - len_pr) as usize;
                let stop = (time_param.r_pos[*ind] - 7.0) as usize;
                let fragment = lead.lead[start..stop].to_vec();
                let fragment = my_filtfilt(&bl, &al, &fragment);

                let (amp_p, ind_p) = self.get_amp_ind_p(&fragment);
                if (amp_p < 1.0) && (amp_p > 0.01) {
                    let pr = len_pr - ind_p as f32;
                    sum_PR = sum_PR + pr;
                    sum_amp_p = sum_amp_p + amp_p;
                    len_sum = len_sum + 1.0;
                }
            }
        }
        self.mean_PR = sum_PR / len_sum;
        self.mean_amp_p = sum_amp_p / len_sum;
    }

    fn get_amp_ind_p(&mut self, fragment: &Vec<f32>) -> (f32, usize) {
        /*
            Возвращает амп. P и его индекс в фрагменте(PR)
            для вычисления mean_amp_p и mean_PR
         */
        let mut amp_p: f32 = 0.0;
        let mut ind_p: usize = 0;
        let (ind_max, vec_max) = find_local_max(fragment);
        if ind_max.len() >= 1 {
            let max_ind_p = find_max(&ind_max, &vec_max);
            let max = max_ind_p.0;
            ind_p = max_ind_p.1;
            let frag_l = fragment[..ind_p].to_vec();
            let frag_r = fragment[ind_p..].to_vec();
            let min_l = find_min(&frag_l);
            let min_r = find_min(&frag_r);
            let mut isoline = 0.0;
            if min_l < min_r {
                isoline = min_l;
            } else {
                isoline = min_r;
            }
            amp_p = max - isoline;
        }
        (amp_p, ind_p)
    }

    fn find_p(&self, fragment: &Vec<f32>) -> f32 {
        /*
            Возвращает 1.0, если P зубец найден 0.0, если нет
         */
        let mut pzub: f32 = 0.0;
        let mut amp_pzub: f32 = 0.0;
        let mut amp_pzub1: f32 = 0.0;
        let mut amp_pzub2: f32 = 0.0;
        let locminmax = LocMinMax::new(fragment);
        /*println!("arr_loc {:?}", &locminmax.arr_loc);
        println!("diff_loc {:?}", &locminmax.diff_loc);
        println!("ind_max {:?}", &locminmax.ind_max);
        println!("ind_min {:?}", &locminmax.ind_min);*/
        if locminmax.ind_max.len() == 1 {
            if locminmax.ind_min.len() == 0 {
                amp_pzub = find_min(&locminmax.diff_loc);
            } else if locminmax.ind_min.len() == 1 {
                if locminmax.ind_max[0] < locminmax.ind_min[0] {
                    amp_pzub = find_min(&locminmax.diff_loc[..2].to_vec());
                } else if locminmax.ind_max[0] > locminmax.ind_min[0] {
                    amp_pzub = find_min(&locminmax.diff_loc[1..3].to_vec());
                }
            } else if locminmax.ind_min.len() == 2 {
                amp_pzub = find_min(&locminmax.diff_loc[1..3].to_vec());
            }
        } else if locminmax.ind_max.len() == 2 {
            if locminmax.ind_min.len() == 1 {
                amp_pzub1 = find_min(&locminmax.diff_loc[..2].to_vec());
                amp_pzub2 = find_min(&locminmax.diff_loc[2..].to_vec());
            } else if locminmax.ind_min.len() == 2 {
                if locminmax.ind_max[0] < locminmax.ind_min[0] {
                    amp_pzub1 = find_min(&locminmax.diff_loc[..2].to_vec());
                    amp_pzub2 = find_min(&locminmax.diff_loc[2..4].to_vec());
                } else if locminmax.ind_max[0] > locminmax.ind_min[0] {
                    amp_pzub1 = find_min(&locminmax.diff_loc[1..3].to_vec());
                    amp_pzub2 = find_min(&locminmax.diff_loc[3..].to_vec());
                }
            } else if locminmax.ind_min.len() == 3 {
                amp_pzub1 = find_min(&locminmax.diff_loc[1..3].to_vec());
                amp_pzub2 = find_min(&locminmax.diff_loc[3..5].to_vec());
            }
            if (amp_pzub1 > amp_pzub2) && (amp_pzub1 / amp_pzub2 > 10.0) {   // > 10.0
                amp_pzub = amp_pzub1;
            } else if (amp_pzub2 > amp_pzub1) && (amp_pzub2 / amp_pzub1 > 10.0) {   // > 10.0
                amp_pzub = amp_pzub2;
            }
        }
        if (amp_pzub > self.mean_amp_p * 0.04) && (amp_pzub > 0.003) {
        // if (amp_pzub > self.mean_amp_p * 0.05) && (amp_pzub > 0.0035) {
            pzub = 1.0;
        }
        pzub
    }

    pub fn get_P_in_lead(&mut self, num: u8, time_param: &TimeParam) -> Vec<f32> {
        let lead = Lead::new(num);
        let b: Vec<f32> = vec![0.02155836, 0.04311672, 0.02155836];
        let a: Vec<f32> = vec![1.0, -1.54383625, 0.6300697];
        let bh: Vec<f32> = vec![0.99749302, -0.99749302];
        let ah: Vec<f32> = vec![1.0, -0.99498604];
        let mut p: Vec<f32> = vec![0.0; time_param.r_pos.len()];
        let mut out: Vec<f32> = vec![0.0; time_param.r_pos.len()];
        self.get_mean_amp_pos(&lead, time_param);
        for i in 4..time_param.r_pos.len() {
            let len_pr = time_param.intervals[i].sqrt() * 3.7;
            let beth_pr = time_param.intervals[i].sqrt() * 0.66;
            let start = (time_param.r_pos[i] - len_pr) as usize;
            let stop = (time_param.r_pos[i] - beth_pr) as usize;
            let start1 = (time_param.r_pos[i] - self.mean_PR - 15.0) as usize;
            let stop1 = (time_param.r_pos[i] - self.mean_PR + self.mean_PR.sqrt() * 3.0) as usize;
            let fragment: Vec<f32> = if self.mean_PR > time_param.intervals[i] * 0.36 {
                lead.lead[start..stop].to_vec()
            } else {
                lead.lead[start1..stop1].to_vec()
            };
            if time_param.chars[i] == 'N' {
                let fragment = my_filtfilt(&b, &a, &fragment);
                let fragment = my_filtfilt(&bh, &ah, &fragment);
                let pzub = self.find_p(&fragment);
                p[i] = pzub;
            } else {
                p[i] = 1.0;
            }
        }
        p
    }
}
