use std::ops::Index;

const PREFIX_EV: &str = "Github PR";

#[allow(unused_imports)]
use wasmedge_bindgen::*;

#[wasmedge_bindgen_macro::wasmedge_bindgen]
pub fn run(event: String) -> String {
    let mut final_msg = String::from(PREFIX_EV);
    final_msg.push_str(": ");

    let owned_event = event.as_str().to_owned();

    final_msg.push_str(judge_msg(&owned_event).as_str());
    return final_msg;
}

fn judge_msg<'a>(owned_event: &'a str) -> String {
    let substr_len = PREFIX_EV.len() + 1;
    let ev_substr = owned_event.index(0..substr_len);
    if ev_substr != format!("{},", PREFIX_EV) {
        return format!("you typed {}", owned_event);
    } else {
        let told = owned_event.index(substr_len..owned_event.len());
        match told {
            "" => return String::from("You said Noting"),
            _ => return String::from(told),
        };
    }
}

#[test]
pub fn test_run() {
    let owned_event = "Github PR,";
    assert_eq!(
        format!("You said Noting"),
        judge_msg(&owned_event.to_owned())
    );
    let owned_event = "Github PR,Hello";
    assert_eq!(format!("Hello"), judge_msg(&owned_event.to_owned()));

    let owned_event = "GitHub PR,";
    assert_eq!(format!("you typed {}", owned_event), judge_msg(owned_event));
    let owned_event = "GitHub PR,Hello";
    assert_eq!(format!("you typed {}", owned_event), judge_msg(owned_event));
}
