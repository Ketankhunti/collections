use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
   /* let a = [1,2,3];
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);*/
/*
    let mut  v = vec![1,2,3,4,5];
    let third = &v[2];
    println!("{}",third);
    v.push(6);

    match v.get(20){
        None => println!("There is no element exists for this index"),
        Some(third) => println!("{}",third)
    }*/
/*    let v = vec![
        SpreadSheetCell::Int(5050),
        SpreadSheetCell::Float(178.69),
            SpreadSheetCell::Text(String::from("ketan mer"))
    ];
    match v[0] {
        SpreadSheetCell::Int(k) => println!("{}",k),
        _ => println!("not ketan mer")
    }
    println!("{:#?}",v[0]);*/
/*
    let string = String::from("नमस्ते");
    for i in string.bytes(){
        println!("{:#?}",i);
    }

    for i in string.chars(){
        println!("{:#?}",i);
    }

    for g in string.graphemes(true){
        println!("{}",g);
    }*/
/*
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(&blue,5050);
    scores.insert(&yellow,6060);

    let score = scores.get(&blue);

    println!("{:#?}",score);

    for (key,value) in scores{
        println!("{:#?} , {:#?}",key,value);
    }*/

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for i in text.split_whitespace(){
        let count = map.entry(i).or_insert(0);
        *count+=1;
    }
    println!("{:#?}",map);

}

