# SWRS
A rusty library that reads, parses and re-constructs sketchware projects.

This might look dumb to use rust, but I'm currently planning to do something bigger with this.

This might also look dumb to even make this library because sketchware is practically dead by now. But I had this dream of making something _rather interesting_ with sketchware, so I would say this project is for _learning-purposes_. I hope I'll be able to adapt this library to sketchware 2 when/if it got released, and probably create support to sketchware mod versions aswell.

## Development state
This project is **Finished-ish**. You can use it, but its not going to be easy-to-use. The current plan is to create a layer of APIs ontop of the parser, so it would make modifying sketchware projects much easier.

Stuff:
 - [x] Encrypting & Decrypting a sketchware project
 - [x] Parsing `project`
 - [x] Parsing `file`
 - [x] Parsing `library`
 - [x] Parsing `resource`
 - [x] Parsing `view`
 - [x] Parsing `logic`
 - [x] Reconstructing `project`
 - [x] Reconstructing `file`
 - [x] Reconstructing `library`
 - [x] Reconstructing `resource`
 - [x] Reconstructing `view`
 - [x] Reconstructing `logic`
 - [ ] Intuitive APIs over the parser to abstract everything out and make everything easier, might be separated as a separate cargo feature

## Curious?
If you're wondering on how to read a sketchware project's data yourself, I have been writing details about the sketchware project structure and how to read & parse them on [`docs/`](docs/reading-a-sketchware-project.md), go ahead and give it a little bit of a read

## Cool, I want to help!
I'd be very very thankful for those that are interested in contributing to this project. I'm new to rust and my code needs some reviewing from an actual rust user and I would love to hear feedbacks from it!

If you are interested in contributing to the code or documentation, go ahead! :^)
