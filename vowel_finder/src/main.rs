
use std::{io, vec};
use std::time::{Instant, Duration};


fn again() {

    println!("Would you like to continue? y/n");
    let mut ye_na: String = String::new();

    let _  = io::stdin().read_line(&mut ye_na);

    let ye_na: String = ye_na.trim().to_lowercase();
    match ye_na.as_str() {
        "y" => {
            print!("\x1B[2J\x1B[1;1H");
            main()
        }
        "n" => return,
        _ => {
            println!("Invalid answer! Type 'y' or 'n'!!");
            again()   
        }
    }
}

fn main() {
    // Variable to store user input
    let mut sentence: String = String::new().to_lowercase();

    // variables to store count data
    let mut a: i32 = 0;
    let mut e: i32 = 0;
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut u: i32 = 0;
    let mut nums: i32 = 0;
    let mut punc: i32 = 0;
    let mut consonants: i32 = 0;
    let mut count: i32 = 0;
    
    //Vectors to store indexes
    let mut aa: Vec<i32> = vec![];
    let mut ee: Vec<i32> = vec![];
    let mut ii: Vec<i32> = vec![];
    let mut oo: Vec<i32> = vec![];
    let mut uu: Vec<i32> = vec![];



    print!("\x1B[2J\x1B[1;1H");
    println!("\nEnter text here: ");
    let _ = io::stdin().read_line(&mut sentence);

    let start_timer: Instant = Instant::now();

    // looping over each character in the string and evaluating for vowel, consonant, number or punctuation.
    sentence.chars().for_each(|x| {

        if x == 'a'{
            a += 1;
            aa.push(count);
            count +=1;
        } else if x == 'e' {
            e += 1;
            ee.push(count);
            count +=1;
        } else if x == 'i' {
            i += 1;
            ii.push(count);
            count +=1;
        } else if x == 'o' {
            o += 1;
            oo.push(count);
            count +=1;
        } else if x == 'u' {
            u += 1;
            uu.push(count);
            count +=1;
        } else if x.is_numeric() {
            nums += 1;
            count +=1;
        } else if x.is_ascii_punctuation() {
            punc += 1;
        } else if x >= 'a' && x <= 'z' {
            consonants += 1;
            count +=1;
        } 
    });

    let duration: Duration = start_timer.elapsed();
    let total:i32  = a + e + i + o + u;
    let vowel_counts: String = format!("\n\t a = {}, found at indexes: {:?}\n\t e = {}, found at indexes: {:?}\n\t i = {}, found at indexes: {:?}\n\t o = {}, found at indexes: {:?}\n\t u = {}, found at indexes: {:?}\n", a, aa, e, ee, i, ii, o, oo, u, uu);
    let total_counts: String = format!("\nThe total lenght of you sentence is {} characters, it has {} vowels, {} consonants. {} numbers, and {} punctuations\n", sentence.trim().replace(" ", "").chars().count(), total, consonants, nums, punc);
    let time_run: String = format!("Calculated in {:?}\n", duration);

    println!("{}{}{}", vowel_counts, total_counts, time_run);

    //continue? 
    again();
}