pub fn add(left: usize, right: usize) -> usize {
    unsafe { temp(); }
    left + right
}

#[link(name="stretch", kind="static")]
extern "C" {
    fn temp();
}
