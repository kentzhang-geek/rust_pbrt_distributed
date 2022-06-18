#[link(name = "rpd_fbx_plugin", kind = "static")]
extern "C" {
    fn fileToSceneBuffer(filename: *mut u8, pathlen: i32, size_len: *mut i32) -> *mut u8;
    fn printHello();
}

pub fn fileToScene(mut filename: String) -> Option<Vec<u8>> {
    let mut size = 0i32;
    unsafe {
        let data = fileToSceneBuffer(filename.as_mut_ptr(), filename.len() as i32, &mut size);
        let dataref = std::slice::from_raw_parts(data, size as usize);
        let ret = Vec::from(dataref);
        return Some(ret);
    }
    return None;
}