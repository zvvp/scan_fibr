use crate::leads::Leads;
use ndarray::{Array1};
use ndarray_npy::write_npy;
use crate::my_lib::{moving_average, my_filtfilt, truncate_win2};
use r_param::Rparam;
use crate::zub_p::Zubp;

mod leads;
mod my_lib;
mod r_param;
mod zub_p;

fn main() {
    // let leads = Leads::new();
    // let b: Vec<f32> = vec![0.00259189, 0.00777566, 0.00777566, 0.00259189];
    // let a: Vec<f32> = vec![1.0, -2.3989593, 1.96548122, -0.54578683];
    // let av_clean_lead1 = my_filtfilt(&b, &a, &leads.lead1);
    // // let av_clean_lead1 = truncate_win2(&leads.lead1, 0.5, 50);
    // let array: Array1<f32> = av_clean_lead1.into();
    // write_npy("av_clean_lead1.npy", &array).expect("Ошибка сохранения .npy файла");
    //
    // let r_param = Rparam::new();
    let leads = Leads::new();
    let r_param = Rparam::new();
    let mut zub_p = Zubp::new();
    zub_p.get_amp_pos(&leads, &r_param);
    println!("mean_PR1 = {}", zub_p.mean_PR1);
    println!("mean_PR2 = {}", zub_p.mean_PR2);
    println!("mean_PR3 = {}", zub_p.mean_PR3);
    println!("mean_amp_p1 = {}", zub_p.mean_amp_p1);
    println!("mean_amp_p2 = {}", zub_p.mean_amp_p2);
    println!("mean_amp_p3 = {}", zub_p.mean_amp_p3);

    // let addr: usize = 280;
    // println!("intervals {:?}", &r_param.intervals[addr..addr+25]);
    // println!("{:?}", &r_param.chars[addr..addr+25]);
    // println!("{:?}", &r_param.chars.len());
    // println!("fintervals {:?}", &r_param.f_intervals[addr..addr+25]);
    // println!("{:?}", &r_param.f_intervals.len());
    // println!("inds_min_diff {:?}", &r_param.inds_min_diff[0..0+25]);
    // println!("{:?}", &r_param.inds_min_diff.len());
    // println!("clean_intervals {:?}", &r_param.average_intervals[addr..addr+25]);
    // println!("{:?}", &r_param.average_intervals.len());
}
