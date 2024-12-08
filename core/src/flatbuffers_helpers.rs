use flatbuffers::{Follow, InvalidFlatbuffer, Verifiable};

pub fn read_message<'buf, T>(data: &'buf [u8]) -> Result<T::Inner, InvalidFlatbuffer>
where
    T: 'buf + Follow<'buf> + Verifiable,
{
    flatbuffers::root::<T>(data)
}
