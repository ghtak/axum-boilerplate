use log4rs;

pub(crate) fn init_file(yaml_file:&'static str) -> anyhow::Result<()>{
    log4rs::init_file(
        yaml_file, 
        Default::default()
    )
}