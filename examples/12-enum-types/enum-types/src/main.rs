#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Aguascalientes,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        WineRegions::Aguascalientes => println!("Aguascalientes is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn get_wine_popularity (wine: Wine) -> String {
    let mut popularity = String::new();
    let region = wine.region;
    match region {
        WineRegions::Aguascalientes => popularity = "Highly Popular!".to_string(),
        WineRegions::Rioja => popularity = "Well known among the experts.".to_string(),
        WineRegions::NapaValley => popularity = "Popular on certain regions.".to_string(),
        _ => popularity = String::from("Unkonwn wine!"),
    }
    popularity
}
fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Cuatro Soles"),
        region: WineRegions::Aguascalientes,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
    let str = get_wine_popularity(wine3);
    println!("{}",str);
}
