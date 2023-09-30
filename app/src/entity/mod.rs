mod sample;

pub(crate) use self::sample::Sample;

pub(crate) trait Entity: Sync + Send
{
    type ID : Sync + Send;

    fn get_id(&self) -> &Self::ID;
}
