use std::io::{BufWriter, Write};
use std::sync::Arc;
use tiled;
use tiled::{ChunkData, Layer, LayerType, TileLayer};

fn write_sprite<T: Write>(writer: &mut BufWriter<T>, tx: i32, ty: i32, tileset: &str, tileset_index: usize, px: f64, py: f64, pz: f64) -> std::io::Result<()> {
    write!(writer,
"\
embedded_components {{
    id: \"tile_{tx}_{ty}\"
    type: \"sprite\"
    data: \"tile_set: \\\"{tileset}\\\"\\n\"
    \"default_animation: \\\"{tileset_index}\\\"\\n\"
    \"material: \\\"/builtins/materials/sprite.material\\\"\\n\"
    \"blend_mode: BLEND_MODE_ALPHA\\n\"
    \"\"
    position {{
        x: {px}
        y: {py}
        z: {pz}
    }}
    rotation {{
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0
    }}
}}
")
}

fn calculate_tile_coords(tx: i32, ty: i32, tile_width: i32, tile_height: i32, layer: &Layer) -> (f64, f64, f64) {
    let rx = ((tx * tile_width) as f64 * 0.5f64) - ((ty * tile_width) as f64 * 0.5f64) + layer.offset_x as f64;
    let ry = ((tx * tile_height) as f64 * 0.5f64) + ((ty * tile_height) as f64 * 0.5f64) + layer.offset_y as f64;
    let rz = (tx as f64 * 0.0001f64 + ty as f64 * 0.01f64) * -0.1f64;
    return (rx, ry, rz);
}

fn write_layers<'map>(dir: &str, layers: impl ExactSizeIterator<Item = Layer<'map>>, tile_width: u32, tile_height: u32) -> std::io::Result<()> {
    for layer in layers {
        if let LayerType::Tiles(tlayer) = layer.layer_type() {
            let mut f = BufWriter::new(std::fs::File::create([dir, "/", &layer.name, ".go"].join(""))?);
            match tlayer {
                TileLayer::Finite(finite) => {
                    for x in 0..finite.width() {
                        for y in 0..finite.width() {
                            if let Some(tile) = finite.get_tile(x as i32, y as i32) {
                                let (rx, ry, rz) = calculate_tile_coords(x as i32, y as i32, tile_width as i32, tile_height as i32, &layer);
                                write_sprite(&mut f, x as i32, y as i32, &tile.get_tileset().name, tile.tileset_index(), rx, ry, rz)?;
                            }
                        }
                    }
                }
                TileLayer::Infinite(infinite) => {
                    for ((base_x, base_y), chunk) in infinite.chunks() {
                        for x in 0..ChunkData::WIDTH {
                            for y in 0..ChunkData::HEIGHT {
                                if let Some(tile) = chunk.get_tile(x as i32, y as i32) {
                                    let tx = x as i32 + base_x;
                                    let ty = y as i32 + base_y;
                                    let (rx, ry, rz) = calculate_tile_coords(tx, ty, tile_width as i32, tile_height as i32, &layer);
                                    write_sprite(&mut f, tx, ty, &tile.get_tileset().name, tile.tileset_index(), rx, ry, rz)?;
                                }
                            }
                        }
                    }
                }
            };
        }
    }
    return Ok(());
}

fn write_tilesource_header<T: Write>(writer: &mut BufWriter<T>, img: &str, width: u32, height: u32) -> std::io::Result<()> {
    write!(writer,
"\
image: {img}
tile_width: {width}
tile_height: {height}
tile_margin: 0
tile_spacing: 0
collision: \"\"
material_tag: \"tile\"
collision_groups: \"default\"
"
    )
}

fn write_tilesource_footer<T: Write>(writer: &mut BufWriter<T>) -> std::io::Result<()> {
    write!(writer,
"\
extrude_borders: 2
inner_padding: 0
sprite_trim_mode: SPRITE_TRIM_MODE_OFF
")
}

fn write_tilesource_animation<T: Write>(writer: &mut BufWriter<T>, id: u32) -> std::io::Result<()> {
    write!(writer,
"\
animations {{
    id: \"{id}\"
    start_tile: {id}
    end_tile: {id}
    playback: PLAYBACK_NONE
    fps: 0
    flip_horizontal: 0
    flip_vertical: 0
}}
"
    )
}

fn write_tilesource(dir: &str, tilesets: &[Arc<tiled::Tileset>]) -> std::io::Result<()> {
    for tileset in tilesets {
        let mut f = BufWriter::new(std::fs::File::create([dir, "/", &tileset.name, ".tilesource"].join(""))?);
        write_tilesource_header(&mut f, &tileset.name, tileset.tile_width, tileset.tile_height)?;

        for (id, _) in tileset.tiles() {
            write_tilesource_animation(&mut f, id)?;
        }

        write_tilesource_footer(&mut f)?;
    }
    return Ok(());
}

fn convert(input_file: &str, output_dir: &str) {
    let mut loader = tiled::Loader::new();
    let map = loader.load_tmx_map(input_file).expect("Couldn't load map!");
    write_tilesource(output_dir, map.tilesets()).expect("Couldn't write tilesource!");
    write_layers(output_dir, map.layers(), map.tile_width, map.tile_height).expect("Couldn't write layers!");
}

fn help() {
    println!("Usage:\ntiled_to_defold <input_map> <output_directory>");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => {
            convert(&args[1], &args[2]);
        },
        _ => {
            help();
        }
    }
}
