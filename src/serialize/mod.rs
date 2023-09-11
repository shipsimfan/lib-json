mod into_json;
mod output;

pub use into_json::{ArrayIter, ObjectIter, ToJSON};
pub use output::{FormatterOutput, Output};

macro_rules! try_write {
    ($expr: expr, $result: expr) => {
        match $expr {
            Ok(()) => {}
            Err(error) => {
                $result = Err(error);
                return false;
            }
        }
    };
}

pub trait Serialize {
    fn serialize<O: Output>(&self, output: &mut O) -> Result<(), O::Error>;
}

impl<T: ToJSON> Serialize for T {
    fn serialize<O: Output>(&self, output: &mut O) -> Result<(), O::Error> {
        serialize(output, self)
    }
}

pub fn serialize<O: Output>(output: &mut O, value: &dyn ToJSON) -> Result<(), O::Error> {
    if let Some(object_iter) = value.object_iter() {
        output.write(&[b'{'])?;

        let mut first = true;
        let mut result = Ok(());
        object_iter.for_each(&mut |key, value| {
            if first {
                first = false;
            } else {
                try_write!(output.write(&[b',']), result);
            }

            try_write!(write!(output, "{}:", key), result);
            try_write!(serialize(output, value), result);

            true
        });
        result?;

        output.write(&[b'}'])
    } else if let Some(array_iter) = value.array_iter() {
        output.write(&[b'['])?;

        let mut first = true;
        let mut result = Ok(());
        array_iter.for_each(&mut |value| {
            if first {
                first = false;
            } else {
                try_write!(output.write(&[b',']), result);
            }

            try_write!(serialize(output, value), result);

            true
        });
        result?;

        output.write(&[b']'])
    } else {
        write!(output, "{}", value.to_json())
    }
}
