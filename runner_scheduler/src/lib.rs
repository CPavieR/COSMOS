use std::{collections::HashMap, fs::File, io::{Read, Seek}};

use crate::operator::CSVFile;
mod operator;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[doc = "This fonction need the file descriptor of the Json returned by the semantic parsor. Return a file
TODO==> Add the Where, Group by, Having and intermediary request"]
pub fn scheduler (mut json_file:&File)->File{
    json_file.rewind().expect("Rewind error==> Can't reset de cursor of the File");// We reset de file descriptor
    //println!("You passed the rewind");
    let mut buffer = Vec::new();
    json_file.read_to_end(&mut buffer).expect("Read to end error");
    let mut str_json :String = String::new();
    for i in buffer{
        str_json.push(i as char);
    }
    let parse_json=json::parse(&str_json.to_string()).unwrap();
    parse_json.dump();
    //println!("{}",parse_json["tables"][0]["columns"]);
    let mut key:Vec<String>=Vec::new();//We need to keep the list of the key in memory
    let mut final_proj:Vec<String>= Vec::new();//list of all the 
    let mut dictionnary: HashMap<String, crate::operator::CSVFile> = HashMap::new();

    for i in 0..parse_json["tables"].len(){
        let mut intermediary_vector:Vec<String>=Vec::new();
        for y in 0..parse_json["tables"][i]["columns"].len(){
            let mut my_str:String = parse_json["tables"][i]["columns"][y][0].to_string();//We rename immediatly the columns for more simplicity in the next operation (for the future we need to search a methode to apply the rename of the user ( key word 'as'))
            my_str.push('.');
            my_str.push_str(&parse_json["tables"][i]["columns"][y][1].to_string());
            intermediary_vector.push(my_str.clone());
            final_proj.push(my_str); 
        }
        key.push(parse_json["tables"][i]["table_name"].to_string());
        //println!("{:?}",intermediary_vector);
        //println!("{}",parse_json["tables"][i]["table_name"]);
        let mut open_file:CSVFile = operator::open_relation(parse_json["tables"][i]["table_name"].to_string(), parse_json["tables"][i]["table_name"].to_string());//We open each relation
        open_file.projection(intermediary_vector);//We made a first projection to keep only the date we use for the request
        dictionnary.insert(parse_json["tables"][i]["table_name"].to_string(),open_file);//We insert the projected file in a dictionnary
    }
    // Now we need to do the cartesian product on all the relation use in the request. For this we made the cartesian product on the first open file.
    for i in 1..key.len(){
        let mut test =dictionnary.get_mut(&key[0]).expect("Get error ").clone();
        let test2 =dictionnary.get(&key[i]).expect("Get error");
        test.cartesian_product(test2);
        dictionnary.insert(key[0].to_string(), test);
    }
    //After the cartesian product, we need to close de file. For this we create a file of first open file (so the first entry create in the dictionnary)
    let mut a1 = dictionnary[&key[0]].clone();
    a1.projection(final_proj);
    a1.to_file();
    return a1.to_file(); ;
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[should_panic]
    fn test_on_create_file(){
        {
            let mut fichier_test:std::fs::File = File::create("test.txt").expect("Can't create a file");
            scheduler(&fichier_test);
            let data = b"hello world";
            fichier_test.write_all(data).expect("Can't write in this file");
            scheduler(&fichier_test);
        }
        let mut fichier_test:std::fs::File = File::open("test.txt").expect("Can't open the file");
        scheduler(&fichier_test);
        let mut buf = [1];
        let mut _a = fichier_test.read(&mut buf).expect("Can't read a file");
        scheduler(&fichier_test);
    }

    #[test]
    fn test_on_json(){
        let fichier_json_test:std::fs::File = File::open("semantique.json").expect("Error ==> Can't read the JSON file");
        scheduler(&fichier_json_test);
    }
}
