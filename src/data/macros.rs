#[macro_use]
pub mod macros {
    macro_rules! lazy_getter {
        ($self:ident, $name:ident, $t:ty, $b:block) => {
            pub fn $name(&$self) -> $t {
                if let Some(val) = unsafe { *$self.$name.get() } {
                    val
                } else {
                    let val = (|| $b)();
                    unsafe {
                        let r = $self.$name.get();
                        *r = Some(val);
                        (&*r).unwrap_unchecked()
                    }
                }
            }
        };
    }
    pub(crate) use lazy_getter;

    macro_rules! lazy_getter_ref {
        ($self:ident, $name:ident, $t:ty, $b:block) => {
            pub fn $name(&$self) -> &$t {
                if let Some(val) = unsafe { &*$self.$name.get() }.as_ref() {
                    val
                } else {
                    let val = (|| $b)();
                    unsafe {
                        let r = $self.$name.get();
                        *r = Some(val);
                        (&*r).as_ref().unwrap_unchecked()
                    }
                }
            }
        };
    }
    pub(crate) use lazy_getter_ref;
}
