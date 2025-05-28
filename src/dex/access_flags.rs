use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct AccessFlags: u32 {
        const PUBLIC = 0x1;
        const PRIVATE = 0x2;
        const PROTECTED = 0x4;
        const STATIC = 0x8;
        const FINAL = 0x10;
        const SYNCHRONIZED = 0x20;
        const VOLATILE = 0x40;
        const BRIDGE = 0x40;
        const TRANSIENT = 0x80;
        const VARARGS = 0x80;
        const NATIVE = 0x100;
        const INTERFACE = 0x200;
        const ABSTRACT = 0x400;
        const STRICT = 0x800;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const CONSTRUCTOR = 0x10000;
        const DECLARED_SYNCHRONIZED = 0x20000;
    }
}

impl AccessFlags {
    pub fn to_human_readable(&self) -> String {
        let mut flags = Vec::new();
        if self.contains(AccessFlags::PUBLIC) {
            flags.push("public");
        }
        if self.contains(AccessFlags::PRIVATE) {
            flags.push("private");
        }
        if self.contains(AccessFlags::PROTECTED) {
            flags.push("protected");
        }
        if self.contains(AccessFlags::STATIC) {
            flags.push("static");
        }
        if self.contains(AccessFlags::FINAL) {
            flags.push("final");
        }
        if self.contains(AccessFlags::SYNCHRONIZED) {
            flags.push("synchronized");
        }
        if self.contains(AccessFlags::VOLATILE) {
            flags.push("volatile");
        }
        if self.contains(AccessFlags::TRANSIENT) {
            flags.push("transient");
        }
        if self.contains(AccessFlags::NATIVE) {
            flags.push("native");
        }
        if self.contains(AccessFlags::INTERFACE) {
            flags.push("interface");
        }
        if self.contains(AccessFlags::ABSTRACT) {
            flags.push("abstract");
        }
        if self.contains(AccessFlags::STRICT) {
            flags.push("strictfp");
        }
        if self.contains(AccessFlags::CONSTRUCTOR) {
            flags.push("constructor");
        }
        if self.contains(AccessFlags::SYNCHRONIZED) {
            flags.push("synchronized");
        }
        return flags.join(" ");
    }
}
