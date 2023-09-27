mod sample;

pub(crate) use self::sample::Sample;

pub(crate) static NEW_REDORD_ID : i64 = -1;

pub(crate) trait Entity {
    fn is_new(&self) -> bool;
}