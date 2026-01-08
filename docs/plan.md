Next steps/What do I want to see:
- I feel like the majority of the generated terrain is not liveable. Consistently getting large continents toward the poles in Ice/Snow biomes, but it feels like the center of the map doesn't have enough/as much temperate land.
- Deal with inland seas. 
    - Do I generate the ocean during the first pass, modifying the logic so it only assigns connected Ocean tiles as Ocean, and then handle inland seas assignment separately? (if not ocean, but below sea level, assign as lake?)
    - Or do I keep it as is and just continue with adding river generation.
- Once I'm happy with the continents/sizing, I  want to add rivers. Separate biome for river tiles, flow from high to low
- I feel like maybe oceans should be present on map edges more often. This would feel more natural
- Work on chunk loading/unloading so I can have a larger world size