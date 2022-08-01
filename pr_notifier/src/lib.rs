use gjson;
#[allow(unused_imports)]
use wasmedge_bindgen::*;

const PREFIX_EV: &str = "Github PR";

#[wasmedge_bindgen_macro::wasmedge_bindgen]
pub fn run(event: String) -> Result<String, String> {
    let mut final_msg = String::from(PREFIX_EV);
    final_msg.push_str(": ");

    let owned_event = event.as_str().to_owned();

    final_msg.push_str(gen_tmpmsg(&owned_event).as_str());
    final_msg.push_str("!");
    return Ok(final_msg);
}

fn gen_tmpmsg<'a>(owned_event: &'a str) -> String {
    let action = gjson::get(&owned_event, "action");
    let usr_name = gjson::get(&owned_event, "pull_request.user.login");
    let pr_num = gjson::get(&owned_event, "pull_request.number");

    let mut tmp_msg = String::from("User named ");
    tmp_msg.push_str(usr_name.str());
    tmp_msg.push_str(" ");
    tmp_msg.push_str(action.str());
    tmp_msg.push_str(" a pull request");

    if action.str() == "opened" || action.str() == "reopened" {
        let is_draft = gjson::get(owned_event, "pull_request.draft");
        if is_draft.bool() {
            tmp_msg.push_str(" as a draft");
        }
        let is_mergable = gjson::get(owned_event, "pull_request.mergeable");
        if is_mergable.bool() {
            tmp_msg.push_str(
                format!(
                    "
                , You can merge this PR by telling me 
                \" Github PR,Merge #{}\"",
                    pr_num.str()
                )
                .as_str(),
            );
        }
    }
    return tmp_msg;
}
