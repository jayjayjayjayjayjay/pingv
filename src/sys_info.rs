use std::fs;
use std::error::Error;

pub struct  MemInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_available: u64
}

impl MemInfo {
    pub fn update_memory_info(&mut self) ->Result<(),Box<dyn Error>>{
        let contents = fs::read_to_string("/proc/meminfo")?;
        for line in contents.lines(){
        if let Some(field_upper_index) = line.find(':') {
            let field: String  = line[0..field_upper_index].into();
             
            let num_str: String = line.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let num: u64 = num_str.parse()?;
            match field.as_str()  {
                "MemTotal"=>self.mem_total = num,
                "MemFree"=>self.mem_free= num,
                "MemAvailable"=>self.mem_available= num,
                _=> continue,
            }
        } else {
            continue;
        }

        }

        Ok(())
    }


}
