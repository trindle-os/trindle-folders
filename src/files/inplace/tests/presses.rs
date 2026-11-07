// A press anywhere in a pane makes it the one in use.

use super::*;

#[test]
fn a_press_anywhere_in_a_pane_makes_it_the_one_in_use() {
    let (dir, mut b) = bench("presses", View::List);
    for p in [0, 1] {
        presses_make_the_pane_in_use(&mut b, p);
        back_goes_back_in_the_pane_in_use(&mut b, &dir, p);
    }
    b.f.update(Msg::Activate(0));
    b.show();
    b.send(Input::Press { button: Button::Left, x: 600.0, y: 6.0 });
    assert_eq!(b.f.side(), 0);
    fs::remove_dir_all(&dir).unwrap();
}

fn presses_make_the_pane_in_use(b: &mut Bench, p: usize) {
    let other = 1 - p;
    let cell = b.cell(p, "report.pdf");
    let items = b.items_at(p);
    let pane_w = b.f.cols[p].width.get();
    let presses: Vec<(&str, (f32, f32))> = vec![
        ("the line saying where it is", (items.x + 6.0, items.y - 34.0)),
        ("a heading", (items.x + 6.0, items.y - 10.0)),
        ("an item", (cell.x + 6.0, cell.y + cell.h / 2.0)),
        ("empty space", (items.x + 6.0, items.y + 380.0)),
        ("the edge, where the scroll bar is", (items.x + pane_w - 2.0, items.y + 40.0)),
    ];
    for (what, (x, y)) in presses {
        for kind in ["left", "right", "touch"] {
            b.f.update(Msg::Activate(other));
            b.show();
            assert_eq!(b.f.side(), other, "{what}, {kind}: the other pane is in use to start with");
            b.send(Input::PointerMove { x, y });
            match kind {
                "left" => b.send(Input::Press { button: Button::Left, x, y }),
                "right" => b.send(Input::Press { button: Button::Right, x, y }),
                _ => b.send(Input::TouchDown { id: 1, x, y }),
            }
            assert_eq!(b.f.side(), p, "pane {p}: a {kind} press on {what} makes it the one in use");
            match kind {
                "left" => b.send(Input::Release { button: Button::Left, x, y }),
                "right" => b.send(Input::Release { button: Button::Right, x, y }),
                _ => b.send(Input::TouchUp { id: 1 }),
            }
            b.f.menu = None;
            b.f.locate = None;
            b.key(Keysym::Escape);
            b.f.update(Msg::Reload);
            b.show();
        }
    }
}

fn back_goes_back_in_the_pane_in_use(b: &mut Bench, dir: &Path, p: usize) {
    let other = 1 - p;
    b.f.update(Msg::Activate(p));
    let (was, other_was) = (b.f.cols[p].at.clone(), b.f.cols[other].at.clone());
    b.f.go(p, Where::Folder(side_dir(dir, p).join("Albums")));
    b.show();
    let pills = b.ui.pill_boxes(&format!("path{p}"));
    let (row_y, left) = (pills[0].y + pills[0].h / 2.0, pills[0].x);
    let back = (0..400).map(|i| left - i as f32).find(|&x| b.ui.label_over(x, row_y).is_some_and(|l| l.starts_with("Back"))).expect("Back");
    b.send(Input::PointerMove { x: back, y: row_y });
    b.send(Input::Press { button: Button::Left, x: back, y: row_y });
    b.send(Input::Release { button: Button::Left, x: back, y: row_y });
    assert_eq!((b.f.side(), &b.f.cols[p].at, &b.f.cols[other].at), (p, &was, &other_was), "the top bar's Back goes back in the pane in use only");
    b.f.update(Msg::Activate(other));
    b.f.go(p, Where::Folder(side_dir(dir, p)));
    b.show();
    let cell = b.cell(p, "report.pdf");
    b.press_at(cell.x + 6.0, cell.y + cell.h / 2.0);
    assert_eq!(b.f.side(), p);
    assert_eq!(chosen(&b.f, p).as_deref(), Some("report.pdf"), "the item press still chooses it");
}
