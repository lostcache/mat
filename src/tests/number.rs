use crate::number::Number;

fn assert_number<T: Number>() {}

#[test]
fn f32_is_number()
{
    assert_number::<f32>();
}

#[test]
fn f64_is_number()
{
    assert_number::<f64>();
}

#[test]
fn i8_is_number()
{
    assert_number::<i8>();
}

#[test]
fn i16_is_number()
{
    assert_number::<i16>();
}

#[test]
fn i32_is_number()
{
    assert_number::<i32>();
}

#[test]
fn i64_is_number()
{
    assert_number::<i64>();
}

#[test]
fn i128_is_number()
{
    assert_number::<i128>();
}

#[test]
fn u8_is_number()
{
    assert_number::<u8>();
}

#[test]
fn u16_is_number()
{
    assert_number::<u16>();
}

#[test]
fn u32_is_number()
{
    assert_number::<u32>();
}

#[test]
fn u64_is_number()
{
    assert_number::<u64>();
}

#[test]
fn u128_is_number()
{
    assert_number::<u128>();
}
