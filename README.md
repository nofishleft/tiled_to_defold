# tiled_to_defold
Converts a .tmx [tile map](https://mapeditor.org) to a [defold gameobject](https://defold.com/).

Defold's tilemap & tilemap editor do not support isometric tilemaps.
This has been written explicitly for non-staggered isometric tilemaps.
Staggered isometric and hexagonal tilemaps will likely not work, but may be added in the future.

It is unlikely orthogonal will ever be added.
If orthogonal tilemaps are needed, it's recommended to use defolds internal tilemap editor.
This has the benefit of more efficient rendering, as this tool just creates a collection of sprites from a tilemap.

## Pre-requisites
[Installing Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Usage
Takes a .tmx file as an input, and outputs a .tilesource for each tileset and a .go for each layer in the tile map.

```sh
cargo run -- Path/To/Map.tmx Path/To/Output/Dir/
```

## Todo
- [x] Isometric
- [ ] Isometric (Staggered)
- [ ] Hexagonal (Staggered)
- [ ] Copy image files or remap relative to the defold projec
- [ ] ~~Orthogonal~~

## Dependencies
| Dependency | Desc                                        | Links                                                                                                                                        |
|------------|---------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| Rust       | A fast & memory safe language.              | [rust-lang](https://www.rust-lang.org/)                                                                                                      |
| Tiled      | A rust crate for reading TMX and TSX files. | [github](https://github.com/mapeditor/rs-tiled), [docs.rs](https://docs.rs/tiled/latest/tiled/). [crates.io](https://crates.io/crates/tiled) |

## See Also
| Desc                                                    | Links                                                                                       |
|---------------------------------------------------------|---------------------------------------------------------------------------------------------|
| TMX Map Format                                          | [doc.mapeditor.org](https://doc.mapeditor.org/en/stable/reference/tmx-map-format/)          |
| JSON Map Format                                         | [doc.mapeditor.org](https://doc.mapeditor.org/en/stable/reference/json-map-format/)         |
| Python Script to convert .JSON map to defold gameobject | [forum.defold.com](https://forum.defold.com/t/using-tilemap-editor-for-isomertic-map/71877) |

