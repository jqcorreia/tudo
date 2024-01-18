# TUDO
**T**UDO **U**niveral **D**esktop **O**perator  
**T**ransforming **U**ser-**D**evice **O**peration  
Everything (in Portuguese)  
Take a pick at the meaning.  

Desktop Omnibar. Type and launch stuff. Pretty simple.  
Semi-serious learning project.  
Meant to be used as a wristwatch. You want it there but you don't need to look at it all the time.  
Maybe just replace the concept of a top/bottom bar with the clock and stuff  

## Nice-to-haves and objectives
- As low latency as possible both in launching and in using
- Fast indexing
- Tray icons, interactable
- TOML configuration (with sane defaults)
- Extendable in Lua
- Minimal list of dependencies

## Dependencies
- SDL2 with `ttf` and `image`

## Features
- Application launcher (XDG only)
- XCB and EWMH based window switching (supports: i3, awesome, xfce, etc.)
- `pass` secrets integration
- (Really) Minimal UI lib with a couple of components and a layout manager
- Primitive Lua sources support (no function exporting yet)
- Texture cache (fonts, icons and generic image files)

## TODO
- [x] action tags and search for action (:run, :window, :secret, etc)
- [ ] Get system default font, maybe
- [ ] Various font sizes and faces.
- [ ] Icon files may not exist even if referenced by XDG icon files format
- [ ] Mouse coords translation to local component coords and list item click
- [ ] Prompt window available to actions
- [ ] More prettier  
- [ ] UI Component state
- [ ] Better search (fuzzy)
- [ ] Solve the mistery of proper font atlas, right now using direct render from SDL2 ttf
- [ ] Implement drawing 'toolkit' context containing Sdl, Canvas, TextureCreator, etc

