# Environment Asset Setup

This project now auto-loads optional environment scenes from:

- `assets/environment/house_shell_two_room.glb#Scene0`
- `assets/environment/house_shell_three_room.glb#Scene0`
- `assets/environment/house_shell.glb#Scene0` (fallback for both layouts)
- `assets/environment/house_decor.glb#Scene0`

If a shell file exists, primitive wall/roof blocks are skipped and the shell scene is used.
If a file is missing, the game falls back to current procedural geometry.

## No Blender Quick Path

If you do not have Blender, use any prebuilt interior `.glb` and drop it in:

1. `assets/environment/house_shell.glb`
2. Optional: `assets/environment/house_decor.glb`

That single `house_shell.glb` is now used for both 2-room and 3-room layout visual shells.
No conversion or scene editing required.

## Recommended Free Pack Combo (CC0 / free)

1. Quaternius Ultimate Home Interior
   - https://quaternius.com/packs/ultimatehomeinterior.html
2. Kenney Furniture Kit
   - https://kenney.nl/assets/furniture-kit
3. Kenney Building Kit
   - https://kenney.nl/assets/building-kit
4. Poly Haven PBR textures (for quick material upgrades)
   - https://polyhaven.com/textures

## Optional Advanced Pipeline (Blender)

Use this only if you want custom-built shells/decor:

1. Import selected kit meshes into Blender.
2. Assemble two shell scenes:
   - Scene A: two-room house shell, aligned around world origin.
   - Scene B: three-room house shell, aligned around world origin.
3. Assemble decor scene (furniture/trim) aligned to same origin.
4. Export each as `.glb` (include materials/textures).
5. Save with exact names:
   - `house_shell_two_room.glb`
   - `house_shell_three_room.glb`
   - `house_decor.glb`
6. Place files in `assets/environment/`.

## Notes

- Keep collisions gameplay-driven: this game still uses `Obstacle` bounds for movement/camera.
- Keep scale in meters and up-axis `+Y`.
- Keep shell wall thickness consistent with gameplay bounds so visuals and collision feel aligned.
