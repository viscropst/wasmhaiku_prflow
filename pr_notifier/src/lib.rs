#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[allow(unused_imports)]
use wasmedge_bindgen::*;
#[wasmedge_bindgen_macro::wasmedge_bindgen]
pub fn run(msg :String) -> Result<String,String> {
    return Ok(msg);
}