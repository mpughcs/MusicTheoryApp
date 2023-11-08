extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality, self};
use text_io::scan;
use std::{io, fs};
use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};
mod chord_progression;
use crate::chord_progression::chord_progression::*;
#[macro_use] extern crate rocket;

// function to display helpful information to the user
fn help(){
println!("
    Notes as strings:
    'C' | 'c' => C,
    'Cs' | 'cs' => C#,
    'D' | 'd' => D,
    'Ds' | 'ds' => D#,
    'E' | 'e' => E,
    'F' | 'f' => F,
    'Fs' | 'fs' => F#,
    'G' | 'g' => G,
    'Gs' | 'gs' => G#,
    'A' | 'a' => A,
    'As' | 'as' => A#,
    'B' | 'b' => B,
    
    Modes of the major scale:
    'Ionian' | 'ionian' => Ionian,
        -contains the notes: 1 2 3 4 5 6 7

    'Dorian' | 'dorian' => Dorian,
        -contains the notes: 1 2 b3 4 5 6 b7

    'Phrygian' | 'phrygian' => Phrygian,
        -contains the notes: 1 b2 b3 4 5 b6 b7

    'Lydian' | 'lydian' => Lydian,
        -contains the notes: 1 2 3 #4 5 6 7

    'Mixolydian' | 'mixolydian' => Mixolydian,
        -contains the notes: 1 2 3 4 5 6 b7

    'Aeolian' | 'aeolian' => Aeolian,
        -contains the notes: 1 2 b3 4 5 b6 b7

    'Locrian' | 'locrian' => Locrian,
        -contains the notes: 1 b2 b3 4 b5 b6 b7
    
    Chord extensions/numbers:
        Triad,
        Seventh,
        MajorSeventh,
        Ninth,
        Eleventh,
        Thirteenth,
    
    Chord qualities:
        Major,
        Minor,
        Diminished,
        Augmented,
        HalfDiminished,
        Dominant,
        Suspended2,
        Suspended4,
    "
    
);
}

// simple enter to continue function
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


// derive debug for all enums
#[derive(Debug)]
enum EntryError {
    TonicError,
    ModeError,
    QualityError,
    ChordNumberError
   
}

// This is a helper function that makes inline input easier.
fn inline_user_input(prompt: &str) -> String {
    let to_return;
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    scan!("{}\n", to_return);
    return to_return;
}
// match_tonic calls the PitchClass::from_regex() method to check if the user input is a valid pitch class
fn match_tonic(input: String) -> Result<PitchClass, EntryError>{
    match PitchClass::from_regex(input.as_str()){
        Ok(_) => Ok(PitchClass::from_regex(input.as_str()).unwrap().0),
        _ => Err(EntryError::TonicError),
    }
}

// match_mode calls the Mode::from_regex() method to check if the user input is a valid mode
fn match_mode(input:String)-> Result<Mode,EntryError>{
    match Mode::from_regex(input.as_str()){
        Ok(_) => Ok(Mode::from_regex(input.as_str()).unwrap().0),
        _ => Err(EntryError::ModeError),
    }
}
// match_chord_number calls the ChordNumber::from_regex() method to check if the user input is a valid chord number
fn match_chord_number(input: String)-> Result<ChordNumber,EntryError>{
    
    match ChordNumber::from_regex(input.as_str()){
        Ok(_)=> Ok(ChordNumber::from_regex(input.as_str()).unwrap().0),

        _ => Err(EntryError::ChordNumberError),
    }
}

fn match_quality(input:String)-> Result<ChordQuality,EntryError>{
    match ChordQuality::from_regex(input.as_str()){
        Ok(_) => Ok(ChordQuality::from_regex(input.as_str()).unwrap().0),
        _ => Err(EntryError::QualityError)
    }
}
fn get_chord_number()->ChordNumber{
    loop {
        let usr=inline_user_input("Enter Extension of the chord (triad, seventh, eleventh): ");
        let num=match_chord_number(usr);
        match num{
            Ok(_) => return num.unwrap(),
            Err(_) => println!("{}","Invalid extension. Try again.".red()),
        }
    }
}
fn get_quality()->ChordQuality{
    loop{
        let usr=inline_user_input("Enter Chord Quality (major, minor, diminished): ");
        let qual= match_quality(usr);
        match qual{
            Ok(_) =>  return qual.unwrap(),
            Err(_) => println!("{}","Invalid quality. Try again.".red()),
        }
    }
}



// get_mode calls get_mode to check if the user input is a valid mode, reprompts if not
fn get_mode()-> Mode{
    loop{
        let usr=inline_user_input("Enter the Mode of the Scale: ");
        let mode = match_mode(usr);
        match mode{
            Ok(_) => return mode.unwrap(),
            Err(_) => println!("{}","Invalid mode. Try again.".red()),
        }
    }
}


// get_mode calls get_tonic to check if the user input is a valid mode, reprompts if not
fn get_tonic() -> PitchClass{
    loop{
        let usr=inline_user_input("Enter the root note: ");
        let tonic = match_tonic(usr);
        match tonic{
            Ok(_) => return tonic.unwrap(),
            Err(_) => println!("{}","Invalid tonic. Try again.".red()),
        }
    }
}


// allow unused because this is a helper function
// #[allow(unused)]
// scale_as_vector returns a vector of notes from a scale
fn scale_as_vector(tonic: PitchClass,mode: Mode, direction: String ) -> Vec<Note> {
    let scale_direction: Direction;
    if direction.to_uppercase() =="ASC"{
        scale_direction = Direction::Ascending;
    } else if direction.to_uppercase() =="DESC"{
        scale_direction = Direction::Descending;
    } else {
        println!("Invalid direction. Defaulting to ascending.");
        scale_direction = Direction::Ascending;
    }
    let scale1 = Scale::new(ScaleType::from_mode(mode), tonic, 4,Some(mode),scale_direction).unwrap();
    return scale1.notes();
}


// print_scale prints a vector of notes
fn print_scale(scale: &Vec<Note>) {
    for note in scale {
        println!("{}", note);
    }
}

// write_notes_to_file: so that we can write user output 
fn write_notes_to_file(scale: &Vec<Note>) {
    let mut file = std::fs::File::create("../scale.txt").unwrap();
    for note in scale {
        writeln!(file, "{},{}", note,note.octave).unwrap();
    }
}
// append_notes_to_file: so that we can chords to the file iteratively
fn append_notes_to_file(notes: &Vec<Note>, file_name: &str) {
   
    let mut file = fs::OpenOptions::new().write(true).append(true).open(file_name).unwrap();
    for note in notes {
        writeln!(file, "{},{}", note,note.octave).unwrap();
    }
}

// takes in the chord progression struct and writes it to a file with the given name
fn write_prog_to_file(to_write:&ChordProgression, file_name:&str){
    let mut i=0;
    // if file doesn't exist, create it
    let destination = format!("../progressions/{}.txt",file_name);

    let file = std::fs::File::create(destination.clone()).unwrap();
    let mut file = fs::OpenOptions::new().write(true).append(true).open(destination.clone()).unwrap();
    
    writeln!(file,"{}", to_write.get_num_chords()).unwrap();
    writeln!(file,"-").unwrap();
    while i < to_write.get_num_chords(){
        let iterator = i as usize;
        let notes_to_add = chord_as_vector(to_write.chord_progression[iterator].root, to_write.chord_progression[iterator].quality,to_write.chord_progression[iterator].number);
        let note_count=notes_to_add.len();
        writeln!(file,"{}", note_count).unwrap();
        append_notes_to_file(&notes_to_add,&destination);
        if i!= to_write.get_num_chords()-1{
            writeln!(file,"-").unwrap();
        }
        i+=1;
    }
    println!("Progression written to {}",destination.green());
    pause();
}



fn view_notes_in_scale(){

    let tonic= get_tonic();

    let mode:Mode = get_mode();
    let direction:String = inline_user_input("Enter the direction of the scale (asc/desc): ");
    let scale_direction: Direction;
    let user_notes = &scale_as_vector(tonic,mode,direction);
    // print all notes in user_notes followed by a newline
    
    print_scale(user_notes);
    write_notes_to_file(user_notes);
    println!("Scale written to {}","../scale.txt".green());
    pause();
}

// takes in the chord progression struct and writes it to a file with the given name
// fn write_prog_to_file(to_write:&ChordProgression, file_name:&str){
//     let mut i=0;
//     // if file doesn't exist, create it
//     let destination = format!("../progressions/{}.txt",file_name);

//     let file = std::fs::File::create(destination.clone()).unwrap();
//     let mut file = fs::OpenOptions::new().write(true).append(true).open(destination.clone()).unwrap();
    
//     writeln!(file,"{}", to_write.get_num_chords()).unwrap();
//     writeln!(file,"-").unwrap();
//     while i < to_write.get_num_chords(){
        // let iterator = i as usize;
        // let notes_to_add = chord_as_vector(to_write.chord_progression[iterator].root, to_write.chord_progression[iterator].quality,to_write.chord_progression[iterator].number);
//         let note_count=notes_to_add.len();
//         writeln!(file,"{}", note_count).unwrap();
//         append_notes_to_file(&notes_to_add,&destination);
//         if i!= to_write.get_num_chords()-1{
//             writeln!(file,"-").unwrap();
//         }
//         i+=1;
//     }
//     println!("Progression written to {}",destination.green());
//     pause();
// }


// takes in 
// returns a vector containing the notes in a scale 

fn view_notes_in_chord(){
    let root:PitchClass= get_tonic();
    let qual = get_quality();
    let extension = get_chord_number();
    // let quality: &str = &quality[..]; 
    //use match to check if the root is a valid note
  
    // store quality as a Quality converted from &str regex to Quality
   
    let chord = Chord::new(root, qual, extension);
    

    let user_notes:&Vec<Note>= &chord.notes();
    for note in user_notes {
        println!("{}", note);
    }
    write_notes_to_file(user_notes);
    pause();

}

fn chord_as_vector(root:PitchClass,quality:ChordQuality,extension:ChordNumber)-> Vec<Note>{
    let chord = Chord::new(root,quality,extension);
    let to_return:Vec<Note>=chord.notes();
    return to_return
}


// entry point for the program

fn display_options(){
    print! ("\x1B[2J\x1B[1;1H"); 
    println!(
        "{}\n > {}\n > {}\n",
        format!("Welcome to Notation!").bold().green().italic(),
        format!("This program will allow you to interact with the Rust Music Theory library.").green(),
        format!("You can create notes, scales, chords!").green(),
    );
   
    loop{
        println!("Choose from one of the following options: \n");
        println!("  1. view notes in a scale");
        println!("  2. view notes in a chord");
        println!("  3. create chord progression");
        println!("  4. help");
        println!("  5. exit");
        //get command line input
        let input = inline_user_input( ":");
        match input.as_str(){
            "1" => view_notes_in_scale(),
            "2" => view_notes_in_chord(),
            // "3" => create_progression(),
            "4" => help(),
            "5" => break,
            _ => println!("{}", "Invalid Input, Try again".red())
        }
    }
    

}

// as a rest api, we don't need to output to the console anymore
// this should be changed to create_progression(name: &str, chords: Vec<Chord>) -> ChordProgression as json
// and then we can return the json to the user


// fn main() {
//     display_options();
// }
#[get("/scale/<tonic>/<mode>/<direction>")]
fn scale(tonic: &str, mode: &str, direction: String ) -> String {
    let mut to_return = String::new();
    for note in scale_as_vector(PitchClass::from_str(tonic).unwrap(), Mode::from_regex(mode).unwrap().0, direction) {
        to_return.push_str(&format!("{}\n", note));
    }
    return to_return;
}

#[get("/chord/<root>/<quality>/<extension>")]
fn get_chord(root: &str, quality: &str, extension: &str) -> String {
    let mut to_return = String::new();
    let chord = chord_as_vector(PitchClass::from_str(root).unwrap(), ChordQuality::from_regex(quality).unwrap().0, ChordNumber::from_regex(extension).unwrap().0);
    to_return.push_str(&format!("{}\n", chord.len()).to_string());
    for note in chord {
        to_return.push_str(&format!("{}, {}\n", note, note.octave).to_string());
    }
    to_return

}

#[get("/")]
fn intstructions() -> String {
    return "Welcome to the Rust Music Theory API! To use this API, you can use the following endpoints:\n
    /scale/<tonic>/<mode>/<direction>\n
    /chord/<root>/<quality>/<extension>\n".to_string();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![scale, intstructions,get_chord])
}