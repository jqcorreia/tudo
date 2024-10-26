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
- Minimal list of dependencies, i.e no UI toolkit

## Thing I will **not** implement
- TTF parser
- HTTP client
## Dependencies
- SDL2 with `ttf` and `image`

## Features
- Application launcher (XDG only)
- XCB and EWMH based window switching (supports: i2, awesome, xfce, etc.)
- `pass` secrets integration
- (Really) Minimal UI lib with a couple of components and a layout manager
- Primitive Lua sources support (no function exporting yet)
- Texture cache (fonts, icons and generic image files)
- Asynchronous load of item sources in order to reduce startup time (no async/await, simple thread spawn)

## Sources
- XDG Applications
- EWMH based window switching (supports: i3, awesome, xfce, etc.)
- `tmux` sessions
- `pass` secrets
- TODO: Notion Notes
- TODO: Browser tabs (this one is a challenge)

## Rolling Dev Notes
- [x] Solve the mistery of proper font atlas, right now using direct render from SDL2 ttf. It's kerning...
- [x] action tags and search for action (:run, :window, :secret, etc)
- [x] Icon files may not exist even if referenced by XDG icon files format
- [x] Various font sizes and faces. This was hard....
- [x] Primitive animations
- [x] tmux list source items, open default terminal in a given session
- [x] Async load sources
- [x] Implement drawing 'toolkit' context containing Sdl, Canvas, TextureCreator, etc
- [x] Show that the sources are still loading (really simple, but working)
- [x] Prevent from starting multiple instances
- [x] Create .config folder, need to check if this is cross platform
- [x] Lua based configuration (very early but promising)
- [x] Migrate to mlua, due to serde support
- [x] Support for multiple screens
- [x] ! Layout builder and simplified component access
- [x] Component get_state and set_state
- [x] Use a new component spinner to sinalize sources loading
- [x] Extend canvas drawing functions implementing more complex shapes (rounded rect, circle, quadrants, ...) 
- [ ] ! Implement component focus (which will receive events)
- [ ] Mouse coords translation to local component coords and list item click
- [ ] Get system default font, maybe
- [ ] Prompt window available to actions
- [ ] More prettier  
- [ ] Better search (fuzzy)
- [ ] Notion integration for notes. Depends on a prompt action.
- [ ] Investigate async/await for async source loading
- [ ] Dashboard-like widgets for things like metrics
- [ ] Submenus for some actions

! - means ongoing

## Wayland support

Just add `SDL_VIDEODRIVER=wayland` to your environment variables

