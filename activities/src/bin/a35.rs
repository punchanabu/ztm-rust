// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:

//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}
fn print_tile(tile: Tile) {
    use Tile::*;

    match tile {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {brick:?}")
        },
        Brick(other) => println!("{other:?} brick"),
        Dirt | Grass | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount,
        }) if amount >= 100 => println!("Lots of gold!"),
        Water(Pressure(pressure)) if pressure >= 10 => println!("High water pressure!"),
        Water(Pressure(pressure)) => println!("Water pressure level: {pressure}"),
        _ => (),
    }
}
fn main() {
    let tile = Tile::Brick(BrickStyle::Gray);
    print_tile(tile);


    let tile = Tile::Sand;
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 200,
    });
    print_tile(tile);

    let tile = Tile::Water(Pressure(0));
    print_tile(tile);
}
