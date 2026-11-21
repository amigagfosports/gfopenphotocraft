// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Service {
    UserInterface,
    Media,
    Updates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Binding {
    pub service: Service,
    pub backend: &'static str,
}

pub const BINDINGS: [Binding; 3] = [
    Binding { service: Service::UserInterface, backend: "GFOpenLayout and OpenGadTools" },
    Binding { service: Service::Media, backend: "GFOpenDataTypes" },
    Binding { service: Service::Updates, backend: "GFOpenUp" },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_native_service_has_one_backend() {
        assert_eq!(BINDINGS.len(), 3);
        for (index, binding) in BINDINGS.iter().enumerate() {
            assert!(!binding.backend.is_empty());
            for other in BINDINGS.iter().skip(index + 1) {
                assert_ne!(binding.service, other.service);
            }
        }
    }
}
