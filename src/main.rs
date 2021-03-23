use std::process::exit;

const TONES: [&'static str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

const B_DIGIT: i32 = 98;
const HASH_DIGIT: i32 = 35;

fn parse_tones() -> Vec<i32> {
    let mut string_tones_line = String::new();
    std::io::stdin().read_line(&mut string_tones_line).unwrap();
    let string_tones_list = string_tones_line.split_whitespace();
    let mut tones = Vec::new(); 

    for string_tone in string_tones_list.rev() {
        if string_tone.is_empty() {
            continue;
        }

        let mut final_tone;
        let mut chars = string_tone.chars();
        let first_char = chars.next().unwrap();

        match TONES.iter().position(|&r| r == first_char.to_string()) {
            Some(value) => final_tone = value as i32,
            None => exit(1)
        }

        for char in chars {
            let digit: i32 = char as i32;

            if digit == B_DIGIT {
                final_tone = (final_tone - 1) % 12;
            } else if digit == HASH_DIGIT {
                final_tone = (final_tone + 1) % 12;
            } else {
                exit(2)
            }
        }
        tones.push(final_tone);
    }
    tones
}

fn print_fretboard(tones: &[i32], highlights: &[i32]) {
    for tone_orig in tones {
        let mut tone = *tone_orig;
        for _ in 0..=24 {
            if highlights.contains(&tone) {
                let output = TONES[tone as usize];
                if output.len() == 2 {
                    print!("| {} ", output);
                } else {
                    print!("| {}  ", output);
                }
            } else {
                print!("|    ");
            }
            tone = (tone + 1) % 12;
        }
        println!("|");
    }
    for _ in 0..=24 {
        print!("-----");
    }
    println!("-");
    for i in 0..=24 {
        if i > 9 {
            print!("| {} ", i);
        } else {
            print!("| {}  ", i);
        }
    }
    println!("|");
}

fn print_modes_chords(tones: &[i32], highlights: &[i32]) {
    if highlights.len() != 7 {
        println!("Only works with scales with 7 tones");
        return;
    }

    let mut half_steps = Vec::new();
    for i  in 0..7 {
        if (highlights[i] + 1) % 12 == highlights[(i + 1) % 7] {
            half_steps.push(highlights[i]);
        }
    }
    //dbg!(&half_tones);

    if half_steps.len() != 2 {
        println!("Found one or more than two tones with half step");
        return;
    }

    let interval_i;
    let difference = half_steps[1] - half_steps[0];
    if difference == 7 {
        interval_i = (half_steps[1] + 1) % 12;
    } else if difference == 5 {
        interval_i = (half_steps[0] + 1) % 12;
    } else {
        println!("Difference between half tones is not 5 nor 7");
        return;
    }

    let interval_ii = (interval_i + 2) % 12;
    let interval_iii = (interval_i + 4) % 12;
    let interval_iv = (interval_i + 5) % 12;
    let interval_v = (interval_i + 7) % 12;
    let interval_vi = (interval_i + 9) % 12;
    let interval_vii = (interval_i + 11) % 12;

    let degree_i = TONES[interval_i as usize];
    let degree_ii = TONES[interval_ii as usize];
    let degree_iii = TONES[interval_iii as usize];
    let degree_iv = TONES[interval_iv as usize];
    let degree_v = TONES[interval_v as usize];
    let degree_vi = TONES[interval_vi as usize];
    let degree_vii = TONES[interval_vii as usize];

    println!("Ionian     (I)  : {}", degree_i);
    println!("Dorian     (II) : {}", degree_ii);
    println!("Phrygian   (III): {}", degree_iii);
    println!("Lydian     (IV) : {}", degree_iv);
    println!("Mixolydian (V)  : {}", degree_v);
    println!("Aeolian    (VI) : {}", degree_vi);
    println!("Locrian    (VII): {}", degree_vii);
    println!();

    println!("Pentatonics: {} {} {} {} {}", degree_i, degree_ii, degree_iii, degree_v, degree_vi);
    print_fretboard(tones, &[interval_i, interval_ii, interval_iii, interval_v, interval_vi]);
    println!();

    println!("{} major: {} {} {}", degree_i, degree_i, degree_iii, degree_v);
    print_fretboard(tones, &[interval_i, interval_iii, interval_v]);
    println!();

    println!("{} minor: {} {} {}", degree_ii, degree_ii, degree_iv, degree_vi);
    print_fretboard(tones, &[interval_ii, interval_iv, interval_vi]);
    println!();

    println!("{} minor: {} {} {}", degree_iii, degree_iii, degree_v, degree_vii);
    print_fretboard(tones, &[interval_iii, interval_v, interval_vii]);
    println!();

    println!("{} major: {} {} {}", degree_iv, degree_iv, degree_vi, degree_i);
    print_fretboard(tones, &[interval_iv, interval_vi, interval_i]);
    println!();

    println!("{} major: {} {} {}", degree_v, degree_v, degree_vii, degree_ii);
    print_fretboard(tones, &[interval_v, interval_vii, interval_ii]);
    println!();

    println!("{} minor: {} {} {}", degree_vi, degree_vi, degree_i, degree_iii);
    print_fretboard(tones, &[interval_vi, interval_i, interval_iii]);
    println!();

    println!("{} diminished: {} {} {}", degree_vii, degree_vii, degree_ii, degree_iv);
    print_fretboard(tones, &[interval_vii, interval_ii, interval_iv]);
    println!();
}

fn main() {
    println!("Enter guitar string tones (from low to high)");
    let tones = parse_tones();
    //dbg!(&tones);

    println!("Enter guitar string tones to highlight");
    let mut highlights = parse_tones();
    highlights.sort();
    highlights.dedup();
    //dbg!(&highlights);

    println!();
    println!("*** All tones ***");
    print_fretboard(tones.as_slice(), highlights.as_slice());
    
    println!();
    println!("*** Scales and Chords ***");
    print_modes_chords(tones.as_slice(), highlights.as_slice());
}

// E A D G B E
// A A# C D E F G