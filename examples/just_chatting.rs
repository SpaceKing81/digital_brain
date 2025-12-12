
/*

README: Runs a simulation of the brain itself, color coded for the neurons and axon connections.
Colors:
    - Red connections mean inhibitory
    - Green axons mean exitory
    - Gray mean 0, null, no effect
    - Blue neurons are output neurons

Capable of communicating with the brain using keyboard. Type whatever you want, and you can see
what Spirion says. White number in the top middle represents the frame-rate

Your choice on how large the brain is, default is 500 neurons

TO RUN - Paste into terminal the following line:
cargo run --example just_chatting

*/

const STARTING_NEURONS:Option<u32> = Some(1000);











use std::collections::HashMap;
use macroquad::prelude::*;
use digital_brain::{Spirion,MAX_THRESHOLD};

fn window_conf() -> Conf {
    Conf {
        window_title: "Brain Simulation".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}



const STARTING_INPUTS:u128 = 42;
const STARTING_OUTPUTS:u32 = 42;
const SPIRION_TEXT_COLOR:Color = Color::new(0.9, 0.3, 0.0, 0.75);
const CLICKABLE_KEYS:[KeyCode;42] = [
    KeyCode::Space,
    KeyCode::Apostrophe,
    KeyCode::Comma,
    KeyCode::Period,
    KeyCode::A,
    KeyCode::B,
    KeyCode::C,
    KeyCode::D,
    KeyCode::E,
    KeyCode::F,
    KeyCode::G,
    KeyCode::H,
    KeyCode::I,
    KeyCode::J,
    KeyCode::K,
    KeyCode::L,
    KeyCode::M,
    KeyCode::N,
    KeyCode::O,
    KeyCode::P,
    KeyCode::Q,
    KeyCode::R,
    KeyCode::S,
    KeyCode::T,
    KeyCode::U,
    KeyCode::V,
    KeyCode::W,
    KeyCode::X,
    KeyCode::Y,
    KeyCode::Z,
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
    KeyCode::Enter,
    KeyCode::Backspace
];


#[macroquad::main(window_conf)]
async fn main() {
    // Get name
    let user_name="User".to_string();

    // Initialize the brain
    // From scratch
    println!("Starting simulation...");
    let (
        mut brain, 
        inputs, 
        outputs
    ) = Spirion::spin_up_new(
        STARTING_NEURONS,
        STARTING_INPUTS, 
        STARTING_OUTPUTS,
        true,
    );
    let mut thought_text:Vec<String> = Vec::new();
    let mut type_text:Vec<String> = vec![String::new()];
    
    // From bin
    // let (
    //     mut brain, 
    //     inputs, 
    //     outputs
    // ) = Spirion::build_from_bin(
    //     "Spirion_speaking.bin",
    // );
    
    // Main loop
    loop {
        // Handle Ending
        if is_key_down(KeyCode::Escape) {
            println!("Saving Brain...");
            brain.save_as_bin("Spirion_speaking.bin");
            println!("Terminating Brain...");
            break;
        }
        // Get keystrokes
        let data = keys_to_input(&inputs);
        type_text = type_to_fused_text(type_text);
        
        
        // Update the brain
        brain.brain_input(data.inputs);

        let raw_output = output_to_keys(
            &outputs,
            brain.tick(Some(1))
        );
        let refined_output = convert_to_strings(raw_output);
        let mut bridge = String::new();
        if let Some(mut tbadded) = refined_output {
            if let Some(middle_last) = tbadded.first() {
                if let Some(first_middle) = thought_text.last() {
                    bridge.push_str(&first_middle);
                    bridge.push_str(&middle_last);
                }
            }
            tbadded.remove(0);
            thought_text.pop();
            thought_text.push(bridge);
            thought_text.append(&mut tbadded);
        }
        

        // Drawing a frame
        { 

        // Clear the screen
        clear_background(BLACK);
        while type_text.len() > 60 {
            type_text.remove(0);
        }
        while thought_text.len() > 60 {
            thought_text.remove(0);
        }

        // Update and draw neurons and axons
        brain.render();
        // Draw Text
        draw_text(
            &format!("{}", get_fps()),
            screen_width()/2.0,
            20.,
            20.,
            WHITE,
        );
        draw_text(
                &format!("{}", user_name),
                screen_width() - 500.,
                25.0,
                20.,
                WHITE,
            );
        for i in 0..type_text.len() {
            draw_text(
                &format!("{}",type_text[i]),
                screen_width() - 500.,
                i as f32 * 12. + 37.,
                20.,
                WHITE,
            );
        }
        draw_text(
                &format!("Spirion"),
                25.0,
                25.0,
                20.,
                SPIRION_TEXT_COLOR,
            );
        for i in 0..thought_text.len() {
            draw_text(
                &format!("{}", thought_text[i]),
                25.0,
                i as f32 * 12. + 37.,
                20.,
                SPIRION_TEXT_COLOR,
            );
        }

        }
        // wait_seconds(1.0);
        // Render the frame
        next_frame().await;
    }
}

struct Data {
    inputs:Option<Vec<(u128,i32)>>,
    // outputs:Option<Vec<KeyCode>>,
}

fn keys_to_input(inputs:&Vec<u128>) -> Data {
    let mut a = 0;
    let mut input = Vec::new();
    for &i in &CLICKABLE_KEYS {
        if is_key_released(i) {
            input.push((inputs[a],MAX_THRESHOLD));
        }
        a += 1;
    }
    if !input.is_empty() {
        return Data {inputs:Some(input)}
    }
    Data {
        inputs:None,
        // outputs:None,
    }
}
fn output_to_keys(outputs:&Vec<u32>, thoughts:Option<Vec<u32>>) -> Option<Vec<KeyCode>> {
    if let Some(thoughts) = thoughts {
        let mut tokens:HashMap<u32,Vec<usize>> = HashMap::new();
        
        for i in 0..thoughts.len() {
            if let Some(letter) = tokens.get_mut(&thoughts[i]) {
                letter.push(i);
            } else {
                tokens.insert(thoughts[i], vec![i]);
            }
        }

        let mut key_converted: Vec<(usize,KeyCode)> = Vec::new();
        for (letter_code, spot) in tokens {
            let letter = convert_to_key(outputs, letter_code);
            for s in spot {
                key_converted.push((s,letter));
            }
        }
        key_converted.sort_by_key(|&(idx, _)| idx);
        let (_, outputs):(Vec<usize>,Vec<KeyCode>) = key_converted.into_iter().unzip();
        return Some(outputs);
    } None
}
fn convert_to_key(outputs:&Vec<u32>, letter:u32) -> KeyCode {
    for i in 0..outputs.len() {
        if outputs[i] == letter {
            return CLICKABLE_KEYS[i]
        }
    }
    unreachable!()
}
fn keycode_to_char(key: KeyCode) -> Option<char> {
    use KeyCode::*;
    // basic a–z
    let base:char = match key {
        A  => 'a',
        B  => 'b',
        C  => 'c',
        D  => 'd',
        E  => 'e',
        F  => 'f',
        G  => 'g',
        H  => 'h',
        I  => 'i',
        J  => 'j',
        K  => 'k',
        L  => 'l',
        M  => 'm',
        N  => 'n',
        O  => 'o',
        P  => 'p',
        Q  => 'q',
        R  => 'r',
        S  => 's',
        T  => 't',
        U  => 'u',
        V  => 'v',
        W  => 'w',
        X  => 'x',
        Y  => 'y',
        Z  => 'z',
        Key0  => '0',
        Key1  => '1',
        Key2  => '2',
        Key3  => '3',
        Key4  => '4',
        Key5  => '5',
        Key6  => '6',
        Key7  => '7',
        Key8  => '8',
        Key9  => '9',

        Space => ' ',
        Comma => ',',
        Period => '.',
        Apostrophe => '\'',
        _ => return None,
    };

    Some(base)
}
fn convert_to_strings(raw:Option<Vec<KeyCode>>) -> Option<Vec<String>> {
    if raw == None {return None}
    let mut refined:Vec<String> = vec![String::new()];

    for i in raw.unwrap() {
        if let Some(ch) = keycode_to_char(i) {
            if let Some(current) = refined.last_mut() {
                current.push(ch);
            }
        } else {
            match i {
                KeyCode::Backspace => {
                    if let Some(current) = refined.last_mut() {
                        current.pop();
                    } else {
                        refined.pop();
                    }
                },
                KeyCode::Enter => refined.push(String::new()),
                _ => panic!("Something slipped through the keycode->char fn")
            }
        }
    }
    if refined.is_empty() {
        return None;
    }
    Some(refined)
}
fn type_to_fused_text(mut current_text:Vec<String>) -> Vec<String>{
    let key = get_keys_released();
    let keys:Vec<char> = key.into_iter().map(|key|
        if let Some(ch) = keycode_to_char(key) {
            ch
        } else if key == KeyCode::Enter || key == KeyCode::Backspace {
            if key == KeyCode::Enter {'+'}
            else {'-'}
        } else {
            '|'
        }
    ).collect();
    let mut full_chop = false;
    for i in keys {
        if i == '|' {continue;}
        if i == '+' {current_text.push(String::new());}
        if let Some(current_string) = current_text.last_mut() {
            if current_string.is_empty() && i == '-' {
                full_chop = true;
            }
            if i == '-' && !current_string.is_empty() {
                current_string.pop();
            }
            if i != '+' && i != '-' && i != '|' {
                current_string.push(i);
            }
        }
        if full_chop {
            current_text.pop();
        }
        full_chop = false; 
    }
    current_text
}


/*
TODO:
- forgot this exists, haven't looked at it in forever. Last time looking at this:
    - May 20 2025
    - May 21 2025
    - Oct 24 2025


NOTES:
- threshold value can fluctuate between 30 and 70 in a neuron. The node records the natural threshold value,
and this can then change based on average firing rate. Once the node fires, the threshold value is updated based
on the average rate, and then artificialy boosted by the reload function, which is constant. 

UNKNOWN GOAL: Each node operates individually, running basic calculations continuously like 
tick and memory increments, and modulating thresholds post firing and slow alteration of axon weights with disuse.
Firing functions kick in and the whole system gets activated from semi-dormant state when surpassing threshold,
pushing an update to all the connected nodes who then possibly update as well.

*/