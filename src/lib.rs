use std::ops::Index;

struct Symbols;

impl Symbols {
    const SYMBOLS: [&'static [(Option<u8>, u8)]; 26] = [
        &[
            (Some(b'c'), 89), /* Actinium  */
            (Some(b'g'), 47), /* Silver    */
            (Some(b'l'), 13), /* Aluminum  */
            (Some(b'm'), 95), /* Americium */
            (Some(b'r'), 18), /* Argon     */
            (Some(b's'), 33), /* Arsenic   */
            (Some(b't'), 85), /* Astatine  */
            (Some(b'u'), 79), /* Gold      */
        ],
        &[
            (None, 5),         /* Boron     */
            (Some(b'a'), 56),  /* Barium    */
            (Some(b'e'), 4),   /* Beryllium */
            (Some(b'h'), 107), /* Bohrium   */
            (Some(b'i'), 83),  /* Bismuth   */
            (Some(b'k'), 97),  /* Berkelium */
            (Some(b'r'), 35),  /* Bromine   */
        ],
        &[
            (None, 6),         /* Carbon      */
            (Some(b'a'), 20),  /* Calcium     */
            (Some(b'd'), 48),  /* Cadmium     */
            (Some(b'e'), 58),  /* Cerium      */
            (Some(b'f'), 98),  /* Californium */
            (Some(b'l'), 17),  /* Chlorine    */
            (Some(b'm'), 96),  /* Curium      */
            (Some(b'n'), 112), /* Copernicium */
            (Some(b'o'), 27),  /* Cobalt      */
            (Some(b'r'), 24),  /* Chromium    */
            (Some(b's'), 55),  /* Caesium     */
            (Some(b'u'), 29),  /* Copper      */
        ],
        &[
            (Some(b'b'), 105), /* Dubnium      */
            (Some(b's'), 110), /* Darmstadtium */
            (Some(b'y'), 66),  /* Dysprosium   */
        ],
        &[
            (Some(b'r'), 68), /* Erbium      */
            (Some(b's'), 99), /* Einsteinium */
            (Some(b'u'), 63), /* Europium    */
        ],
        &[
            (None, 9),         /* Fluorine  */
            (Some(b'e'), 26),  /* Iron      */
            (Some(b'l'), 114), /* Flerovium */
            (Some(b'm'), 100), /* Fermium   */
            (Some(b'r'), 87),  /* Francium  */
        ],
        &[
            (Some(b'a'), 31), /* Gallium    */
            (Some(b'd'), 64), /* Gadolinium */
            (Some(b'e'), 32), /* Germanium  */
        ],
        &[
            (None, 1),         /* Hydrogen */
            (Some(b'e'), 2),   /* Helium   */
            (Some(b'f'), 72),  /* Hafnium  */
            (Some(b'g'), 80),  /* Mercury  */
            (Some(b'o'), 67),  /* Holmium  */
            (Some(b's'), 108), /* Hassium  */
        ],
        &[
            (None, 53),       /* Iodine  */
            (Some(b'n'), 49), /* Indium  */
            (Some(b'r'), 77), /* Iridium */
        ],
        &[], // J
        &[
            (None, 19),       /* Potassium */
            (Some(b'r'), 36), /* Krypton   */
        ],
        &[
            (Some(b'a'), 57),  /* Lanthanum   */
            (Some(b'i'), 3),   /* Lithium     */
            (Some(b'r'), 103), /* Lawrencium  */
            (Some(b'u'), 71),  /* Lutetium    */
            (Some(b'v'), 116), /* Livermorium */
        ],
        &[
            (Some(b'c'), 115), /* Moscovium   */
            (Some(b'd'), 101), /* Mendelevium */
            (Some(b'g'), 12),  /* Magnesium   */
            (Some(b'n'), 25),  /* Manganese   */
            (Some(b'o'), 42),  /* Molybdenum  */
            (Some(b't'), 109), /* Meitnerium  */
        ],
        &[
            (None, 7),         /* Nitrogen  */
            (Some(b'a'), 11),  /* Sodium    */
            (Some(b'b'), 41),  /* Niobium   */
            (Some(b'd'), 60),  /* Neodymium */
            (Some(b'e'), 10),  /* Neon      */
            (Some(b'h'), 113), /* Nihonium  */
            (Some(b'i'), 28),  /* Nickel    */
            (Some(b'o'), 102), /* Nobelium  */
            (Some(b'p'), 93),  /* Neptunium */
        ],
        &[
            (None, 8),         /* Oxygen    */
            (Some(b'g'), 118), /* Oganesson */
            (Some(b's'), 76),  /* Osmium    */
        ],
        &[
            (None, 15),       /* Phosphorus   */
            (Some(b'a'), 91), /* Protactinium */
            (Some(b'b'), 82), /* Lead         */
            (Some(b'd'), 46), /* Palladium    */
            (Some(b'm'), 61), /* Promethium   */
            (Some(b'o'), 84), /* Polonium     */
            (Some(b'r'), 59), /* Praseodymium */
            (Some(b't'), 78), /* Platinum     */
            (Some(b'u'), 94), /* Plutonium    */
        ],
        &[],
        &[
            (Some(b'a'), 88),  /* Radium        */
            (Some(b'b'), 37),  /* Rubidium      */
            (Some(b'e'), 75),  /* Rhenium       */
            (Some(b'f'), 104), /* Rutherfordium */
            (Some(b'g'), 111), /* Roentgenium   */
            (Some(b'h'), 45),  /* Rhodium       */
            (Some(b'n'), 86),  /* Radon         */
            (Some(b'u'), 44),  /* Ruthenium     */
        ],
        &[
            (None, 16),        /* Sulfur     */
            (Some(b'b'), 51),  /* Antimony   */
            (Some(b'c'), 21),  /* Scandium   */
            (Some(b'e'), 34),  /* Selenium   */
            (Some(b'g'), 106), /* Seaborgium */
            (Some(b'i'), 14),  /* Silicon    */
            (Some(b'm'), 62),  /* Samarium   */
            (Some(b'n'), 50),  /* Tin        */
            (Some(b'r'), 38),  /* Strontium  */
        ],
        &[
            (Some(b'a'), 73),  /* Tantalum   */
            (Some(b'b'), 65),  /* Terbium    */
            (Some(b'c'), 43),  /* Technetium */
            (Some(b'e'), 52),  /* Tellurium  */
            (Some(b'h'), 90),  /* Thorium    */
            (Some(b'i'), 22),  /* Titanium   */
            (Some(b'l'), 81),  /* Thallium   */
            (Some(b'm'), 69),  /* Thulium    */
            (Some(b's'), 117), /* Tennessine */
        ],
        &[(None, 92) /* Uranium */],
        &[(None, 23) /* Vanadium */],
        &[(None, 74) /* Tungsten */],
        &[(Some(b'e'), 54) /* Xenon */],
        &[
            (None, 39),       /* Yttrium   */
            (Some(b'b'), 70), /* Ytterbium */
        ],
        &[
            (Some(b'n'), 30), /* Zinc      */
            (Some(b'r'), 40), /* Zirconium */
        ],
    ];
}

impl Index<u8> for Symbols {
    type Output = [(Option<u8>, u8)];

    fn index(&self, letter: u8) -> &'static Self::Output {
        match letter {
            b'a'..=b'z' => Self::SYMBOLS[(letter - b'a') as usize],
            b'A'..=b'Z' => Self::SYMBOLS[(letter - b'A') as usize],
            _ => &[],
        }
    }
}

pub fn convert(word: &str) -> Vec<String> {
    let mut results = Vec::new();

    econ(word.as_bytes(), String::new(), &mut results);

    results
}

fn econ(word: &[u8], mut result: String, results: &mut Vec<String>) {
    if let Some(&first) = word.first() {
        if first == b' ' {
            result.push(' ');
            return econ(&word[1..], result, results);
        }
        for i in 0..Symbols[first].len() {
            let mut s = result.clone();
            s.push(first.to_ascii_uppercase() as char);
            if let (Some(c), _) = Symbols[first][i] {
                if word.len() > 1 && word[1] == c {
                    s.push(c as char);
                    econ(&word[2..], s, results);
                }
            } else {
                econ(&word[1..], s, results);
            }
        }

        return;
    }

    results.push(result);
}
