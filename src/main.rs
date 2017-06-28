extern crate clap;
use clap::{Arg, App};

use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Orthography {
    BCMSLatin,
    BCMSCyrillic,
}

impl FromStr for Orthography {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Orthography::{BCMSLatin, BCMSCyrillic};
        if s == "LATIN" {
            Ok(BCMSLatin)
        } else if s == "CYRILLIC" {
            Ok(BCMSCyrillic)
        } else {
            Err("Must use either LATIN or CYRILLIC as inputs.")
        }
    }
}

struct NaturalText {
    orthography: Orthography,
    contents: String,
}

type ReplacementPairs<'a> = [(&'a str, &'a str); 60];

fn map_replace(replacements: ReplacementPairs, txt: &String) -> String {
    let mut out: String = txt.clone();
    for &(src, dst) in replacements.iter() {
        out = out.replace(src, dst);
    }
    out
}

macro_rules! mirrored {
    ($fwd:ident, $rev:ident, [$(($a:expr, $b:expr),)*]) => (
        const $fwd: ReplacementPairs = [ $( ($a, $b), )* ];
        const $rev: ReplacementPairs = [ $( ($b, $a), )* ];
    )
}

mirrored!(
    REPLACEMENTS,
    FLIPPED_REPLACEMENTS,
    [
        ("a", "а"),
        ("b", "б"),
        ("c", "ц"),
        ("č", "ч"),
        ("ć", "ћ"),
        ("dž", "џ"),
        ("d", "д"),
        ("đ", "ђ"),
        ("e", "е"),
        ("f", "ф"),
        ("g", "г"),
        ("h", "х"),
        ("i", "и"),
        ("j", "j"),
        ("k", "к"),
        ("lj", "љ"),
        ("l", "л"),
        ("m", "м"),
        ("nj", "њ"),
        ("n", "н"),
        ("o", "о"),
        ("p", "п"),
        ("r", "р"),
        ("s", "с"),
        ("š", "ш"),
        ("t", "т"),
        ("u", "у"),
        ("v", "в"),
        ("z", "з"),
        ("ž", "ж"),
        ("A", "А"),
        ("B", "Б"),
        ("C", "Ц"),
        ("Č", "Ч"),
        ("Ć", "Ћ"),
        ("DŽ", "Џ"),
        ("D", "Д"),
        ("Đ", "Ђ"),
        ("E", "Е"),
        ("F", "Ф"),
        ("G", "Г"),
        ("H", "Х"),
        ("I", "И"),
        ("J", "J"),
        ("K", "К"),
        ("LJ", "Љ"),
        ("L", "Л"),
        ("M", "М"),
        ("NJ", "Њ"),
        ("N", "Н"),
        ("O", "О"),
        ("P", "П"),
        ("R", "Р"),
        ("S", "С"),
        ("Š", "Ш"),
        ("T", "Т"),
        ("U", "У"),
        ("V", "В"),
        ("Z", "З"),
        ("Ž", "Ж"),
    ]
);

fn convert(dest: Orthography, txt: NaturalText) -> NaturalText {
    match txt.orthography {
        Orthography::BCMSLatin if dest != Orthography::BCMSLatin => NaturalText {
            orthography: Orthography::BCMSLatin,
            contents: map_replace(REPLACEMENTS, &txt.contents),
        },
        Orthography::BCMSCyrillic if dest != Orthography::BCMSCyrillic => NaturalText {
            orthography: Orthography::BCMSCyrillic,
            contents: map_replace(FLIPPED_REPLACEMENTS, &txt.contents),
        },
        _ => txt,
    }
}

fn main() {
    let args = App::new("BCMS Alphabet Converter")
        .version("0.1")
        .author("Yghor Kerscher <kerscher@acm.org>")
        .about(
            "Converts text between Latin and Cyrillic versions of the BCMS language.",
        )
        .arg(
            Arg::with_name("alphabet")
                .short("a")
                .long("alphabet")
                .value_name("LATIN | CYRILLIC")
                .help("Which alphabet to convert to")
                .required(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("File to read text from")
                .required(true)
                .index(1),
        )
        .get_matches();

    let orthography = Orthography::from_str(args.value_of("alphabet").unwrap()).unwrap();
    let input_file = File::open(args.value_of("INPUT").unwrap()).unwrap();
    let buf = BufReader::new(input_file);


    use Orthography::{BCMSLatin, BCMSCyrillic};
    let target = if orthography == BCMSLatin {
        BCMSCyrillic
    } else {
        BCMSLatin
    };

    for line in buf.lines() {
        if line.is_ok() {
            let conv = convert(
                target,
                NaturalText {
                    orthography: orthography,
                    contents: line.unwrap(),
                },
            );
            println!("{}", conv.contents);
        }
    }
}
