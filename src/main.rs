use ndarray::{Array1};
use ndarray_npy::write_npy;
use crate::my_lib::{moving_average, Lead, my_filtfilt, truncate_win2, get_coef_p};
use time_param::TimeParam;
use crate::zub_p::Zubp;

mod leads;
mod my_lib;
mod time_param;
mod zub_p;

fn main() {
    let coef_p = get_coef_p();
    let coef_p: Vec<f64> = coef_p.iter().map(|&x| x as f64).collect();
    let array: Array1<f64> = coef_p.into();
    write_npy("coef_p.npy", &array).expect("Ошибка сохранения .npy файла");
}
