use rand::distributions::uniform::SampleUniform;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_in_range<T>(min: T, max: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_string(length: usize) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}
