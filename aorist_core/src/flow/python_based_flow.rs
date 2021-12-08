
use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::python::{NativePythonPreamble, PythonPreamble, RPythonPreamble};
use aorist_primitives::TPrestoEndpoints;
use aorist_primitives::{AString, AVec, AoristUniverse};
use pyo3::prelude::*;

pub trait PythonBasedFlow<U>: ETLFlow<U>
where
    U: AoristUniverse,
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<AString>;
    fn get_python_preamble(&self) -> PyResult<AVec<PythonPreamble>> {
        let preambles = match self.get_dialect() {
            Some(Dialect::Python(_)) => match self.get_preamble_string() {
                Some(p) => Ok(vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p)?,
                )]),
                None => Ok(AVec::new()),
            },
            Some(Dialect::R(_)) => match self.get_preamble_string() {
                Some(p) => Ok(vec![PythonPreamble::RPythonPreamble(RPythonPreamble::new(
                    p,
                )?)]),
                None => Ok(AVec::new()),
            },
            _ => Ok(AVec::new()),
        };
        preambles
    }
}
