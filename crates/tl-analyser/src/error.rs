use crate::context::Type;

#[derive(Debug)]
pub enum AnalysisErr {
    Undefined(String),
    TypeMismatch { expected: Type, found: Type },
    MissingAnnotation(String),
    AlreadyDefined(String)
}