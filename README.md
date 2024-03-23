# Rustys demo application
A demo application for the 2D physics engine [Rustycs](https://github.com/divtor/rustycs).

## Choosing a scene
The current implementation of `rustycs_demo` uses `WorldFactory`s to load predetermined `WorldScene`s.
* The available scenes can be viewed in the file `rustycs_demo/src/demo_scenes.rs` in the implementation block annotated with `// demo`.
* All other scenes were used for testing and can be found in the implementation block annotated with `// testing`.

In the file `main.rs` load the desired scene via the factory methods. 

Once the correct scene has been chosen with its desired parameters (such as tickrate of the physics simulation), the simulation can be started by executing the cli `cargo run --release` from the root directory of the cloned repository.

## Debug functionality
Pressing the `ESC` key toggles the simulation from `Running` to `Paused`, and vice versa. 
Once paused, the program presents you with a list of possible actions and how to use them, specified via the associated keys in their `[Key]` prefix.
Iff the simulation is paused the user can hover over a dynamic body in the simulation and gets useful debugging information:
* The body location.
* The body velocity.
* The body angular velocity (rotation).
* A visual indicator starting from the body origin going into the velocity direction.

## Interactive functionality
Users can interact with the simulation by spawning new physics bodies or attractors ontop of their current mouse position. 
The relevant keymappings are specified in the pause menu.

* Bodies are physical objects within the simulation, that get affected by forces of the world and can collide with other bodies. There are dynamic and static bodies and their possible shapes etc. are listed in the following section.

* Attractors are entities within the simulation that emit a gravitational pull towards bodies. The strength relies on their own mass and the mass of the body that gets pulled.
