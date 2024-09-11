use dotenvy::Error as DotenvError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    DotenvError(DotenvError),
}

impl From<DotenvError> for ConfigError {
    fn from(e: DotenvError) -> Self {
        ConfigError::DotenvError(e)
    }
}

pub struct Config;

impl Config {
    pub fn init() -> Result<(), ConfigError> {
        tracing_subscriber::fmt()
            .pretty()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .init();

        dotenvy::dotenv()?;
        Ok(())
    }
}
