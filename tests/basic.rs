use pod_enum::pod_enum;

#[pod_enum]
/// An enum for testing purposes
///
/// Variants exist for 0, 1, and 2, and the rest are unknown.
#[repr(u8)]
enum Foo {
    /// Bar
    Bar = 0,
    /// Bat
    Bat = 1,
    /// Baz
    Baz = 2,
}

/// Test the `Debug` impl
#[test]
fn test_debug() {
    /// Assert that the given variant has the given format
    #[track_caller]
    fn assert_format(foo: Foo, format: &str) {
        assert_eq!(&format!("{foo:?}"), format);
    }
    assert_format(Foo::Bar, "Bar");
    assert_format(Foo::Bat, "Bat");
    assert_format(Foo::Baz, "Baz");
    assert_format(Foo::from(3), "Unknown (3)");
    assert_format(Foo::from(255), "Unknown (255)");
}

/// Test round-trip conversions between the enum and the repr
#[test]
fn test_round_trip() {
    /// Assert that the given `u8` round-trip converts back to itself
    #[track_caller]
    fn assert_round_trip(start: u8) {
        assert_eq!(start, u8::from(Foo::from(start)));
    }
    assert_round_trip(0);
    assert_round_trip(1);
    assert_round_trip(2);
    assert_round_trip(3);
    assert_round_trip(255);
}

/// Test the `PartialEq` impl
#[test]
fn test_equality() {
    // Assert that variants compare with each other correctly
    assert_eq!(Foo::Bar, Foo::Bar);
    assert_ne!(Foo::Bar, Foo::Bat);
    assert_ne!(Foo::Bar, Foo::Baz);

    assert_ne!(Foo::Bat, Foo::Bar);
    assert_eq!(Foo::Bat, Foo::Bat);
    assert_ne!(Foo::Bat, Foo::Baz);

    assert_ne!(Foo::Baz, Foo::Bar);
    assert_ne!(Foo::Baz, Foo::Bat);
    assert_eq!(Foo::Baz, Foo::Baz);

    // Assert that unknowns compare as unequal to everything
    let variants = [Foo::Bar, Foo::Bat, Foo::Baz];
    let unknowns = [Foo::from(3), Foo::from(8), Foo::from(32), Foo::from(255)];
    for (a, b) in variants
        .into_iter()
        .chain(unknowns.into_iter())
        .flat_map(|v| unknowns.into_iter().map(move |u| (v, u)))
    {
        assert_ne!(a, b);
    }
}
