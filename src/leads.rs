use std::io::Read;
use npy::NpyData;

pub struct Leads {
    // pub current_dir: PathBuf,
    pub lead1: Vec<f32>,
    pub lead2: Vec<f32>,
    pub lead3: Vec<f32>,
}

impl Leads {
    pub fn new() -> Leads {
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {}", current_dir.display());
        let mut buf = vec![];
        std::fs::File::open(current_dir.join("clean_lead1.npy")).unwrap().read_to_end(&mut buf).unwrap();
        let lead1:NpyData<f32> = NpyData::from_bytes(&buf).unwrap();

        let mut buf = vec![];
        std::fs::File::open(current_dir.join("clean_lead2.npy")).unwrap().read_to_end(&mut buf).unwrap();
        let lead2:NpyData<f32> = NpyData::from_bytes(&buf).unwrap();

        let mut buf = vec![];
        std::fs::File::open(current_dir.join("clean_lead3.npy")).unwrap().read_to_end(&mut buf).unwrap();
        let lead3:NpyData<f32> = NpyData::from_bytes(&buf).unwrap();

        Leads {
            // current_dir: current_dir,
            lead1: lead1.to_vec(),
            lead2: lead2.to_vec(),
            lead3: lead3.to_vec(),
        }
    }
}
