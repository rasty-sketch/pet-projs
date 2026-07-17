use rand::RngExt;

static LATIN_QUOTES: [&str; 10] = [
    "Memento mori.",
    "Amor fati.",
    "Per aspera ad astra.",
    "Dum spiro, spero.",
    "Fortuna audaces iuvat.",
    "Nosce te ipsum.",
    "Tempus fugit.",
    "Carpe diem.",
    "Faber est suae quisque fortunae.",
    "Non ducor, duco.",
];

pub fn random_quote() -> &'static str {
    let mut num = rand::rng();

    let random = num.random_range(0..10);

    LATIN_QUOTES[random]
}

pub fn english_translation(quote: &str) -> &'static str {
    match quote {
        "Memento mori." => {
            "Remember that you will die; let mortality determine what deserves your time."
        }

        "Amor fati." => "Love your fate, including the suffering that shaped you.",

        "Per aspera ad astra." => "The path toward greatness usually passes through hardship.",

        "Dum spiro, spero." => "As long as life remains, so does the possibility of change.",

        "Fortuna audaces iuvat." => "Opportunity tends to reveal itself to those willing to act.",

        "Nosce te ipsum." => {
            "Understanding yourself is harder and more valuable than judging others."
        }

        "Tempus fugit." => {
            "Time does not merely pass; it quietly consumes every unused possibility."
        }

        "Carpe diem." => "Use the present deliberately, because tomorrow is never guaranteed.",

        "Faber est suae quisque fortunae." => {
            "A person gradually constructs their future through repeated choices."
        }

        "Non ducor, duco." => {
            "Do not merely follow the direction of circumstances; create direction."
        }

        _ => "Translation not found.",
    }
}
