# Brain
A fun little project of making a digital brain (Named Spirion)

Originally was just gonna be a small little project, an idea I had while working on my Ants project and a nice distraction.

Ended up spending an obsene amount of time on it testing and iterating. Some tears were shed, but can confidently say that my work on the ant project vastly prepared me for this one, which was good because another project spawned off this one as a result anyways. Wooo.

Only now got around to actually making it a repository, because it was feeling more and more like an actual project rather then a side quest. But yeah, we are here now, so progress can be adequetly tracked.

And now, much later, can confidently say that work here has vastly prepared me for my Ants project as well, so thats wonderful!

----------

How to use:
(Techinically its a library, so eventually I'll get this into crates.io when its not abhorent.)

Download the necessary files, import {use digital_brain::Spirion;}
Make use of the various tools avaible. Right now there are not a lot.




Can run example files as well, currently there are 2 (Technically)
Run in terminal:
  - cargo run --example just_chatting
  - cargo run --example pong
  - cargo run --example normal_pong (human play)
  - cargo run --example normal_snake (human play)


-----

Some samples of the examples:



<img width="1511" height="964" alt="Screenshot_just_chatting" src="https://github.com/user-attachments/assets/7b2aff7a-156a-45db-89fe-80f08ea4ec10" />

*Spirion - Just Chatting example. Visual simulation of neuron-graph with neuron as yellow nodes, axons as green/red lines, and blue as the output nodes which correspond to individual keys. Can type user input (top right) and can receive the response from Spirion (top left).*

<img width="797" height="595" alt="Screenshot_pong" src="https://github.com/user-attachments/assets/2386278b-d8d3-45a3-a16a-62b17cefaedc" />

*Spirion - Pong example. Spirion playing pong (really badly). Input corresponds to the entire tile layout (white = +1, black = -1), output corresponds to Up or Down to move the paddle. Score (top right). Losing a point causes 'pain' to alter function, and gaining a point causes 'pleasure' to deepen function.*



-----


Checkpoints Complete:
- Checkpoint 1


Goals before next checkpoint (2):
- Refine current connection alterations, add building connections as well as breaking unwanted connections -> Started progress, some elusive bugs are crawling around now. 
  - Thinking of reworking neurons to include neuropeptide signaling. Would need spacial organization-esk thing to allow action over a local region. Maybe something that functions similar? Traverse signal a couple connections up and down stream? Would be costly.
- Streamline input-data convertions (ie camera vs microphone or swap to a pressure sensor) and data - outputs convertions (ie motor, actuator, button, video game, pnumatic pump)
- Optimize current tick function to run faster and more efficently -> mostly done?
- Current rendering iteration is cool, but super cluncky. Switch it so that it only renders active neurons prob, I have this cool idea about fading the further connections into blackness or smth. -> Done-ish
- New test case: Snake game. Seems like a fun idea, so who knows. -> Human-play done, just need to integrate spirion


 Next Checkpoint: Save options. Spinning up a new brain brings with it complete randomness and is dumber then the dumbest animal. Jellyfish? The ability to save the current state of the neurons and then spin up a brain using saved data (.bin probably). This allows continuity, and so that everytime its shut down, it can be paused and played rather then killed and reborn.


Checkpoint 1: Basic usage of the brain. The logic and stuff should be stable and usable and scalable. Efficency pathways should be semi-clear. I also want some practice options for testing various features
