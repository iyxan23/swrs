# SWRS
[![Hits-of-Code](https://hitsofcode.com/github/iyxan23/swrs?branch=main)](https://hitsofcode.com/github/iyxan23/swrs/view?branch=main)

A rusty library that reads, parses and re-constructs sketchware projects.

This might look dumb to use rust, but I'm currently planning to do something bigger with this.

This might also look dumb to even make this library because sketchware is practically dead by now. But I had this dream of making something _rather interesting_ with sketchware, so I would say this project is for _learning-purposes_. I hope I'll be able to adapt this library to sketchware 2 when/if it got released, and probably create support to sketchware mod versions aswell.

## Development state
This project is **Almost finished**. `api` and `parser` are working as expected (on multiple tests from public project off of Sketchub), I just need to do more and more tests, write a lot of documentation, and implement resource management.

Stuff:
 - [x] Encrypting & Decrypting a sketchware project
 - [x] Parsing of all data files
 - [x] Reconstruction of all data files
 - [ ] APIs
   - [x] Project metadata retrival
   - [x] Screens
     - [x] Screen metadata retrival
     - [x] Layout
     - [x] Blocks / Logic
       - [x] Events
   - [x] Conversion back to parser models
   - [ ] **TESTING**
 - [ ] Resources implementation

## Cool, I want to help!
I'd be very very thankful for those that are interested in contributing to this project. I'm new to rust and my code needs some reviewing from an actual rust user and I would love to hear feedbacks from it!

If you are interested in contributing to the code or documentation, go ahead! :^)
