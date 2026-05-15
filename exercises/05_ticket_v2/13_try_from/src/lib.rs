// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::fmt::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl std::convert::TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let val= value.to_lowercase();
        if val == "todo" {
            return Ok(Status::ToDo);
        } else if val == "inprogress" {
            return Ok(Status::InProgress);
        } else if val == "done" {
            return Ok(Status::Done);
        }
        return Err("Invalid Value".to_string());
    }
}
impl std::convert::TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val= value.to_lowercase();
        if val == "todo" {
            return Ok(Status::ToDo);
        } else if val == "inprogress" {
            return Ok(Status::InProgress);
        } else if val == "done" {
            return Ok(Status::Done);
        }
        return Err("Invalid Value".to_string());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
