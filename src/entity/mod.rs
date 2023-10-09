mod sample;
mod user;

pub(crate) use self::sample::Sample;
pub(crate) use self::user::User;

pub(crate) trait Entity: Sync + Send
{
    type ID : Sync + Send;

    fn get_id(&self) -> &Self::ID;
}
