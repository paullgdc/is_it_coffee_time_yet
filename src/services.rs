use stdweb::{js, unstable::TryInto, Value};

pub fn random() -> f64 {
    let rand: Value = js! {
        return Math.random();
    };
    rand.try_into()
        .expect("Couldn't convert random value to f64")
}
