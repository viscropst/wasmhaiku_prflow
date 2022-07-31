#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use gjson;
#[allow(unused_imports)]
use wasmedge_bindgen::*;

#[wasmedge_bindgen_macro::wasmedge_bindgen]
pub fn run(event :String) -> Result<String,String> {
    let mut final_msg = String::from("Github PR: ");
    
    let owned_event = event.as_str().to_owned();
    let action = gjson::get(&owned_event, "action");
    let usr_name = gjson::get(&owned_event, "pull_request.user.login");

    let mut tmp_msg = String::from("User named ");
    tmp_msg.push_str(usr_name.str());
    tmp_msg.push_str(" ");
    tmp_msg.push_str(action.str());
    tmp_msg.push_str(" a pull request");

    if action.str() == "opened" || action.str() == "reopened" {
        let is_draft = gjson::get(&owned_event, "pull_request.draft");
        if is_draft.bool() {
            tmp_msg.push_str(" as a draft");  
        }
    }

    final_msg.push_str(tmp_msg.as_str());
    final_msg.push_str("!");
    return Ok(final_msg);
}