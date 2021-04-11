use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

pub struct Output {
    inner: Box<dyn Write>,
    is_json: bool,
    write_no: usize,
}

impl Output {
    pub fn new(output_file: &Option<PathBuf>, is_json: bool) -> Self {
        let mut inner: Box<dyn Write> = match output_file {
            Some(file) => {
                let file = fs::File::create(file).unwrap();
                colored::control::set_override(false);
                Box::new(file)
            }
            None => Box::new(io::stdout()),
        };

        if is_json {
            write!(inner, "[\n").unwrap();
        }

        Self {
            inner,
            is_json,
            write_no: 0,
        }
    }

    pub fn write(&mut self, message: String) {
        if self.is_json {
            return;
        }

        write!(self.inner, "{}\n", message).unwrap();
    }

    pub fn write_json(&mut self, message: String) {
        if self.write_no > 0 {
            write!(self.inner, ",\n").unwrap();
        }

        write!(self.inner, "{}", message).unwrap();

        self.write_no += 1;
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        if self.is_json {
            write!(self.inner, "]\n").unwrap();
        }
    }
}
