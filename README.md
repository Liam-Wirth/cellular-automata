# Cellular Automata Engine of sorts

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

Built using Emilk's [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), template repo. Eframe is a framework for writing apps using [egui](https://github.com/emilk/egui/).




# Check it out Here!
https://cellular-automata.wirthless.dev



## Todo:
Make sure to update the build actions eventually to re-include -D warnings

possibly cut back on CI/CD by trimming out alot of the excess deployment stuff?
I dunno
hopefully works?


## Credits/Thanks:
Huge shoutout and thanks to [Todd Smith](https://github.com/toddATavail) In my research on trying to learn how to do all this, I stumbled across articles that he was also writing articles on simulating Elementary CAS, [First one Here](https://xebia.com/blog/cellular-automata-using-rust-part-i/) what was so interesting to me was the fact that he was also using Rust, and had yet to write the Third article in the mini-series, meaning he was actively writing them as I was working on the project. On a whim I reached out to him on linkedin, and he was kind enough to take time out of his day to look at my crappy rust code, and provide advice! I couldn't thank the man enough for doing that, as it meant alot to me to be able to have someone look at my code. If you have the time please make sure to check out his articles that I linked above and give em a read! 

### References and Sources Used:
Rest in peace [John Walker](https://en.wikipedia.org/wiki/John_Walker_(programmer)) his project "CellLab" has proved immensely helpful in my research, 
He passed on February 2, 2024. Less than a week before I started working on this project
#### Wikipedia:
* [Cellular Automata](https://en.wikipedia.org/wiki/Cellular_automata)
* [Von Neumann Neighborhood](https://en.wikipedia.org/wiki/Von_Neumann_neighborhood)
* [Moore Neighborhood](https://en.wikipedia.org/wiki/Moore_neighborhood)
* [Elementary Cellular Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
* [Lattice Gas Automaton](https://en.wikipedia.org/wiki/Lattice_gas_automaton)
* [Multi agent systems](https://en.wikipedia.org/wiki/Multi-agent_system)


#### Research Papers and Books
(note, I may or may not have read these papers, but am including all of the ones I found interesting, or ones that seemed like they could be helpful to look at later here)

* [Two Dimensional Cellular Automata-Norman Packard and Stephen Wolfram](https://content.wolfram.com/sw-publications/2020/07/two-dimensional-cellular-automata.pdf)
* [Stephen Wolframs 'A new Kind of Science'](https://www.wolframscience.com/nks/p170--cellular-automata/)
* [Preston and Duff's "Modern Cellular Automata"](https://link.springer.com/book/10.1007/978-1-4899-0393-8)
* [Particle-Based Fluid Simulation for Interactive Applications - Matthias Müller, David Charypar and Markus Gross](https://matthias-research.github.io/pages/publications/sca03.pdf)
* [Particle-based Viscoelastic Fluid Simulation](http://www.ligum.umontreal.ca/Clavet-2005-PVFS/pvfs.pdf)



#### Other Open source projects
* Riley Shaw's [Terra.js](https://rileyjshaw.com/terra/) ([link to repo](https://github.com/rileyjshaw/terra))
* Fourmilab's [Cellular Automata Laboratory](https://www.fourmilab.ch/cellab/webca/)
* George Mason University's [MASON](https://github.com/eclab/mason/)

#### Rules to look into (for me)

* [Brian's Brain](https://en.wikipedia.org/wiki/Brian's_Brain)
* [Seeds](https://en.wikipedia.org/wiki/Seeds_(cellular_automaton))
* [List of Rules](https://en.wikipedia.org/wiki/Category:Cellular_automaton_rules)
* [Lenia](https://en.wikipedia.org/wiki/Lenia) [repo](https://github.com/Chakazul/Lenia)


#### Misc stuff to check out
* [SandBox](https://github.com/hakolao/sandbox)
* [PowderToy](https://github.com/The-Powder-Toy/The-Powder-Toy)
* [Particle Life](https://github.com/hunar4321/particle-life)
* [FluidSim](https://github.com/SebLague/Fluid-Sim)
* [Automato](https://github.com/tsoding/atomato)

#### Related Videos I Watched
* [How to code a falling sand simulation (like noita) with cellular automata - MARF](https://youtu.be/5Ka3tbbT-9E?si=Ay0CW-jYHkft_iae) - Happened to be in my Recommended, and was an interesting watch
* [Cellular Automata: Multi-State world (rock, paper, scissor, lizard, spock)-Efrans](https://www.youtube.com/watch?v=TvZI6Xc0J1Y) 
* [Complex Behaviour from Simple Rules: 3 Simulations - Sebastian Lague](https://www.youtube.com/watch?v=kzwT3wQWAHE)
* [Coding Adventure: Simulating Fluids-Sebastian Lague](https://www.youtube.com/watch?v=rSKMYc1CQHE)
* [Living Machines in TypeScript (Autocell Ep.02)- TsodingDaily](https://www.youtube.com/watch?v=Hz_13P7lRoA)
* [Cellular Automata: Complexity From Simplicity- AceRolla](https://www.youtube.com/watch?v=t_HcBAO_Yas)
## Instructions from the original E-Frame Template
### Running localy

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`
### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website



