use iscc::{content_id_text, data_id_reader, instance_id_reader, meta_id};
use std::error::Error;

pub fn get_iscc_id_text(title: &str, content: &str) -> Result<String, Box<dyn Error>> {
    // Generate ISCC Component Codes
    let (mid, _title, _extra) = meta_id(title, "");
    let cid = content_id_text(content, false);
    let did = data_id_reader(&mut content.as_bytes());
    let (iid, _tophash) = instance_id_reader(&mut content.as_bytes());

    // Join ISCC Components to fully qualified ISCC Code
    let iscc_code = [mid, cid, did, iid].join("-");
    Ok(iscc_code)
}
