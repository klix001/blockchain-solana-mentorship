
use std::str::FromStr;

pub enum Route {
    Student,
    StudentManager,
    Teacher,
}

impl FromStr for Route {
    type Err = String;

    fn from_str(route: &str) -> Result<Self, Self::Err> {
        match route.to_lowercase().as_str() {
            "student"         => Ok(Route::Student),
            "admin" => Ok(Route::StudentManager),
            "teacher"         => Ok(Route::Teacher),
            _                 => Err(String::from("Enter a valid role")),
        }
    }
}

pub enum AdminAction {
    AddStudent,
    AddTeacher,
    ViewStudent,
    ClassAverage,
    Delete,
    UpdateScore,
    PassStatus,
    EligibilityReport,
    Summary,
    Exit,
}

impl FromStr for AdminAction {
    type Err = String;

    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match action.to_lowercase().as_str() {
            "add student"        => Ok(AdminAction::AddStudent),
            "add teacher"        => Ok(AdminAction::AddTeacher),
            "class list"         => Ok(AdminAction::ViewStudent),
            "class average"      => Ok(AdminAction::ClassAverage),
            "delete student"     => Ok(AdminAction::Delete),
            "update score"       => Ok(AdminAction::UpdateScore),
            "pass report"        => Ok(AdminAction::PassStatus),
            "eligibility report" => Ok(AdminAction::EligibilityReport),
            "student summary"    => Ok(AdminAction::Summary),
            "exit"               => Ok(AdminAction::Exit),
            _                    => Err(String::from("Invalid action")),
        }
    }
}

pub enum StudentAction{
    ViewProfile,
    PassReport,
    ScholarshipEligibility,
    HonouraryStatus,
    Grade,
    Exit,
}

impl FromStr for StudentAction{
    type Err = String;

    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
            "view profile" => Ok(StudentAction::ViewProfile),
            "pass report" => Ok(StudentAction::PassReport),
            "eligibility" => Ok(StudentAction::ScholarshipEligibility),
            "honourary status" => Ok(StudentAction::HonouraryStatus),
            "grade" => Ok(StudentAction::Grade),
            "exit" =>Ok(StudentAction::Exit),
            _ => Err(format!("Invalid student action")),
        }
    }
}

pub enum TeacherAction{
    UpdateScore,
    GenerateReport,
    ClassList,
    Evaluation,
    ClassStatistics,
    Summary,
    Exit,
}

impl FromStr for TeacherAction{
    type Err = String;

    fn from_str(action:&str)->Result<Self, Self::Err>{
        match action.to_lowercase().as_str(){
            "update score" => Ok(TeacherAction::UpdateScore),
            "generate report" => Ok(TeacherAction::GenerateReport),
            "class list" => Ok(TeacherAction::ClassList),
            "evaluation" => Ok(TeacherAction::Evaluation),
            "class stat" => Ok(TeacherAction::ClassStatistics),
            "student summary" => Ok(TeacherAction::Summary),
            "exit" =>Ok(TeacherAction::Exit),
            _ => Err(format!("Invalid teacher action")),
        }
    }
}