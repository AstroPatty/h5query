use hdf5_metno::{Group, H5Type, Selection};
use ndarray::{ArrayD, IxDyn};
use std::io::Result;

pub(crate) trait Column {
    fn instantiate<T, U>(&self, group: Group, index: Option<U>) -> Result<ArrayD<T>>
    where
        T: H5Type,
        U: Into<Selection>;
}

impl Column for String {
    fn instantiate<T, U>(&self, group: Group, index: Option<U>) -> Result<ArrayD<T>>
    where
        T: H5Type,
        U: Into<Selection>,
    {
        let ds = group.dataset(self)?;
        let data = if let Some(ix) = index {
            ds.read_slice::<T, U, IxDyn>(ix)?
        } else {
            ds.read::<T, IxDyn>()?
        };

        Ok(data)
    }
}
