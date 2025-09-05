use ndarray::{Array1};
use ndarray_npy::write_npy;
use crate::my_lib::{get_coef_fibr, get_coef_p};
use time_param::TimeParam;
use crate::disp::get_coef_disp;
use crate::fibr::Fibr;

mod leads;
mod my_lib;
mod time_param;
mod zub_p;
mod disp;
mod fibr;

fn main() {
    let time_param = TimeParam::new();
    let trs = &time_param.threshold;
    let coef_p = get_coef_p(&time_param);
    let coef_disp = get_coef_disp(&time_param);
    let coef_fibr = get_coef_fibr(&coef_p, &coef_disp, &time_param);
    let mask: Vec<usize> = vec![1; coef_fibr.len()];
    let fibr = Fibr::new(&coef_fibr, &trs, &mask);
    println!("{:?}", fibr);

    // let trs: Vec<f64> = trs.iter().map(|&x| x as f64).collect();
    // let array: Array1<f64> = trs.into();
    // write_npy("trs.npy", &array).expect("Ошибка сохранения .npy файла");
    //
    // let coef_fibr: Vec<f64> = coef_fibr.iter().map(|&x| x as f64).collect();
    // let array: Array1<f64> = coef_fibr.into();
    // write_npy("coef_fibr.npy", &array).expect("Ошибка сохранения .npy файла");

}
