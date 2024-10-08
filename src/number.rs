pub trait Number: Default + Clone {}

impl Number for f32 {}

impl Number for f64 {}

impl Number for i8 {}

impl Number for i16 {}

impl Number for i32 {}

impl Number for i64 {}

impl Number for i128 {}

impl Number for u8 {}

impl Number for u16 {}

impl Number for u32 {}

impl Number for u64 {}

impl Number for u128 {}

#[cfg(test)]
mod tests
{
    use super::*;

    fn assert_number<T: Number>() {}

    #[test]
    fn test_f32_is_number()
    {
        assert_number::<f32>();
    }

    #[test]
    fn test_f64_is_number()
    {
        assert_number::<f64>();
    }

    #[test]
    fn test_i8_is_number()
    {
        assert_number::<i8>();
    }

    #[test]
    fn test_i16_is_number()
    {
        assert_number::<i16>();
    }

    #[test]
    fn test_i32_is_number()
    {
        assert_number::<i32>();
    }

    #[test]
    fn test_i64_is_number()
    {
        assert_number::<i64>();
    }

    #[test]
    fn test_i128_is_number()
    {
        assert_number::<i128>();
    }

    #[test]
    fn test_u8_is_number()
    {
        assert_number::<u8>();
    }

    #[test]
    fn test_u16_is_number()
    {
        assert_number::<u16>();
    }

    #[test]
    fn test_u32_is_number()
    {
        assert_number::<u32>();
    }

    #[test]
    fn test_u64_is_number()
    {
        assert_number::<u64>();
    }

    #[test]
    fn test_u128_is_number()
    {
        assert_number::<u128>();
    }
}
