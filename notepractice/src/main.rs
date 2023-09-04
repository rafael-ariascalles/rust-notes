use std::io;
use std::io::Write;

// Array than contain all the musical notes
const NOTES: [&str; 12] = ["C", "C# / Db", "D", "D# / Eb", "E", "F","F# / Gb", "G", "G# / Ab", "A", "A# / Bb", "B"];

fn main() {
    
    println!("Entern the number of iterations:");
    let mut iterations = String::new();
    io::stdin().read_line(&mut iterations).expect("Failed to read line");
    
    println!("Enter the time you want to wait in seconds between intervals: ");
    let mut time_interval = String::new();
    io::stdin().read_line(&mut time_interval).expect("Failed to read line");
    
    //convert the string to u32
    let time_interval: u32 = time_interval.trim().parse().expect("Please type a number!");
    let iterations: u32 = iterations.trim().parse().expect("Please type a number!");

    println!("Starting the program...");
    //loop for the number of iterations given by the user
    for _i in 0..iterations {
        //call the function that print the note
        println!("Iteration: {}", _i);
        print_note();
        //print the time interval in 5 seconds
        for _t in 0..time_interval {
            print!("{} ", time_interval - _t);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("... starting new note");
        
    }
}

fn print_note(){
    let note_position: usize  = rand::random::<usize>() % NOTES.len();
    //return the selected notes
    println!("#########################  {} ", NOTES[note_position]);
    println!("#########################  {} ", NOTES[note_position]);
    println!("#########################  {} ", NOTES[note_position]);
    std::thread::sleep(std::time::Duration::from_secs(3));
    
}   