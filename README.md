# Brain
A fun little project of making a digital brain (Named Spirion)

Originally was just gonna be a small little project, an idea I had while working on my Ants project and a nice distraction.

Ended up spending an obsene amount of time on it testing and iterating. Some tears were shed, but can confidently say that my work on the ant project vastly prepared me for this one, which was good because another project spawned off this one as a result anyways. Wooo.

Only now got around to actually making it a repository, because it was feeling more and more like an actual project rather then a side quest. But yeah, we are here now, so progress can be adequetly tracked.

And now, much later, can confidently say that work here has vastly prepared me for my Ants project as well, so thats wonderful!

How to use:
(Techinically its a library, so eventually I'll get this into crates.io when its not abhorent.)

Download the nessisary files, import {use digital_brain::Spirion;}
Make use of the various tools avaible. Right now there are not a lot.

Can run example files as well, currently there are 2 (Technically)
Run in terminal:
  - cargo run --example just_chatting
  - cargo run --example pong
  - cargo run --example normal_pong


Checkpoints Complete:
- Checkpoint 1


Goals before next checkpoint (2):
- Refine current connection alterations, add building connections as well as breaking unwanted connections
- Streamline input-data convertions (ie camera vs microphone or swap to a pressure sensor) and data - outputs convertions (ie motor, actuator, button, video game, pnumatic pump)
- Optimize current tick function to run faster and more efficently
- Current rendering iteration is cool, but super cluncky. Switch it so that it only renders active neurons prob, I have this cool idea about fading the further connections into blackness or smth.
- New test case: Snake game. Seems like a fun idea, so who knows.


 Next Checkpoint: Save options. Spinning up a new brain brings with it complete randomness and is dumber then the dumbest animal. Jellyfish? The ability to save the current state of the neurons and then spin up a brain using saved data (.bin probably). This allows continuity, and so that everytime its shut down, it can be paused and played rather then killed and reborn.


Checkpoint 1: Basic usage of the brain. The logic and stuff should be stable and usable and scalable. Efficency pathways should be semi-clear. I also want some practice options for testing various features
