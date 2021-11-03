# SWRS
A rusty library that reads, parses and re-constructs sketchware projects.

This might look dumb to use rust, but I'm currently planning to do something bigger with this.

This might also look dumb to even make this library because sketchware is practically dead by now. But I had this dream of making something _rather interesting_ with sketchware, so I would say this project is for _learning-purposes_. I hope I'll be able to adapt this library to sketchware 2 when/if it got released, and probably create support to sketchware mod versions aswell.

## Development state
This project is **In-progress (unfinished; partly finished)**. I really do hope in getting this project finished lmao

Stuff:
 - [x] Encrypting & Decrypting a sketchware project
 - [x] Parsing `project`
 - [x] Parsing `file`
 - [x] Parsing `library`
   > Note: I'm planning to do something better, the current implementation only works one-way (only deserializing)
 - [x] Parsing `resource`
 - [x] Parsing `view`
   > Note: This is a simple implementation, I'm planning to create a way to neglect unnecessary fields depending on its view type (something like TextView, it only cares about `text` and other stuff, fields like `spinner_mode` will be erased) and convert them back into its root form to serialize/reconstruct it.
 - [x] Parsing `logic`
 - [x] Reconstructing `project`
 - [x] Reconstructing `file`
 - [x] Reconstructing `library`
 - [x] Reconstructing `resource`
 - [ ] Reconstructing `view`
 - [ ] Reconstructing `logic`

## Curious?
If you're wondering on how to read a sketchware project's data yourself, I have been writing details about the sketchware project structure and how to read & parse them on [`docs/`](docs/reading-a-sketchware-project.md), go ahead and give it a little bit of a read

## Cool, I want to help!
I'd be very very thankful for those that are interested in contributing to this project. I'm new to rust and my code needs some reviewing from an actual rust user and I would love to hear feedbacks from it!

If you are interested in contributing to the code or documentation, go ahead! :^)
