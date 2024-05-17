A example of modular characters in response to [SnowdenWintermute](https://github.com/SnowdenWintermute/bevy-modular-characters-proof-of-concept)'s [video](https://www.youtube.com/watch?v=jbYDljqf4kg).

# Explanation

A `Component` is created for each module that the character can have.  
https://github.com/hukasu/bevy-modular-characters/blob/6006e8d8b88409b55e9a93934caf7f9aa22efda8/src/modular/components.rs#L6
In this example there are 4 modules, `Head`, `Body`, `Legs`, and `Feet`.

An `Entity` is created with all 4 `Componentes` and the skeleton.
https://github.com/hukasu/bevy-modular-characters/blob/6006e8d8b88409b55e9a93934caf7f9aa22efda8/src/main.rs#L76

Cycle through the modules in response to keyboard inputs.  
https://github.com/hukasu/bevy-modular-characters/blob/6006e8d8b88409b55e9a93934caf7f9aa22efda8/src/modular/mod.rs#L246

When `Scene` finishes spawning, transfer data from it to the modular character. The critical part is the creation of
the `SkinnedMesh` component. It's necessary to collect the names of the joints and search their counterpart on the skeleton.
Preserve the order of the joints is mandatory.  
https://github.com/hukasu/bevy-modular-characters/blob/6006e8d8b88409b55e9a93934caf7f9aa22efda8/src/modular/mod.rs#L126

If on update the `Scene` has yet not finished loading, send an event to the `reset_changed` system for a retry next frame.  
https://github.com/hukasu/bevy-modular-characters/blob/6006e8d8b88409b55e9a93934caf7f9aa22efda8/src/modular/mod.rs#L277

# Models
The models were taken from the Quaternius [Ultimate Modular Women](https://quaternius.com/packs/ultimatemodularwomen.html).  

This example uses the Adventurer, SciFi, Soldier and Witch models with minor adjustments. The original
models contain one (1) scene that loads the Armature and the meshes (head, torso, legs, and feet). Also included
is 2 models that were used by Snowden on his video, that have most of it's content deleted.