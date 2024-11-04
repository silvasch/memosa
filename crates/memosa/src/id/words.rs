use rand::{seq::SliceRandom, thread_rng};

const WORDS: [&str; 50] = [
    "enemy", "torch", "heavy", "reign", "award", "large", "dream", "short", "stake", "pluck",
    "waist", "match", "cheat", "chair", "ready", "stain", "moral", "novel", "quote", "nerve",
    "prove", "south", "brick", "radio", "petty", "craft", "metal", "twist", "swipe", "error",
    "young", "scale", "bland", "sweet", "brush", "knock", "trust", "brand", "trace", "outer",
    "money", "plant", "berry", "beard", "layer", "cream", "breed", "share", "frank", "snail",
];

pub fn get_random_word() -> &'static str {
    return WORDS
        .choose(&mut thread_rng())
        .expect("WORDS should never be empty");
}
