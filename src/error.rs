use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShranError<'ebuf> {
    #[error("Error: {msg:?} file does not exist\nFile: {file:?}\nLine: {line:?}")]
    BuildFileError {
        msg: String,
        file: &'ebuf str,
        line: u32,
    },
}

pub type ShranErrorType<'ebuf, T> = Result<T, ShranError<'ebuf>>;
