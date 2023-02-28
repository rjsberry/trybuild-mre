pub struct Assert<const L: usize, const R: usize>;

impl<const L: usize, const R: usize> Assert<L, R> {
    pub const GREATER_EQ: usize = L - R;
}

#[allow(path_statements)]
pub fn const_assert_gteq<const L: usize, const R: usize>() {
    Assert::<L, R>::GREATER_EQ;
}
