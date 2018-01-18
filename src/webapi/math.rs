use webcore::try_from::TryInto;
use webcore::value::Reference;
use webcore::once::Once;
use webcore::value::Value;

pub struct Math( Reference );

reference_boilerplate! { Math, instanceof Math }

/// The global [Math object](struct.Math.html).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math)
pub fn math() -> Math {
    unsafe { js!( return Math; ).into_reference_unchecked() }.unwrap()
}

impl Math {
    pub fn random(&self) -> f64 {
        js!( return @{self}.random() ).try_into().unwrap()
    }
}
