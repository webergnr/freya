use freya_core::{
    integration::*,
    prelude::*,
};
use freya_testing::TestingRunner;
use torin::size::Size;

#[test]
fn a_pinch_reaches_on_pinch_and_not_on_wheel() {
    fn app() -> impl IntoElement {
        let mut pinched = use_consume::<State<Vec<f64>>>();
        let mut wheeled = use_consume::<State<i32>>();

        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .on_pinch(move |e: Event<WheelEventData>| pinched.write().push(e.delta_y))
            .on_wheel(move |_| *wheeled.write() += 1)
    }

    let (mut test, (pinched, wheeled)) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(Vec::<f64>::new())),
                runner.provide_root_context(|| State::create(0i32)),
            )
        },
        1.,
    );
    test.sync_and_update();

    test.send_event(PlatformEvent::Wheel {
        name: WheelEventName::Pinch,
        scroll: (0., 0.25).into(),
        cursor: (50., 50.).into(),
        source: WheelSource::Pixel,
    });
    test.sync_and_update();
    assert_eq!(*pinched.peek(), vec![0.25]);
    assert_eq!(*wheeled.peek(), 0, "a pinch is not a wheel turn");

    // outside the element: nothing
    test.send_event(PlatformEvent::Wheel {
        name: WheelEventName::Pinch,
        scroll: (0., 0.5).into(),
        cursor: (150., 150.).into(),
        source: WheelSource::Pixel,
    });
    test.sync_and_update();
    assert_eq!(*pinched.peek(), vec![0.25]);

    // and a wheel turn is not a pinch
    test.scroll((50., 50.), (0., 10.));
    assert_eq!(*pinched.peek(), vec![0.25]);
    assert_eq!(*wheeled.peek(), 1);
}
