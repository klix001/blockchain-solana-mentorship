use std::{fmt, num::{ParseIntError, ParseFloatError}};

#[derive(Debug)]
pub enum SystemError{
    EmptyDatabase,
    StudentNotFound,
    ParseError(Box<dyn std::error::Error>),
    AddStudentFailure,
    AddTeacherFailure
}

impl fmt::Display for SystemError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            SystemError::StudentNotFound => write!(f, "student not found"),
            SystemError::EmptyDatabase => write!(f, "empty database"),
            SystemError::ParseError(e) => write!(f, "{}", e),
            SystemError::AddStudentFailure => write!(f, "Check if the name is empty | age > 16 | score < 100"),
            SystemError::AddTeacherFailure => write!(f, "add teacher failure, Reason: there is an empty field"),
        }
    }
}


impl From<ParseIntError> for SystemError {
    fn from(e: ParseIntError) -> Self {
        SystemError::ParseError(Box::new(e)) 

    }
}
impl From<ParseFloatError> for SystemError {
    fn from(e: ParseFloatError) -> Self {
        SystemError::ParseError(Box::new(e))  
    }
}

impl std::error::Error for  SystemError{}
