use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct IpHash {
    //Contains the ips address and the data from there
    //[0] = string of u8, [1] = string of date time
    ips: HashMap<String, [String; 2]>,
    ///Contains the limit for value
    pub limit: u8,
}
impl IpHash {
    pub fn new() -> Self {
        IpHash {
            //The Hash for all ips
            ips: HashMap::new(),
            //This is the limit of ips values, the limit is the max size of u8
            limit: 99,
        }
    }
    ///Insert a ip into the ip hash
    pub fn insert(&mut self, ip: String) {
        //Ip exist
        if self.get_value(&ip) > 0 {
            let mut hash_value: [String; 2] = self.get_hash(&ip);
            let value: u8 = self.convert_string_to_u8(hash_value[0].clone());
            //Check if the limit is overflowed
            if value >= self.limit {
                //Update the date to now to reenable the timeout
                self.update_date_to_now_from_array(&mut hash_value);
            } else {
                //Increase the value to +1
                self.increase_value_from_array(&mut hash_value);
            }
            //Update the hash value
            self.ips.insert(ip, hash_value);
        }
        //Not exist
        else {
            //Create new hash value
            self.create_new_array_to_ip(ip);
        }
    }

    ///Get the quantity of ips in ip hash
    pub fn length(&self) -> u16 {
        let length: usize = self.ips.len();
        let converted_length: Result<u16, _> = length.try_into();

        match converted_length {
            //u16 value
            Ok(value) => value,
            //Overflow
            Err(_) => 1000,
        }
    }

    ///Get the value from ip, returns 0 if not exist
    pub fn get_value(&self, ip: &String) -> u8 {        
        //Get the hash value from the ips hash
        if let Some(ip_value) = self.ips.get(ip) {
            //Convert the u8 string to u8
            match ip_value[0].parse::<u8>() {
                Ok(parsed_value) => {
                    return parsed_value;
                }
                Err(_) => {
                    return 0;
                }
            }
        }
        0
    }

    ///Get the hash from ip
    fn get_hash(&self, ip: &String) -> [String; 2] {
        self.ips
            .get(ip)
            .cloned()
            .unwrap_or([String::from("0"), String::from("0")])
    }

    ///Convert the string to u8, in case of errors return 0
    fn convert_string_to_u8(&self, value: String) -> u8 {
        //Convert the u8 string to u8
        match value.parse::<u8>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => 0,
        }
    }

    ///Update the date time from the array to now
    fn update_date_to_now_from_array(&self, array: &mut [String; 2]) {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH);
        let time_now_string = match time_now {
            Ok(duration) => format!("{:?}", duration.as_secs()),
            Err(_) => String::from("0"),
        };
        array[1] = time_now_string;
    }

    ///Increase the value from array to +1
    fn increase_value_from_array(&self, array: &mut [String; 2]) {
        let value: u8 = self.convert_string_to_u8(array[0].clone()) + 1;
        array[0] = value.to_string();
    }

    ///Create a new array to be placed in the ip hash map
    fn create_new_array_to_ip(&mut self, ip: String) {
        //Create the array
        let mut array = [String::from("1"), String::from("0")];
        //Update date time to now
        self.update_date_to_now_from_array(&mut array);
        //Insert in the hash
        self.ips.insert(ip, array);
    }
}
