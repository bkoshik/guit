mod get {
    mod email;
    mod name;
}
mod display;
mod from;
mod new;
mod to_signature;

pub struct AuthorInfo {
    name: String,
    email: String,
}
