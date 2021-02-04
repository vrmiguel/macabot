/// Decides if the bot should randomly respond to a message
pub const SAY_SOMETHING: &[bool] = &[true, false];
pub const SAY_SOMETHING_WEIGHTS: &[i32] = &[3, 97];

pub const RAND_PHRASES: &[&str] = &[
    "vc é ancap ??",
    "Concordo.",
    "Isso aí que você tá falando é coisa de comunista",
];

pub const RAND_PHRASES_WEIGHTS: &[i32] = &[2, 1, 1];

pub const USER_LEFT: &[&str] = &["Vai pela sombra, USER!", "Volte USER, eu não mereço esse castigo!\nPode até brigar comigo\nMas não me deixe nessa solidão"];
pub const USER_LEFT_WEIGHTS: &[i32] = &[1, 1];

pub const PINNED_MESSAGE: &[&str] = &["Coisa de desocupado", "1, 2, 3, pin!"];
pub const PINNED_MESSAGE_WEIGHTS: &[i32] = &[2, 3];
