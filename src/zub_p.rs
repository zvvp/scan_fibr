use crate::leads::Leads;
use crate::my_lib::{find_local_max, find_max, find_min, my_filtfilt};
use crate::r_param::Rparam;

pub struct Zubp {
    pub mean_amp_p1: f32,
    pub mean_amp_p2: f32,
    pub mean_amp_p3: f32,
    pub mean_PR1: f32,
    pub mean_PR2: f32,
    pub mean_PR3: f32,
    pub coef_p: Vec<f32>,
}

impl Zubp {
    pub fn new() -> Zubp {
        Zubp {
            mean_amp_p1: 0.0,
            mean_amp_p2: 0.0,
            mean_amp_p3: 0.0,
            mean_PR1: 0.0,
            mean_PR2: 0.0,
            mean_PR3: 0.0,
            coef_p: vec![],
        }
    }
    pub fn get_amp_pos(&mut self, leads: &Leads, r_param: &Rparam) {
        let bl: Vec<f32> = vec![0.00259189, 0.00777566, 0.00777566, 0.00259189];
        let al: Vec<f32> = vec![1.0, -2.3989593, 1.96548122, -0.54578683];
        let mut sum_amp_p1: f32 = 0.0;
        let mut sum_amp_p2: f32 = 0.0;
        let mut sum_amp_p3: f32 = 0.0;
        let mut sum_PR1: f32 = 0.0;
        let mut sum_PR2: f32 = 0.0;
        let mut sum_PR3: f32 = 0.0;
        let mut len_sum1: f32 = 1.0;
        let mut len_sum2: f32 = 1.0;
        let mut len_sum3: f32 = 1.0;
        for ind in &r_param.inds_min_diff {
            if (r_param.chars[*ind] == 'N') { // && (r_param.chars[*ind - 1] == 'N') {
                let len_pr = r_param.intervals[*ind] * 0.36 + 5.0;
                // if *ind < 50 {
                //     println!("ind = {ind}, len_pr = {len_pr}");
                // }
                let start = (r_param.r_pos[*ind] - len_pr) as usize;
                let stop = (r_param.r_pos[*ind] - 7.0) as usize;
                let fragment1 = leads.lead1[start..stop].to_vec();
                let fragment2 = leads.lead2[start..stop].to_vec();
                let fragment3 = leads.lead3[start..stop].to_vec();
                let fragment1 = my_filtfilt(&bl, &al, &fragment1);
                let fragment2 = my_filtfilt(&bl, &al, &fragment2);
                let fragment3 = my_filtfilt(&bl, &al, &fragment3);
                let (amp_p1, ind_p1) = self.get_amp_ind_p(&fragment1);
                // if *ind < 50 {
                //     println!("amp_p1 = {amp_p1}, ind_p1 = {ind_p1}");
                // }
                if (amp_p1 < 1.0) && (amp_p1 > 0.01) {
                    let pr1 = len_pr - ind_p1 as f32;
                    sum_PR1 = sum_PR1 + pr1;
                    sum_amp_p1 = sum_amp_p1 + amp_p1;
                    len_sum1 = len_sum1 + 1.0;
                }
                let (amp_p2, ind_p2) = self.get_amp_ind_p(&fragment2);
                if (amp_p2 < 1.0) && (amp_p2 > 0.01) {
                    let pr2 = len_pr - ind_p2 as f32;
                    sum_PR2 = sum_PR2 + pr2;
                    sum_amp_p2 = sum_amp_p2 + amp_p2;
                    len_sum2 = len_sum2 + 1.0;
                }
                let (amp_p3, ind_p3) = self.get_amp_ind_p(&fragment3);
                if (amp_p3 < 1.0) && (amp_p3 > 0.01) {
                    let pr3 = len_pr - ind_p3 as f32;
                    sum_PR3 = sum_PR3 + pr3;
                    sum_amp_p3 = sum_amp_p3 + amp_p3;
                    len_sum3 = len_sum3 + 1.0;
                }
            }
        }
        self.mean_PR1 = sum_PR1 / len_sum1;
        self.mean_PR2 = sum_PR2 / len_sum2;
        self.mean_PR3 = sum_PR3 / len_sum3;
        self.mean_amp_p1 = sum_amp_p1 / len_sum1;
        self.mean_amp_p2 = sum_amp_p2 / len_sum2;
        self.mean_amp_p3 = sum_amp_p3 / len_sum3;
    }

    fn get_amp_ind_p(&mut self, fragment: &Vec<f32>) -> (f32, usize) {
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

    pub fn get_P(&mut self, leads: &Leads, rparam: &Rparam) {
        let b: Vec<f32> = vec![0.02155836, 0.04311672, 0.02155836];
        let a: Vec<f32> = vec![1.0, -1.54383625, 0.6300697];
        let bh: Vec<f32> = vec![0.99749302, -0.99749302];
        let ah: Vec<f32> = vec![1.0, -0.99498604];
        let mut p1: Vec<f32> = vec![0.0; rparam.r_pos.len()];
        let mut p2: Vec<f32> = vec![0.0; rparam.r_pos.len()];
        let mut p3: Vec<f32> = vec![0.0; rparam.r_pos.len()];
        let mut out: Vec<f32> = vec![0.0; rparam.r_pos.len()];
        for i in 4..rparam.r_pos.len() {
            let len_pr = rparam.intervals[i].sqrt() * 3.7;
            let beth_pr = rparam.intervals[i].sqrt() * 0.66;
            let start = (rparam.r_pos[i] - len_pr) as usize;
            let stop = (rparam.r_pos[i] - beth_pr) as usize;
            let start1 = (rparam.r_pos[i] - self.mean_PR1 - 15.0) as usize;
            let start2 = (rparam.r_pos[i] - self.mean_PR2 - 15.0) as usize;
            let start3 = (rparam.r_pos[i] - self.mean_PR3 - 15.0) as usize;
            let stop1 = (rparam.r_pos[i] - self.mean_PR1 + self.mean_PR1.sqrt() * 3.0) as usize;
            let stop2 = (rparam.r_pos[i] - self.mean_PR2 + self.mean_PR2.sqrt() * 3.0) as usize;
            let stop3 = (rparam.r_pos[i] - self.mean_PR3 + self.mean_PR3.sqrt() * 3.0) as usize;
            if self.mean_PR1 > rparam.intervals[i] * 0.36 {
                let fragment_l1 = leads.lead1[start..stop].to_vec();
            } else {
                let fragment_l1 = leads.lead1[start1..stop1].to_vec();
            }
            if self.mean_PR2 > rparam.intervals[i] * 0.36 {
                let fragment_l2 = leads.lead2[start..stop].to_vec();
            } else {
                let fragment_l2 = leads.lead2[start2..stop2].to_vec();
            }
            if self.mean_PR3 > rparam.intervals[i] * 0.36 {
                let fragment_l3 = leads.lead3[start..stop].to_vec();
            } else {
                let fragment_l3 = leads.lead3[start3..stop3].to_vec();
            }
        }
    }
}
