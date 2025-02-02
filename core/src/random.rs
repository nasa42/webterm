use rand::distr::uniform::SampleUniform;
use rand::distr::Alphanumeric;
use rand::Rng;

pub fn random_in_range<T>(min: T, max: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_alphanumeric(length: usize) -> String {
    let random_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub fn random_bytes<T: AsMut<[u8]>>(mut buffer: T) {
    let mut rng = rand::rng();
    rng.fill(buffer.as_mut());
}
