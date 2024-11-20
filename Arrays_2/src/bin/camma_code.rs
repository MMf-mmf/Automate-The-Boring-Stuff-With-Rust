use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let spam = vec![
        "apples",
        "bananas",
        "tofu",
        "cats",
        "dogs",
        "fish",
        "mice",
        "birds",
        "elephants",
        "tigers",
        "lions",
        "bears",
        "wolves",
        "foxes",
        "pandas",
        "koalas",
        "kangaroos",
        "giraffes",
        "zebras",
        "rhinos",
        "hippos",
        "crocodiles",
        "alligators",
        "snakes",
        "spiders",
        "scorpions",
        "wasps",
        "bees",
        "hornets",
        "mosquitos",
        "flies",
        "ants",
        "cockroaches",
        "beetles",
        "ladybugs",
        "butterflies",
        "moths",
        "caterpillars",
        "centipedes",
        "millipedes",
        "earthworms",
        "slugs",
        "snails",
        "octopuses",
        "squids",
        "cuttlefish",
        "nautiluses",
        "starfish",
        "sea urchins",
        "sea cucumbers",
        "jellyfish",
        "coral",
        "anemones",
        "crabs",
        "lobsters",
        "shrimps",
        "prawns",
        "barnacles",
        "horseshoe crabs",
        "ticks",
        "mites",
        "fleas",
        "lice",
        "bedbugs",
        "hawks",
        "eagles",
        "sparrows",
        "robins",
        "peacocks",
        "flamingos",
        "swans",
        "ducks",
        "geese",
        "penguins",
        "albatrosses",
        "parrots",
        "canaries",
        "macaws",
        "cockatoos",
        "owls",
        "ravens",
        "crows",
        "magpies",
        "bluebirds",
        "cardinals",
        "woodpeckers",
        "doves",
        "pigeons",
        "herons",
        "storks",
        "cranes",
        "ospreys",
        "kingfishers",
        "hummingbirds",
        "gulls",
        "terns",
        "pelicans",
        "boobies",
        "frigatebirds",
        "kites",
        "buzzards",
        "vultures",
        "condors",
        "chickens",
        "roosters",
        "turkeys",
        "quails",
        "pheasants",
        "partridges",
        "emus",
        "ostriches",
        "cassowaries",
        "kiwis",
        "platypuses",
        "echidnas",
        "wallabies",
        "wombats",
        "tasmanian devils",
        "dingoes",
        "goats",
        "sheep",
        "cows",
        "bulls",
        "buffalo",
        "yaks",
        "donkeys",
        "horses",
        "zebus",
        "alpacas",
        "llamas",
        "camels",
        "dromedaries",
        "boars",
        "pigs",
        "rabbits",
        "hares",
        "deer",
        "moose",
        "elk",
        "caribou",
        "reindeer",
        "antelopes",
        "gazelles",
        "bison",
        "oxen",
        "chimpanzees",
        "gorillas",
        "orangutans",
        "monkeys",
        "lemurs",
        "tarsiers",
        "gibbons",
        "macaques",
        "baboons",
        "mandrills",
        "armadillos",
        "sloths",
        "anteaters",
        "pangolins",
        "tapirs",
        "raccoons",
        "coatis",
        "badgers",
        "weasels",
        "otters",
        "ferrets",
        "martens",
        "wolverines",
        "skunks",
        "kinkajous",
        "squirrels",
        "chipmunks",
        "prairie dogs",
        "groundhogs",
        "beavers",
        "porcupines",
        "hedgehogs",
        "moles",
        "shrews",
        "voles",
        "hamsters",
        "gerbils",
        "guinea pigs",
        "chinchillas",
        "capybaras",
        "agoutis",
        "coypu",
        "muskrats",
        "manatees",
        "dugongs",
        "narwhals",
        "belugas",
        "orcas",
        "dolphins",
        "porpoises",
        "whales",
        "seals",
        "sea lions",
        "walruses",
        "polar bears",
        "arctic foxes",
        "snow leopards",
        "lynxes",
        "bobcats",
        "jaguars",
        "leopards",
        "cheetahs",
        "hyenas",
        "coyotes",
        "jackals",
        "wolves",
        "domestic cats",
        "house mice",
        "rats",
        "marmots",
        "pikas",
        "platypuses",
        "hedgehogs",
        "kangaroo rats",
        "quokkas",
        "dormice",
        "tree frogs",
        "toads",
        "salamanders",
        "newts",
        "axolotls",
        "caecilians",
        "turtles",
        "tortoises",
        "terrapins",
        "geckos",
        "iguanas",
        "chameleons",
        "komodo dragons",
        "monitor lizards",
        "skinks",
        "tuataras",
        "alligators",
        "crocodiles",
        "caimans",
        "gharials",
        "frogs",
        "eels",
        "sharks",
        "rays",
        "skates",
        "hagfish",
        "lampreys",
        "trout",
        "salmon",
        "pikes",
        "bass",
        "carp",
        "catfish",
        "eels",
        "tuna",
        "swordfish",
        "marlin",
        "seahorses",
        "pipefish",
        "sea dragons",
        "lionfish",
        "clownfish",
        "angelfish",
        "butterflyfish",
        "groupers",
        "snapper",
        "barracuda",
        "parrotfish",
        "wrasse",
        "damselfish",
        "blennies",
        "gobies",
        "scorpionfish",
        "stonefish",
        "triggerfish",
        "pufferfish",
        "boxfish",
        "moray eels",
        "electric eels",
        "manta rays",
        "stingrays",
        "horseshoe crabs",
        "sponges",
        "sea slugs",
        "nudibranchs",
        "barnacles",
        "amphipods",
        "isopods",
        "krill",
    ];

    fn comma_code(spam: &[&str]) -> String {
        match spam.len() {
            0 => String::new(),
            1 => spam[0].to_string(),
            _ => {
                // Pre-calculate the capacity
                let capacity =
                    spam.iter().map(|s| s.len()).sum::<usize>() + 2 * (spam.len() - 1) + 5;
                let mut result = String::with_capacity(capacity);

                // Use iterators to avoid the need for manual indexing
                let mut iter = spam.iter();
                if let Some(first) = iter.next() {
                    result.push_str(first);
                }

                let last = iter.next_back();
                for item in iter {
                    result.push_str(", ");
                    result.push_str(item);
                }

                if let Some(last_item) = last {
                    result.push_str(" and ");
                    result.push_str(last_item);
                }

                result
            }
        }
    }

    let result = comma_code(&spam);
    let elapsed_time = start_time.elapsed();
    println!("Result: {}", result);
    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
