# For conways:
* "Infinite/ Wallpaper" Mode, basically, if population gets less than some amount, spawn in a couple more randomly, just to keep things constantly moving
* Toggle for tiling walls (basically overlapping as if the walls aren't there), and "hard" walls
* Toggle to infinitely draw the map to fill the viewscreen and give the impression that the simulation is infinite





# Rewrite Plan of action:


1. "Strip" out all code pertaining to conways game of life, store that separately for now
2. Keep all basic UI Elements, then, re-implement the "Map" struct,
   and write generalized code that will draw a grid of cells to the screen.
3. The grid of cells must have the following functionality
    a) Most importantly, be able to actually click on individual cells in the map and toggle their state!!!

    b) The view is based on a viewport, that, if your viewport is smaller than that of the actual total size of the array, (Say the map is 100x100 cells, but my viewport is 50x50) then, you can pan around the viewport by dragging, also be able to zoom with scroll wheel
    c) If you are zoomed in with the viewport, have a minimap that pops up somewhere, that just draws a smaller square/rectangle inside of a larger one representing where your viewport is in relation to the larger screen

    d) implement lazy loading/rendering for the viewport. running with the previous example above, on every draw call we can make a quick check to see if the index of the cell we are looking at is inside of the viewport, if it is? we make our draw call, if it isnt? we still do our logic checks, but dont make the draw call. That way we might be able to eek some performance improvements out on more intensive simulations

4. Only after you get all of the above functionality implemented and working, can you go back to rewriting conways, that way you'll have a good generalized base for how you want things to go, and you can build your app from there

