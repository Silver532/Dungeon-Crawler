# Dungeon Crawler Program

This is the currently active repo for my Dungeon Crawler project.  
I initialy made this project in order to practice work on larger codebases,  
rather than a single file for each project. New iterations of the project  
served to help me learn more, as well as allow for rewrites and code cleanups.



## Iterations

This is my sixth iteration of this project, and the first time I am writing a big  
project in a language other than Python. I decided to move this project into  
Rust to take advantage of static types, improved performance, and as a way  
to learn the language. 

### Where to find Iterations

<ul>
<li>Iteration 1 can be found in my "Personal Projects" repo.</li>
<li>Iteration 2 and 3 are not available, as I no longer have the source code for them.</li>
<li>Iterations 4 and 5 are my "Dungeon Generator" repo, with iteration 4 being commits before March 20, 2026</li>
<li>Iteration 6 is this repo, and will remain until I have another full code rewrite, which will hopefully last longer this time</li>
</ul>

### What each Iteration improved on

<ol>
<li>Started this project. i1 is an unfinished text based dungeon crawler. There were<br> planned systems for save/load, combat, and looting. The game was controlled<br> through a CLI, with commands such as 'go' providing control for the player.</li><br>

<li>Complete rewrite of the code. i2 gave the game graphics for both the original<br> dungeon maps and new room maps. Procedual room generation was added this<br> iteration. Similar to i1, this version was incomplete, with planned systems for player<br> control, combat, and the stitched tilemap system.</li><br>

<li>Partial rewrite of i2, with dungeon generation having a complete rewrite. This<br> iteration was incredibly unoptimized, though I liked the room theme system<br> introduced in this iteration. This was also the first implementation of the stitched<br> tilemap system and full dungeon view. Enemies were added for this iteration, but<br> they couldnt do anything.</li><br>

<li>Complete rewrite of the program after I lost iterations 2 and 3. Dungeon and Room<br> generation both entirely reworked. Generation pipeline stayed the same, but both<br> logic and code were new. There was a major focus on optimization and fixing older<br> issues this iteration.</li><br>

<li>Pipeline rewrite from i4 after I ran into some roadblocks with the implementation.<br> Lots of extra tweaks, and the debugger and visualizer for this iteration is the best<br> one I have built by far. The plan for this iteration was to build the frameworks for<br> as many other systems as I could, before inevitable rewrites for i6. This includes the<br> player, the modular item system, and enemies. This iteration was a major milestone,<br> as it was the first time I was confident enough in my older code to keep nearly all of<br> it through a rewrite.</li><br>

<li>Current Iteration. The goals for this iteration are a full rework of i5 in Rust, with<br> some improvements being made along the way. Once this is done, I intend to lock<br> in the map generator and start iteration on new features such as the item system<br> and the actual gameplay.</li>
</ol>