// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
impl std::ops::Div<SquareRootPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: SquareRootPrefixOrPostfix) -> Self::Output {
        self.square_root()
    }
}
impl std::ops::DivAssign<SquareRootPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: SquareRootPrefixOrPostfix) {
        *self = self.square_root()
    }
}
impl SquareRoot for Scalar {
    fn square_root(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ f32::powf(self[scalar], 0.5))
    }
}
