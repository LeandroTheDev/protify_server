use std::collections::HashMap;

pub struct ItemDownload {
    token: String,
    item_id: u32,
    files: HashMap<String, [u128; 2]>,
}
impl ItemDownload {
    pub fn new(_token: String, _item_id: u32, _files: HashMap<String, [u128; 2]>) -> ItemDownload {
        Self {
            token: _token,
            item_id: _item_id,
            files: _files,
        }
    }

    /// true if success updated the item downloading, false if not
    pub fn update_file_byte_remaining(&mut self, file_name: String, bytes_remaining: u128) -> bool {
        if let Some(bytes) = self.files.get_mut(&file_name) {
            bytes[1] = bytes_remaining;
            true
        } else {
            false
        }
    }
}
