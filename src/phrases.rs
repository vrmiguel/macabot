/// Decides if the bot should randomly respond to a message
pub const SAY_SOMETHING: &[bool] = &[true, false];
pub const SAY_SOMETHING_WEIGHTS: &[i32] = &[1, 105];

pub const RAND_PHRASES: &[&str] = &[
    "vc é ancap ??",
    "Concordo.",
    "Isso aí que você tá falando é coisa de comunista",
    "tive um passarinho que morreu assim",
    "Já falei sobre isso no Medium.",
    "Impressionante! Nada do que você fala faz sentido."
];
pub const RAND_PHRASES_WEIGHTS: &[i32] = &[1, 2, 2, 2, 2, 2];   

pub const BAIT: &[&str] = &[
    "Curioso como o governo Bolsonaro tem muitas semelhanças com Lula I",
    "Vocês acham preferível PJ ou CLT??",
    "Pessoal, o que vocês acham sobre FGTS?",
    "estamos refazendo o backend na minha empresa e agora estou em dúvida\n\nme recomendam utilizar Rust ou Go?"
];

pub const BAIT_WEIGHTS: &[i32] = &[1, 1, 2, 1];   
pub const USER_LEFT: &[&str] = &["Vai pela sombra, USER!", "Volte USER, eu não mereço esse castigo!\nPode até brigar comigo\nMas não me deixe nessa solidão"];
pub const USER_LEFT_WEIGHTS: &[i32] = &[1, 1];

pub const PINNED_MESSAGE: &[&str] = &["Coisa de desocupado", "1, 2, 3, pin!"];
pub const PINNED_MESSAGE_WEIGHTS: &[i32] = &[2, 3];
