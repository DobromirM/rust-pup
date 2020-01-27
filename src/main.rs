use clap::{Arg, App};
use std::{fs, process, env, collections};
use maplit::btreemap;
use lazy_static::lazy_static;

mod visualise;
mod download;

//use phf::{Map, phf_map};

fn main() {
    let image_path = match get_path() {
        Some(path) => path,
        None => {
            println!("Cannot find a suitable place for a pup :(");
            process::exit(1);
        }
    };

    let args = App::new("")
        .version("0.1")
        .arg(Arg::with_name("quiet")
            .required(false)
            .value_name("quiet")
            .default_value("true")
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .required(false)
            .value_name("FILE")
            .default_value(&image_path)
            .takes_value(true))
        .arg(Arg::with_name("hd")
            .required(false)
            .value_name("hd")
            .long("hd")
            .short("h")
            .help("Displays the image in HD")
            .takes_value(false))
        .arg(Arg::with_name("breed")
            .required(false)
            .value_name("breed")
            .long("breed")
            .short("b")
            .help("Select the breed of the pups")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .required(false)
            .value_name("list")
            .long("list")
            .short("l")
            .help("List all possible breeds")
            .takes_value(false))
        .get_matches();


    let url_string = match args.value_of("breed") {
        Some(breed) if BREEDS.contains_key(breed) => format!("https://dog.ceo/api/breed/{}/images/random", BREEDS.get(breed).unwrap()),
        _ => String::from("https://dog.ceo/api/breeds/image/random")
    };

    if args.is_present("list") {
        for (breed, _) in BREEDS.iter() {
            println!("{}", breed);
        }
    } else {
        let file_path = match download::get_image(&args, &url_string) {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to retrieve pup :(");
                process::exit(2);
            }
        };

        if args.is_present("hd") {
            if process::Command::new("feh")
                .arg("&")
                .arg("-x")
                .arg("-FZ")
                .arg(&file_path)
                .output().is_err() {
                println!("The pup is hiding :(");
            }
        } else {
            if visualise::draw_image(&file_path).is_err() {
                println!("The pup is hiding :(");
            }
        }

        if fs::remove_file(&file_path).is_err() {
            println!("The pup got stuck :( \nYou can find it in {}", &file_path);
            process::exit(3);
        }
    }
}

fn get_path() -> Option<String>
{
    let image_path = format!("{}/{}", env::temp_dir().to_str()?, "dog.jpg");
    Some(image_path)
}

lazy_static! {
    static ref BREEDS: collections::BTreeMap<&'static str, &'static str> = btreemap! {
        "affenpinscher" => "affenpinscher",
        "afghan_hound" => "hound/afghan",
        "african" => "african",
        "airedale" => "airedale",
        "akita" => "akita",
        "american_terrier" => "terrier/american",
        "appenzeller" => "appenzeller",
        "australian" => "australian",
        "australian_cattledog" => "cattledog/australian",
        "australian_shepherd" => "shepherd/australian",
        "australian_terrier" => "terrier/australian",
        "basenji" => "basenji",
        "basset_hound" => "hound/basset",
        "beagle" => "beagle",
        "bedlington_terrier" => "terrier/bedlington",
        "bernese_mountain" => "mountain/bernese",
        "bichon_frise" => "frise/bichon",
        "blenheim_spaniel" => "spaniel/blenheim",
        "blood_hound" => "hound/blood",
        "bluetick" => "bluetick",
        "border_collie" => "collie/border",
        "border_terrier" => "terrier/border",
        "borzoi" => "borzoi",
        "boston_bulldog" => "bulldog/boston",
        "bouvier" => "bouvier",
        "boxer" => "boxer",
        "brabancon" => "brabancon",
        "briard" => "briard",
        "brittany_spaniel" => "spaniel/brittany",
        "buhund" => "buhund",
        "bull_mastiff" => "mastiff/bull",
        "bulldog" => "bulldog",
        "bullterrier" => "bullterrier",
        "cairn" => "cairn",
        "cardigan_corgi" => "corgi/cardigan",
        "cattledog" => "cattledog",
        "chesapeake_retriever" => "retriever/chesapeake",
        "chihuahua" => "chihuahua",
        "chow" => "chow",
        "clumber" => "clumber",
        "cockapoo" => "cockapoo",
        "cocker_spaniel" => "spaniel/cocker",
        "collie" => "collie",
        "coonhound" => "coonhound",
        "corgi" => "corgi",
        "coton_de_tulear" => "cotondetulear",
        "curly_retriever" => "retriever/curly",
        "dachshund" => "dachshund",
        "dalmatian" => "dalmatian",
        "dandie_terrier" => "terrier/dandie",
        "dane" => "dane",
        "deerhound" => "deerhound",
        "dhole" => "dhole",
        "dingo" => "dingo",
        "doberman" => "doberman",
        "elkhound" => "elkhound",
        "english_bulldog" => "bulldog/english",
        "english_hound" => "hound/english",
        "english_mastiff" => "mastiff/english",
        "english_setter" => "setter/english",
        "english_sheepdog" => "sheepdog/english",
        "english_springer" => "springer/english",
        "entlebucher" => "entlebucher",
        "eskimo" => "eskimo",
        "flat_coated_retriever" => "retriever/flatcoated",
        "fox_terrier" => "terrier/fox",
        "french_bulldog" => "bulldog/french",
        "frise" => "frise",
        "german_pointer" => "pointer/german",
        "german_longhaired_pointer" => "pointer/germanlonghair",
        "german_shepherd" => "germanshepherd",
        "giant_schnauzer" => "schnauzer/giant",
        "golden_retriever" => "retriever/golden",
        "gordon_setter" => "setter/gordon",
        "great_dane" => "dane/great",
        "greyhound" => "greyhound",
        "groenendael" => "groenendael",
        "hound" => "hound",
        "husky" => "husky",
        "ibizan_hound" => "hound/ibizan",
        "irish_setter" => "setter/irish",
        "irish_spaniel" => "spaniel/irish",
        "irish_terrier" => "terrier/irish",
        "irish_wolfhound" => "wolfhound/irish",
        "italian_greyhound" => "greyhound/italian",
        "japanese_spaniel" => "spaniel/japanese",
        "keeshond" => "keeshond",
        "kelpie" => "kelpie",
        "kerryblue_terrier" => "terrier/kerryblue",
        "komondor" => "komondor",
        "kuvasz" => "kuvasz",
        "labrador" => "labrador",
        "lakeland_terrier" => "terrier/lakeland",
        "leonberg" => "leonberg",
        "lhasa" => "lhasa",
        "malamute" => "malamute",
        "malinois" => "malinois",
        "maltese" => "maltese",
        "mastiff" => "mastiff",
        "mexican_hairless" => "mexicanhairless",
        "miniature_pinscher" => "pinscher/miniature",
        "miniature_poodle" => "poodle/miniature",
        "miniature_schnauzer" => "schnauzer/miniature",
        "mix" => "mix",
        "mountain" => "mountain",
        "newfoundland" => "newfoundland",
        "norfolk_terrier" => "terrier/norfolk",
        "norwegian_buhund" => "buhund/norwegian",
        "norwegian_elkhound" => "elkhound/norwegian",
        "norwich_terrier" => "terrier/norwich",
        "otterhound" => "otterhound",
        "papillon" => "papillon",
        "patterdale_terrier" => "terrier/patterdale",
        "pekinese" => "pekinese",
        "pembroke" => "pembroke",
        "pinscher" => "pinscher",
        "pitbull" => "pitbull",
        "pointer" => "pointer",
        "pomeranian" => "pomeranian",
        "poodle" => "poodle",
        "pug" => "pug",
        "puggle" => "puggle",
        "pyrenees" => "pyrenees",
        "redbone" => "redbone",
        "retriever" => "retriever",
        "rhodesian_ridgeback" => "ridgeback/rhodesian",
        "ridgeback" => "ridgeback",
        "rottweiler" => "rottweiler",
        "russell_terrier" => "terrier/russell",
        "saluki" => "saluki",
        "samoyed" => "samoyed",
        "schipperke" => "schipperke",
        "schnauzer" => "schnauzer",
        "scottish_deerhound" => "deerhound/scottish",
        "scottish_terrier" => "terrier/scottish",
        "sealyham_terrier" => "terrier/sealyham",
        "setter" => "setter",
        "sheepdog" => "sheepdog",
        "shetland_sheepdog" => "sheepdog/shetland",
        "shiba" => "shiba",
        "shihtzu" => "shihtzu",
        "silky_terrier" => "terrier/silky",
        "spaniel" => "spaniel",
        "spanish_waterdog" => "waterdog/spanish",
        "springer" => "springer",
        "staffordshire_bullterrier" => "bullterrier/staffordshire",
        "standard_poodle" => "poodle/standard",
        "st_bernard" => "stbernard",
        "sussex_spaniel" => "spaniel/sussex",
        "swiss_mountain" => "mountain/swiss",
        "terrier" => "terrier",
        "tibetan_mastiff" => "mastiff/tibetan",
        "tibetan_terrier" => "terrier/tibetan",
        "toy_poodle" => "poodle/toy",
        "toy_terrier" => "terrier/toy",
        "vizsla" => "vizsla",
        "walker_hound" => "hound/walker",
        "waterdog" => "waterdog",
        "weimaraner" => "weimaraner",
        "welsh_spaniel" => "spaniel/welsh",
        "westhighland_terrier" => "terrier/westhighland",
        "wheaten_terrier" => "terrier/wheaten",
        "whippet" => "whippet",
        "wolfhound" => "wolfhound",
        "yorkshire_terrier" => "terrier/yorkshire",
    };
}