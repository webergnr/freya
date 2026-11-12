use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

// Each scroll view takes the left half of the 500x500 window; the right half is outside it.
const INSIDE: (f64, f64) = (100., 100.);
const OUTSIDE: (f64, f64) = (400., 100.);
const THUMB: (f64, f64) = (245., 20.);

fn scroll_view_app() -> impl IntoElement {
    rect().horizontal().child(
        ScrollView::new()
            .width(Size::px(250.))
            .children((0..4).map(|i| rect().key(i).height(Size::px(200.)).width(Size::fill()))),
    )
}

fn virtual_scroll_view_app() -> impl IntoElement {
    rect().horizontal().child(
        VirtualScrollView::new(|item, _| rect().key(item.index).height(Size::px(50.)).into())
            .length(30usize)
            .item_size(50.)
            .width(Size::px(250.)),
    )
}

fn for_each_scroll_view(check: impl Fn(&mut TestingRunner)) {
    let apps: [fn() -> Element; 2] = [
        || scroll_view_app().into_element(),
        || virtual_scroll_view_app().into_element(),
    ];
    for app in apps {
        let mut test = launch_test(app);
        test.sync_and_update();
        check(&mut test);
    }
}

fn scrollbar_shown(test: &TestingRunner) -> bool {
    let scrollbars = test.find_many(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.relative_layer == 999.into())
            .map(|_| node)
    });
    assert_eq!(scrollbars.len(), 1);
    scrollbars[0].layout().area.width() > 0.
}

#[test]
fn scrollbar_hidden_on_mount_with_the_pointer_outside() {
    for_each_scroll_view(|test| {
        assert!(!scrollbar_shown(test), "shown on mount");
        test.move_cursor(OUTSIDE);
        assert!(!scrollbar_shown(test), "shown with the pointer outside");
    });
}

#[test]
fn scrollbar_shown_while_hovered_hidden_once_the_pointer_leaves() {
    for_each_scroll_view(|test| {
        test.move_cursor(INSIDE);
        assert!(scrollbar_shown(test), "hidden after the pointer moved in");
        test.move_cursor(OUTSIDE);
        assert!(!scrollbar_shown(test), "still shown after the pointer left");
    });
}

#[test]
fn scrollbar_shown_on_a_move_after_a_press_without_one() {
    // A press marks the view hovered without an over event, so the move after it must show the bar.
    for_each_scroll_view(|test| {
        test.press_cursor(INSIDE);
        test.release_cursor(INSIDE);
        test.move_cursor((INSIDE.0, INSIDE.1 + 10.));
        assert!(scrollbar_shown(test), "hidden after a press then a move");
    });
}

#[test]
fn scrollbar_hidden_once_the_pointer_leaves_the_window() {
    for_each_scroll_view(|test| {
        test.move_cursor(INSIDE);
        assert!(scrollbar_shown(test));
        test.send_event(PlatformEvent::PointerExit {
            cursor: INSIDE.into(),
        });
        test.sync_and_update();
        assert!(
            !scrollbar_shown(test),
            "still shown after the pointer left the window"
        );
    });
}

#[test]
fn scrollbar_hidden_while_hovered_but_idle() {
    for_each_scroll_view(|test| {
        test.move_cursor(INSIDE);
        test.poll(Duration::from_millis(20), Duration::from_secs(2));
        assert!(!scrollbar_shown(test), "shown while hovering idle");
        test.move_cursor((INSIDE.0, INSIDE.1 + 10.));
        assert!(scrollbar_shown(test), "hidden after moving again");
    });
}

#[test]
fn scrollbar_kept_while_dragging_the_thumb_outside() {
    for_each_scroll_view(|test| {
        test.move_cursor(THUMB);
        test.press_cursor(THUMB);
        test.move_cursor((400., 300.));
        assert!(scrollbar_shown(test), "hidden while dragging the thumb");
        test.release_cursor((400., 300.));
        assert!(
            !scrollbar_shown(test),
            "still shown after releasing the thumb outside"
        );
    });
}
