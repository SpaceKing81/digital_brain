use std::collections::HashMap;

use macroquad::prelude::*;
use digital_brain::Brain;
use digital_brain::MAX_THRESHOLD;

/// Calculate Modulus operations
// fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b}

fn window_conf() -> Conf {
    Conf {
        window_title: "Brain Simulation".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}


const STARTING_NEURONS:u32 = 100;
const STARTING_INPUTS:u128 = 32;
const STARTING_OUTPUTS:u32 = 32;
const CLICKABLE_KEYS:[KeyCode;32] = [
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
    KeyCode::Enter,
    KeyCode::Backspace
];
// const IDEAL_TPS:f64 = 60.0;

#[macroquad::main(window_conf)]
async fn main() {
    
    // Initialize the brain
    println!("Starting simulation...");
    let mut ticks = 0.0;
    let center = Vec2::new(screen_width()/2.0, screen_height()/2.0);
    let (
        mut brain, 
        inputs, 
        outputs
    ) = Brain::spin_up_new(
        STARTING_NEURONS,
        STARTING_INPUTS, 
        STARTING_OUTPUTS
    );
    let mut text: Vec<KeyCode> = Vec::new();
    
    // Main loop
    loop {
        // Handle Ending
        if is_key_down(KeyCode::Escape) {
            println!("Terminating Brain...");
            break;
        }
        let data = keys_to_input(&inputs);

        // Update the brain
        brain.brain_input(data.inputs);

        if let Some(mut output) = 
        output_to_keys(
            &outputs,brain.tick(Some(29))
        )
        .outputs {
            text.append(&mut output);
        }
        

        // Drawing a frame
        { 

        // Clear the screen
        clear_background(BLACK);
        // Update and draw neurons and axons
        brain.render(center);
        // Draw FPS and other info
        draw_text(
            &format!("Clock {}", brain.clock),
            20.,
            20.,
            20.,
            WHITE,
        );
        draw_text(
            &format!("{:?}", text),
            20.,
            40.,
            20.,
            WHITE,
        );

        }
        // Render the frame
        next_frame().await;
    }
}

struct Data {
    inputs:Option<Vec<(u128,i32)>>,
    outputs:Option<Vec<KeyCode>>,
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
        return Data {inputs:Some(input), outputs:None}
    }
    Data {
        inputs:None,
        outputs:None,
    }
}
fn output_to_keys(outputs:&Vec<u32>, thoughts:Option<Vec<u32>>) -> Data {
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
    return Data {inputs:None,outputs:Some(outputs)};
}
    
    Data {
        inputs:None,
        outputs:None,
    }  
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

        Space => ' ',
        Comma => ',',
        Period => '.',
        Apostrophe => ',',
        _ => return None,
    };

    Some(base)
}
fn keycode_to_action(key: KeyCode) -> Option<char> {todo!()}



/*
TODO:
- forgot this exists, haven't looked at it in forever. Last time looking at this:
    - May 20 2025
    -


NOTES:
- threshold value can fluctuate between 30 and 70 in a neuron. The node records the natural threshold value,
and this can then change based on average firing rate. Once the node fires, the threshold value is updated based
on the average rate, and then artificialy boosted by the reload function, which is constant. 

UNKNOWN GOAL: Each node operates individually, running basic calculations continuously like 
tick and memory increments, and modulating thresholds post firing and slow alteration of axion weights with disuse.
Firing functions kick in and the whole system gets activated from semi-dormant state when surpassing threshold,
pushing an update to all the connected nodes who then possibly update as well.

*/