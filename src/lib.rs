use binaryfile::{BinaryReader, BinaryWriter};
use sjis::{decode, is_sjis};
use pathobj::PathObj;
use anyhow::Result;
use unique_id::{Generator, string::StringGenerator};


pub struct Fixed {
    pub filepath: String,
    pub is_tmp: bool,
}

impl Drop for Fixed {
    fn drop(&mut self) {
        if self.is_tmp {
            std::fs::remove_file(&self.filepath).unwrap();
        }
    }
}

impl Fixed {

    pub fn filepath(&self) -> String {
        self.filepath.clone()
    }

    pub fn new(filepath: &str) -> Result<Self> {

        let target: String;

        let result = check_arrive(filepath);
        if let Err(e) = result {
            return Err(e);
        }
            
        let is_tmp = is_file_sjis(filepath);

        if is_tmp {
            let gen = StringGenerator::default();
            let uid = gen.next_id();

            let mut p = PathObj::new();
            p.getcwd();
            p.push(&uid);

            target = p.get();

            let mut writer = BinaryWriter::new(&target).unwrap();
            let reader = BinaryReader::open(filepath).unwrap();
            for line in reader {
                let buf = match line {
                    Ok(v) => decode(v),
                    Err(_) => "".to_string(),
                };

                let out_buf = buf.as_bytes().to_vec();
                writer.write(&out_buf).unwrap();
                writer.write(&vec![0x0a]).unwrap();
            }


        } else {
            target = filepath.to_string(); 
        }

        Ok(
            Self {
                filepath: target,
                is_tmp,
            }
        )
    }
}

fn check_arrive(filepath: &str) -> Result<()> {
    let mut p = PathObj::new();
    p.from_str(filepath);
    if p.is_exists() == false {
        return Err(anyhow::anyhow!("file not found!"));
    }
    Ok(())
}

fn is_file_sjis(filepath: &str) -> bool {
    let reader = BinaryReader::open(filepath).unwrap();
    let mut flg: bool = false;
    for line in reader {
        match line {
            Ok(v) => {
                if is_sjis(&v) {
                    flg = true;
                }
            },
            Err(_) => {},
        };
    }
    return flg;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let target = "test.txt".to_string();
        let fixed = Fixed::new(&target).unwrap();

        let reader = BinaryReader::open(&fixed.filepath()).unwrap();
        for line in reader {
            match line {
                Ok(v) => println!("{}", String::from_utf8(v).unwrap()),
                Err(_) => {
                    assert!(false, "failed");
                },
            };
        }

        println!("filepath: {}", fixed.filepath());
        
        assert!(true, "yey!");
    }
}
