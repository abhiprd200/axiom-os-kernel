use crate::println;

#[derive(Debug, Clone)]
pub struct TrustedData<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
    pub expected_hash: [u8; 32],
}

impl<'a> TrustedData<'a> {
    pub fn new(name: &'a str, data: &'a [u8]) -> Self {
        let hash = provenance_hash(data);
        TrustedData { name, data, expected_hash: hash }
    }

    pub fn verify_or_halt(&self) -> bool {
        let current_hash = provenance_hash(self.data);
        if current_hash != self.expected_hash {
            println!("[AXIOM KERNEL] PROVENANCE VIOLATION: \"{}\"", self.name);
            println!("[AXIOM KERNEL] EXECUTION BLOCKED");
            return false;
        }
        println!("[AXIOM KERNEL] VERIFIED: \"{}\"", self.name);
        true
    }
}

pub fn provenance_hash(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

pub fn tamper(_data: &[u8]) -> &'static [u8] {
    b"TAMPERED: Journalist report from Jharkhand"
}
