use super::prelude::*;
use crate as nu_protocol;

#[derive(Clone, Copy, Debug, IntoValue, PartialEq, Eq, Serialize, Deserialize)]
pub struct LsConfig {
    pub use_ls_colors: bool,
    pub clickable_links: bool,
    pub use_sequences: bool,
}

impl Default for LsConfig {
    fn default() -> Self {
        Self {
            use_ls_colors: true,
            clickable_links: true,
            use_sequences: false,
        }
    }
}

impl UpdateFromValue for LsConfig {
    fn update<'a>(
        &mut self,
        value: &'a Value,
        path: &mut ConfigPath<'a>,
        errors: &mut ConfigErrors,
    ) {
        let Value::Record { val: record, .. } = value else {
            errors.type_mismatch(path, Type::record(), value);
            return;
        };

        for (col, val) in record.iter() {
            let path = &mut path.push(col);
            match col.as_str() {
                "use_ls_colors" => self.use_ls_colors.update(val, path, errors),
                "clickable_links" => self.clickable_links.update(val, path, errors),
                "use_sequences" => self.use_sequences.update(val, path, errors),
                _ => errors.unknown_option(path, val),
            }
        }
    }
}
