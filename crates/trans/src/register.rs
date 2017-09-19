#[derive(Debug, Copy, Clone)]
pub enum Register32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

#[cfg(test)]
mod test {}
