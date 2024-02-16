use std::string::String;
use std::{collections::HashMap, fs::File, io::{Read, Seek}};
use std::error::Error;
//use json::JsonValue::String; J'espère que les String json sont les
//même ques les trings classique car j'arrive pas a compiler avec
//les json string

use crate::operator::CSVFile;
mod operator;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
//Hey Matthieu j'ai changé un peu ta fonction afin de gérer les erreurs
//Je n'ai rien delete, j'ai juste mis en commentaire les parties qui
//ne sont plus utiles
pub fn scheduler(mut json_file:&File)->Result<&str,Box<dyn Error>>{
    json_file.rewind()?; //.expect("Rewind error==> Can't reset de cursor of the File");
    //println!("You passed the rewind");
    let mut buffer = Vec::new();
    json_file.read_to_end(&mut buffer)?; //.expect("Read to end error");
    let mut str_json  : String = String::new();
    for i in buffer{
        str_json.push(i as char);
    }
    let parse_json=json::parse(&str_json.to_string()).unwrap();
    parse_json.dump();
    //println!("{}",parse_json["table"][0]["columns"]);
    let mut key:Vec<String>=Vec::new();
    let mut final_proj:Vec<String>= Vec::new();
    let mut dictionnary: HashMap<String, crate::operator::CSVFile> = HashMap::new();
    for i in 0..parse_json["table"].len(){
        let mut intermediary_vector:Vec<String>=Vec::new();
        for y in 0..parse_json["table"][i]["columns"].len(){
            let mut my_str:String = parse_json["table"][i]["columns"][y][0].to_string();
            my_str.push('.');
            my_str.push_str(&parse_json["table"][i]["columns"][y][1].to_string());
            intermediary_vector.push(my_str.clone());
            final_proj.push(my_str); 
        }
        key.push(parse_json["table"][i]["table_name"].to_string());
        //println!("{:?}",intermediary_vector);
        //println!("{}",parse_json["table"][i]["table_name"]);
        let mut open_file:CSVFile = operator::open_relation(parse_json["table"][i]["table_name"].to_string(), parse_json["table"][i]["table_name"].to_string())?;
        open_file.projection(intermediary_vector);
        dictionnary.insert(parse_json["table"][i]["table_name"].to_string(),open_file);
    }
    for i in 1..key.len(){
        //gestion erreur avec le clone
        //let mut test =dictionnary.get_mut(&key[0]).ok_or_else(); //.expect("Get error ").clone();

        let mut test : CSVFile;
        match dictionnary.get_mut(&key[0]){
            Some(res) => test = res.clone(),
            _ => return Err(Box::from("Error : Runner : Key 0 doesn't exist"))
        };

        //let test2 =dictionnary.get(&key[i]).expect("Get error");
        let test2 : CSVFile;
        match dictionnary.get_mut(&key[i]){
            Some(res) => test2 = res.clone(),
            _ => return Err(Box::from("Error : Runner : Key i doesn't exist"))
        };

        test.cartesian_product(&test2);
        dictionnary.insert(key[0].to_string(), test);
    }
    dictionnary[&key[0]].to_file();
    //return "../data/transferFile/result.csv" ;
    Ok("../data/transferFile/result.csv")
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




