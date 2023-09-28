mod sample;

pub(crate) use self::sample::Sample;

pub(crate) static NEW_REDORD_ID: i64 = -1;

pub(crate) trait Entity {
    const NEW_ENTITY_ID: i64 = -1;
    fn is_new(&self) -> bool;
}
