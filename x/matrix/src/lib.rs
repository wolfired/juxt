use std::mem::MaybeUninit;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ptr::write;

pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait Magnitude {
    fn magnitude(self) -> f32;
}

pub trait Normalize {
    type Output;

    fn normalize(self) -> Self::Output;
}

pub trait NormalizeAssign {
    fn normalize_assign(&mut self);
}

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

pub trait CrossAssign<Rhs = Self> {
    fn cross_assign(&mut self, rhs: Rhs);
}

pub trait Transpose {
    type Output;

    fn transpose(self) -> Self::Output;
}

pub trait TransposeAssign<Rhs = Self> {
    fn transpose_assign(&mut self);
}

macro_rules! get_set_def {
    ($id00:ident, $id01:ident, $id10:ident, $id11:ident, $id20:ident, $id21:ident $(,)?) => {
        pub trait $id00<T> {
            fn $id01(&self) -> T;
        }
        pub trait $id10<T> {
            fn $id11(&self) -> &T;
        }
        pub trait $id20<T> {
            fn $id21(&mut self) -> &mut T;
        }
    };
}
get_set_def!(X, x, XRef, x_ref, XMut, x_mut,);
get_set_def!(Y, y, YRef, y_ref, YMut, y_mut,);
get_set_def!(Z, z, ZRef, z_ref, ZMut, z_mut,);
get_set_def!(W, w, WRef, w_ref, WMut, w_mut,);

macro_rules! impl_zero_one {
    ($($t:ty),*$(,)?) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0 as $t
                }
            }

            impl One for $t {
                fn one() -> Self {
                    1 as $t
                }
            }
        )*
    };
}
impl_zero_one!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,);

mod inner {
    use super::One;
    use super::Transpose;
    use super::TransposeAssign;
    use super::Zero;
    use std::mem::MaybeUninit;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Div;
    use std::ops::DivAssign;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Sub;
    use std::ops::SubAssign;
    use std::ptr::read;

    #[derive(Debug)]
    pub struct MatrixInner<const R: usize, const C: usize, T> {
        pub(super) elements: [[T; C]; R],
    }

    impl<const R: usize, const C: usize, T: Clone> Clone for MatrixInner<R, C, T> {
        fn clone(&self) -> Self {
            Self { elements: self.elements.clone() }
        }
    }

    impl<const R: usize, const C: usize, T: Copy> Copy for MatrixInner<R, C, T> {}

    impl<const R: usize, const C: usize, T: Default> Default for MatrixInner<R, C, T> {
        fn default() -> Self {
            let mut out: Self = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = T::default();
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: PartialEq> PartialEq for MatrixInner<R, C, T> {
        fn eq(&self, other: &Self) -> bool {
            self.elements == other.elements
        }
    }

    impl<const R: usize, const C: usize, T: Eq> Eq for MatrixInner<R, C, T> {}

    impl<const R: usize, const C: usize, T> From<[[T; C]; R]> for MatrixInner<R, C, T> {
        fn from(value: [[T; C]; R]) -> Self {
            Self { elements: value }
        }
    }

    impl<const R: usize, const C: usize> From<MatrixInner<R, C, f32>> for MatrixInner<R, C, i32> {
        fn from(value: MatrixInner<R, C, f32>) -> Self {
            let mut out: Self = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = value.elements[r][c] as i32;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize> From<MatrixInner<R, C, i32>> for MatrixInner<R, C, f32> {
        fn from(value: MatrixInner<R, C, i32>) -> Self {
            let mut out: Self = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = value.elements[r][c] as f32;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Add<Self> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            <&Self as Add<&Self>>::add(&self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Add<&Self> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn add(self, rhs: &Self) -> Self::Output {
            <&Self as Add<&Self>>::add(&self, rhs)
        }
    }

    impl<const R: usize, const C: usize, T> Add<Self> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] + &rhs.elements[r][c];
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Add<MatrixInner<R, C, T>> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Add<T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn add(self, rhs: MatrixInner<R, C, T>) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] + rhs.elements[r][c];
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T> AddAssign<&Self> for MatrixInner<R, C, T>
    where
        for<'a> T: AddAssign<&'a T>,
    {
        fn add_assign(&mut self, rhs: &Self) {
            for r in 0..R {
                for c in 0..C {
                    self.elements[r][c] += &rhs.elements[r][c];
                }
            }
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> AddAssign<Self> for MatrixInner<R, C, T>
    where
        for<'a> T: AddAssign<&'a T>,
    {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            <Self as AddAssign<&Self>>::add_assign(self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Sub<Self> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            <&Self as Sub<&Self>>::sub(&self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Sub<&Self> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: &Self) -> Self::Output {
            <&Self as Sub<&Self>>::sub(&self, rhs)
        }
    }

    impl<const R: usize, const C: usize, T> Sub<Self> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] - &rhs.elements[r][c];
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Sub<MatrixInner<R, C, T>> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Sub<T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn sub(self, rhs: MatrixInner<R, C, T>) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] - rhs.elements[r][c];
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T> SubAssign<&Self> for MatrixInner<R, C, T>
    where
        for<'a> T: SubAssign<&'a T>,
    {
        fn sub_assign(&mut self, rhs: &Self) {
            for r in 0..R {
                for c in 0..C {
                    self.elements[r][c] -= &rhs.elements[r][c];
                }
            }
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> SubAssign<Self> for MatrixInner<R, C, T>
    where
        for<'a> T: SubAssign<&'a T>,
    {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            <Self as SubAssign<&Self>>::sub_assign(self, &rhs)
        }
    }

    impl<const R: usize, const M: usize, const C: usize, T: Copy + Clone> Mul<MatrixInner<M, C, T>> for MatrixInner<R, M, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        #[inline]
        fn mul(self, rhs: MatrixInner<M, C, T>) -> Self::Output {
            <&Self as Mul<&MatrixInner<M, C, T>>>::mul(&self, &rhs)
        }
    }

    impl<const R: usize, const M: usize, const C: usize, T: Copy + Clone> Mul<&MatrixInner<M, C, T>> for MatrixInner<R, M, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        #[inline]
        fn mul(self, rhs: &MatrixInner<M, C, T>) -> Self::Output {
            <&Self as Mul<&MatrixInner<M, C, T>>>::mul(&self, rhs)
        }
    }

    impl<const R: usize, const M: usize, const C: usize, T> Mul<&MatrixInner<M, C, T>> for &MatrixInner<R, M, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn mul(self, rhs: &MatrixInner<M, C, T>) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][0] * &rhs.elements[0][c];
                    for m in 1..M {
                        out.elements[r][c] += &self.elements[r][m] * &rhs.elements[m][c];
                    }
                }
            }

            out
        }
    }

    impl<const R: usize, const M: usize, const C: usize, T: Copy + Clone> Mul<MatrixInner<M, C, T>> for &MatrixInner<R, M, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn mul(self, rhs: MatrixInner<M, C, T>) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][0] * rhs.elements[0][c];
                    for m in 1..M {
                        out.elements[r][c] += &self.elements[r][m] * rhs.elements[m][c];
                    }
                }
            }

            out
        }
    }

    impl<const R: usize, T> MulAssign<&Self> for MatrixInner<R, R, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        fn mul_assign(&mut self, rhs: &Self) {
            for r in 0..R {
                let mut tmp: [T; R] = unsafe { MaybeUninit::uninit().assume_init() };
                for c in 0..R {
                    tmp[c] = &self.elements[r][0] * &rhs.elements[0][c];
                    for m in 1..R {
                        tmp[c] += &self.elements[r][m] * &rhs.elements[m][c];
                    }
                }
                for c in 0..R {
                    self.elements[r][c] = unsafe { read((&tmp as *const T).add(c)) };
                }
            }
        }
    }

    impl<const R: usize, T: Copy + Clone> MulAssign<Self> for MatrixInner<R, R, T>
    where
        T: AddAssign<T>,
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            <Self as MulAssign<&Self>>::mul_assign(self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Mul<T> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: T) -> Self::Output {
            <&Self as Mul<&T>>::mul(&self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Mul<&T> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: &T) -> Self::Output {
            <&Self as Mul<&T>>::mul(&self, rhs)
        }
    }

    impl<const R: usize, const C: usize, T> Mul<&T> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn mul(self, rhs: &T) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] * rhs;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Mul<T> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Mul<T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn mul(self, rhs: T) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] * rhs;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T> MulAssign<&T> for MatrixInner<R, C, T>
    where
        for<'a> T: MulAssign<&'a T>,
    {
        fn mul_assign(&mut self, rhs: &T) {
            for r in 0..R {
                for c in 0..C {
                    self.elements[r][c] *= rhs;
                }
            }
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> MulAssign<T> for MatrixInner<R, C, T>
    where
        for<'a> T: MulAssign<&'a T>,
    {
        #[inline]
        fn mul_assign(&mut self, rhs: T) {
            <Self as MulAssign<&T>>::mul_assign(self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Div<T> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Div<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn div(self, rhs: T) -> Self::Output {
            <&Self as Div<&T>>::div(&self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Div<&T> for MatrixInner<R, C, T>
    where
        for<'a> &'a T: Div<&'a T, Output = T>,
    {
        type Output = Self;

        #[inline]
        fn div(self, rhs: &T) -> Self::Output {
            <&Self as Div<&T>>::div(&self, rhs)
        }
    }

    impl<const R: usize, const C: usize, T> Div<&T> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Div<&'a T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn div(self, rhs: &T) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] / rhs;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Div<T> for &MatrixInner<R, C, T>
    where
        for<'a> &'a T: Div<T, Output = T>,
    {
        type Output = MatrixInner<R, C, T>;

        fn div(self, rhs: T) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[r][c] = &self.elements[r][c] / rhs;
                }
            }

            out
        }
    }

    impl<const R: usize, const C: usize, T> DivAssign<&T> for MatrixInner<R, C, T>
    where
        for<'a> T: DivAssign<&'a T>,
    {
        fn div_assign(&mut self, rhs: &T) {
            for r in 0..R {
                for c in 0..C {
                    self.elements[r][c] /= rhs;
                }
            }
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> DivAssign<T> for MatrixInner<R, C, T>
    where
        for<'a> T: DivAssign<&'a T>,
    {
        #[inline]
        fn div_assign(&mut self, rhs: T) {
            <Self as DivAssign<&T>>::div_assign(self, &rhs)
        }
    }

    impl<const R: usize, const C: usize, T: Copy + Clone> Transpose for MatrixInner<R, C, T> {
        type Output = MatrixInner<C, R, T>;

        fn transpose(self) -> Self::Output {
            let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..C {
                    out.elements[c][r] = self.elements[r][c];
                }
            }

            out
        }
    }

    impl<const R: usize, T: Copy + Clone> TransposeAssign for MatrixInner<R, R, T> {
        fn transpose_assign(&mut self) {
            for r in 0..R {
                for c in r..R {
                    (self.elements[r][c], self.elements[c][r]) = (self.elements[c][r], self.elements[r][c])
                }
            }
        }
    }

    impl<const R: usize, T: Zero + One> MatrixInner<R, R, T> {
        pub fn identity() -> Self {
            let mut out: Self = unsafe { MaybeUninit::uninit().assume_init() };

            for r in 0..R {
                for c in 0..R {
                    out.elements[r][c] = if r == c { T::one() } else { T::zero() };
                }
            }

            out
        }
    }
}

#[cfg(doc)]
#[doc(hidden)]
pub use inner::MatrixInner as _;

pub type Matrix<const R: usize, const C: usize, T> = inner::MatrixInner<R, C, T>;
pub type Matrix2<T> = Matrix<2, 2, T>;
pub type Matrix2i = Matrix2<i32>;
pub type Matrix2f = Matrix2<f32>;
pub type Matrix3<T> = Matrix<3, 3, T>;
pub type Matrix3i = Matrix3<i32>;
pub type Matrix3f = Matrix3<f32>;
pub type Matrix4<T> = Matrix<4, 4, T>;
pub type Matrix4i = Matrix4<i32>;
pub type Matrix4f = Matrix4<f32>;
pub type Vector<const C: usize, T> = Matrix<1, C, T>;
pub type Vector2<T> = Vector<2, T>;
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;
pub type Vector3<T> = Vector<3, T>;
pub type Vector3i = Vector3<i32>;
pub type Vector3f = Vector3<f32>;
pub type Vector4<T> = Vector<4, T>;
pub type Vector4i = Vector4<i32>;
pub type Vector4f = Vector4<f32>;

impl<const C: usize, T> From<[T; C]> for Vector<C, T> {
    fn from(value: [T; C]) -> Self {
        Self { elements: [value] }
    }
}

impl<const C: usize> Magnitude for &Vector<C, f32> {
    fn magnitude(self) -> f32 {
        let mut sum = 0.0;

        for c in 0..C {
            sum += self.elements[0][c] * self.elements[0][c];
        }

        sum.sqrt()
    }
}

impl<const C: usize> Normalize for &Vector<C, f32> {
    type Output = Vector<C, f32>;

    fn normalize(self) -> Self::Output {
        let m = self.magnitude();

        let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

        for c in 0..C {
            out.elements[0][c] = self.elements[0][c] / m;
        }

        out
    }
}

impl<const C: usize> NormalizeAssign for Vector<C, f32> {
    fn normalize_assign(&mut self) {
        let m = self.magnitude();
        for c in 0..C {
            self.elements[0][c] /= m;
        }
    }
}

impl<const C: usize, T: Copy + Clone + Default> Dot<Self> for Vector<C, T>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        <&Self as Dot<&Self>>::dot(&self, &rhs)
    }
}

impl<const C: usize, T: Copy + Clone + Default> Dot<&Self> for Vector<C, T>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: &Self) -> Self::Output {
        <&Self as Dot<&Self>>::dot(&self, rhs)
    }
}

impl<const C: usize, T: Default> Dot<Self> for &Vector<C, T>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut out = Self::Output::default();

        for c in 0..C {
            out += &self.elements[0][c] * &rhs.elements[0][c];
        }

        out
    }
}

impl<const C: usize, T: Copy + Clone + Default> Dot<Vector<C, T>> for &Vector<C, T>
where
    T: AddAssign<T>,
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = T;

    fn dot(self, rhs: Vector<C, T>) -> Self::Output {
        let mut out = Self::Output::default();

        for c in 0..C {
            out += &self.elements[0][c] * rhs.elements[0][c];
        }

        out
    }
}

macro_rules! impl_get_set_x {
    ($($t:ty),+$(,)?) => {
        $(
            impl<T: Copy> X<T> for $t {
                #[inline]
                fn x(&self) -> T {
                    self.elements[0][0]
                }
            }

            impl<T> XRef<T> for $t {
                #[inline]
                fn x_ref(&self) -> &T {
                    &self.elements[0][0]
                }
            }

            impl<T> XMut<T> for $t {
                #[inline]
                fn x_mut(&mut self) -> &mut T {
                    &mut self.elements[0][0]
                }
            }
        )*
    };
}
macro_rules! impl_get_set_y {
    ($($t:ty),+$(,)?) => {
        $(
            impl<T: Copy> Y<T> for $t {
                #[inline]
                fn y(&self) -> T {
                    self.elements[0][1]
                }
            }

            impl<T> YRef<T> for $t {
                #[inline]
                fn y_ref(&self) -> &T {
                    &self.elements[0][1]
                }
            }

            impl<T> YMut<T> for $t {
                #[inline]
                fn y_mut(&mut self) -> &mut T {
                    &mut self.elements[0][1]
                }
            }
        )*
    };
}
macro_rules! impl_get_set_z {
    ($($t:ty),+$(,)?) => {
        $(
            impl<T: Copy> Z<T> for $t {
                #[inline]
                fn z(&self) -> T {
                    self.elements[0][2]
                }
            }

            impl<T> ZRef<T> for $t {
                #[inline]
                fn z_ref(&self) -> &T {
                    &self.elements[0][2]
                }
            }

            impl<T> ZMut<T> for $t {
                #[inline]
                fn z_mut(&mut self) -> &mut T {
                    &mut self.elements[0][2]
                }
            }
        )*
    };
}
macro_rules! impl_get_set_w {
    ($($t:ty),+$(,)?) => {
        $(
            impl<T: Copy> W<T> for $t {
                #[inline]
                fn w(&self) -> T {
                    self.elements[0][3]
                }
            }

            impl<T> WRef<T> for $t {
                #[inline]
                fn w_ref(&self) -> &T {
                    &self.elements[0][3]
                }
            }

            impl<T> WMut<T> for $t {
                #[inline]
                fn w_mut(&mut self) -> &mut T {
                    &mut self.elements[0][3]
                }
            }
        )*
    };
}
impl_get_set_x!(Vector2<T>, Vector3<T>, Vector4<T>);
impl_get_set_y!(Vector2<T>, Vector3<T>, Vector4<T>);
impl_get_set_z!(Vector3<T>, Vector4<T>);
impl_get_set_w!(Vector4<T>);

impl<T: Copy + Clone> Cross<Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn cross(self, rhs: Self) -> Self::Output {
        <&Self as Cross<&Self>>::cross(&self, &rhs)
    }
}

impl<T: Copy + Clone> Cross<&Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn cross(self, rhs: &Self) -> Self::Output {
        <&Self as Cross<&Self>>::cross(&self, rhs)
    }
}

impl<T> Cross<Self> for &Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Vector3<T>;

    fn cross(self, rhs: Self) -> Self::Output {
        let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

        out.elements[0][0] = &self.elements[0][1] * &rhs.elements[0][2] - &self.elements[0][2] * &rhs.elements[0][1];
        out.elements[0][1] = &self.elements[0][2] * &rhs.elements[0][0] - &self.elements[0][0] * &rhs.elements[0][2];
        out.elements[0][2] = &self.elements[0][0] * &rhs.elements[0][1] - &self.elements[0][1] * &rhs.elements[0][0];

        out
    }
}

impl<T: Copy + Clone> Cross<Vector3<T>> for &Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = Vector3<T>;

    fn cross(self, rhs: Vector3<T>) -> Self::Output {
        let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

        out.elements[0][0] = &self.elements[0][1] * rhs.elements[0][2] - &self.elements[0][2] * rhs.elements[0][1];
        out.elements[0][1] = &self.elements[0][2] * rhs.elements[0][0] - &self.elements[0][0] * rhs.elements[0][2];
        out.elements[0][2] = &self.elements[0][0] * rhs.elements[0][1] - &self.elements[0][1] * rhs.elements[0][0];

        out
    }
}

impl<T> CrossAssign<&Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    fn cross_assign(&mut self, rhs: &Self) {
        unsafe { write(self as *mut Self, self.cross(rhs)) }
    }
}

impl<T: Copy + Clone> CrossAssign<Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    #[inline]
    fn cross_assign(&mut self, rhs: Self) {
        <Self as CrossAssign<&Self>>::cross_assign(self, &rhs);
    }
}

impl<T: Copy + Clone + Zero> Cross<Self> for Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn cross(self, rhs: Self) -> Self::Output {
        <&Self as Cross<&Self>>::cross(&self, &rhs)
    }
}

impl<T: Copy + Clone + Zero> Cross<&Self> for Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn cross(self, rhs: &Self) -> Self::Output {
        <&Self as Cross<&Self>>::cross(&self, rhs)
    }
}

impl<T: Zero> Cross<Self> for &Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Vector4<T>;

    fn cross(self, rhs: Self) -> Self::Output {
        let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

        out.elements[0][0] = &self.elements[0][1] * &rhs.elements[0][2] - &self.elements[0][2] * &rhs.elements[0][1];
        out.elements[0][1] = &self.elements[0][2] * &rhs.elements[0][0] - &self.elements[0][0] * &rhs.elements[0][2];
        out.elements[0][2] = &self.elements[0][0] * &rhs.elements[0][1] - &self.elements[0][1] * &rhs.elements[0][0];
        out.elements[0][3] = T::zero();

        out
    }
}

impl<T: Copy + Clone + Zero> Cross<Vector4<T>> for &Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = Vector4<T>;

    fn cross(self, rhs: Vector4<T>) -> Self::Output {
        let mut out: Self::Output = unsafe { MaybeUninit::uninit().assume_init() };

        out.elements[0][0] = &self.elements[0][1] * rhs.elements[0][2] - &self.elements[0][2] * rhs.elements[0][1];
        out.elements[0][1] = &self.elements[0][2] * rhs.elements[0][0] - &self.elements[0][0] * rhs.elements[0][2];
        out.elements[0][2] = &self.elements[0][0] * rhs.elements[0][1] - &self.elements[0][1] * rhs.elements[0][0];
        out.elements[0][3] = T::zero();

        out
    }
}

impl<T: Zero> CrossAssign<&Self> for Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    fn cross_assign(&mut self, rhs: &Self) {
        unsafe { write(self as *mut Self, self.cross(rhs)) }
    }
}

impl<T: Copy + Clone + Zero> CrossAssign<Self> for Vector4<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    #[inline]
    fn cross_assign(&mut self, rhs: Self) {
        <Self as CrossAssign<&Self>>::cross_assign(self, &rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m0 = Into::<Vector2i>::into([0, 1]);
        let m1 = m0.clone();
        assert_eq!(m0, m1);

        let m2 = m0.transpose();
        let m3 = Into::<_>::into([[0], [1]]);
        assert_eq!(m2, m3);

        let m4 = Matrix::identity();
        let m5 = Into::<_>::into([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(m4, m5);

        let mut m6 = Into::<Matrix2<_>>::into([[0.0, 1.0], [2.0, 3.0]]);
        let m7 = Into::<_>::into([[0.0, 2.0], [4.0, 6.0]]);
        assert_eq!(m6 + m6, m7);
        assert_eq!(m6 + &m6, m7);
        assert_eq!(&m6 + m6, m7);
        m6 += m6;
        assert_eq!(m6, m7);

        let mut m8 = Into::<Matrix2<_>>::into([[0.0, 1.0], [2.0, 3.0]]);
        let m9 = Default::default();
        assert_eq!(m8 - m8, m9);
        assert_eq!(m8 - &m8, m9);
        assert_eq!(&m8 - m8, m9);
        m8 -= m8;
        assert_eq!(m8, m9);
    }
}
