# SWRS
[![Hits-of-Code](https://hitsofcode.com/github/iyxan23/swrs?branch=main)](https://hitsofcode.com/github/iyxan23/swrs/view?branch=main)

A rusty library that reads, parses and re-constructs sketchware projects.

This might look dumb to use rust, but I'm currently planning to do something bigger with this.

This might also look dumb to even make this library because sketchware is practically dead by now. But I had this dream of making something _rather interesting_ with sketchware, so I would say this project is for _learning-purposes_. I hope I'll be able to adapt this library to sketchware 2 when/if it got released, and probably create support to sketchware mod versions aswell.

## Development state
This project is **Partly finished** (and its not stabilised yet). The part that's finished is the parser (the easy part), if you just wanted to tweak around some low-level stuff, like listing activities and customviews, then the parser should be enough. If you wanted to do something high-level, like iterating through blocks, it unfortunately is WIP, you can check its progress on the `api` folder.

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
 - [ ] APIs
   - [x] Project metadata retrival
   - [ ] Screens
     - [x] Screen metadata retrival
     - [ ] Layout
     - [ ] Blocks / Logic
       - [ ] Events
   - [ ] Resources
 - [ ] Resources implementation

## Cool, I want to help!
I'd be very very thankful for those that are interested in contributing to this project. I'm new to rust and my code needs some reviewing from an actual rust user and I would love to hear feedbacks from it!

If you are interested in contributing to the code or documentation, go ahead! :^)
