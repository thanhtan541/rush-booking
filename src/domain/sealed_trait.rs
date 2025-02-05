mod private {
    pub struct Token;
    pub trait Sealed {}
}

pub trait SealedTrait: private::Sealed {
    fn method(&self);
    fn sealed_method(&self, _: private::Token);
}

pub struct ExportedType {}

impl private::Sealed for ExportedType {}

// Partial-sealed trait
impl SealedTrait for ExportedType {
    // Downstream can overide this method
    fn method(&self) {}
    // Downstream can read the method but not able to call it due to missing parameter.
    fn sealed_method(&self, token: private::Token) {}
}

// This is a workaround,
// another solution is comming:
// [Final keyword](https://internals.rust-lang.org/t/pre-rfc-final-trait-methods/18407)
pub fn call_to_sealed_meothod(value: &impl SealedTrait) {
    value.sealed_method(private::Token {});
}
