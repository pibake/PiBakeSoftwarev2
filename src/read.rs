use std::io::{self, BufRead};
use std::path::Path;
use linapi::system::modules::ModuleFile;

use crate::temperature;
use crate::uuid;

pub struct TempRead<'a> {
    pub temp_c: Option<u32>,
    pub temp_f: Option<u32>,
    pub base_dir: &'a Path,
}

impl<'a> TempRead<'a> {
    pub fn new() -> TempRead<'a> {
        let mut modvec: Vec<ModuleFile> = Vec::new();
        
        modvec.push(ModuleFile::from_name("w1-gpio").unwrap());
        modvec.push(ModuleFile::from_name("w1-therm").unwrap());

        for i in &modvec {
            i.load("").unwrap();
        }

        TempRead {
            temp_c: None,
            temp_f: None,
            base_dir: Path::new("/sys/bus/w1/devices/280/w1_slave"),
        }

        // initalize concatination

        // Command::new("sh")
        //     .args(&["modprobe"])
        //     .args(&["w1-gpio"])
        //     .output()
        //     .expect("That isn't supposed to happen!");

        // Command::new("sh")
        //     .args(&["modprobe"])
        //     .args(&["w1-therm"])
        //     .output()
        //     .expect("That isn't supposed to happen!");
        
    }

    fn read_temp_raw(&self) -> Vec<u8> {
        let mut cursor = io::Cursor::new(self.base_dir.to_str().unwrap());
        let mut buf: Vec<u8> = Vec::new();
        let bytes: u8 = 1 ;
        
        while bytes != 0{
            match cursor.read_until(b'\n', &mut buf) {
                Ok(b) => b,
                Err(_) => panic!("Can't read file!")
            };
        }

        buf
    }

    pub fn read_temp(&self) {
        let lines = self.read_temp_raw();
        let lines_slice = lines.as_slice()[1];
        let l = std::str::from_utf8(lines.as_slice()).unwrap();
        let equal_pos = l.find("t=");
        
        // if the variable is at the end of the vec slice
        // not complete, should not be Option<None>
        if equal_pos == None {
            let uuid = uuid::TempUuid::new();
            let e = equal_pos.unwrap();
            let f: [u8; 8] = e.to_be_bytes();
            let mut sum: u8 = 0;

            for i in &f {
                if f.iter().next() != None {
                    sum = sum + i;
                }
            }

            let temp_string: i32 = (lines.as_slice()[1] + sum).into();
            let temp_c: i32 = temp_string / 1000;
            let temp_f: i32 = temp_c * 9_0 / 5_0 + 32_0;
            temperature::Temperature::new(temp_c, temp_f, &uuid.uuid);
        }
    }
}