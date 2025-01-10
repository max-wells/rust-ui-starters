use regex::Regex;
use serde::Serialize;
use validator_derive::Validate;

const REGEX_PATTERN_SPECIAL_CHARACTERS: &str = r"^[a-zA-Z0-9\s]+$";

#[derive(Debug, Validate, Serialize)]
pub struct XxxsFormValidator {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    #[validate(length(max = 10, message = "Title cannot be longer than 10 characters"))]
    #[validate(contains(pattern = "z", message = "Must contain the letter 'z' 😄"))]
    // #[validate(does_not_contain(pattern = "TITLE", message = "Must not contain TITLE"))]
    #[validate(custom = "fn_validate_no_special_characters")]
    pub title: String,

    #[validate(length(min = 1, message = "Author cannot be empty"))]
    pub author: String,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// * 💁 Internal
fn fn_validate_no_special_characters(title: &str) -> Result<(), validator::ValidationError> {
    let re = Regex::new(REGEX_PATTERN_SPECIAL_CHARACTERS).unwrap();
    if !re.is_match(title) {
        let mut error = validator::ValidationError::new("special_characters");
        error.message = Some("Title cannot contain special characters".into());
        return Err(error);
    }
    Ok(())
}
