use std::{cell::RefCell, rc::Rc};

use gpui::{
    App, Context, Entity, Focusable as _, IntoElement, Modifiers, MouseButton, Render, Styled,
    TestAppContext, VisualTestContext, Window, div, point, prelude::*, px,
};
use gpui_component::table::{Column, DataTable, TableDelegate, TableEvent, TableState};

/// Cells in column 1 select their row on mouse down.
struct RowsDelegate;

impl TableDelegate for RowsDelegate {
    fn columns_count(&self, _: &App) -> usize {
        2
    }

    fn rows_count(&self, _: &App) -> usize {
        3
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        let name = format!("column-{col_ix}");
        Column::new(name.clone(), name)
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        div()
            .id(("cell", row_ix * 2 + col_ix))
            .size_full()
            .debug_selector(move || format!("cell-{row_ix}-{col_ix}"))
            .when(col_ix == 1, |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |table, _, _, cx| table.set_selected_row(row_ix, cx)),
                )
            })
    }
}

struct Harness {
    table: Entity<TableState<RowsDelegate>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug_selector(|| "host".into())
            .w(px(400.))
            .h(px(300.))
            .child(DataTable::new(&self.table))
    }
}

fn harness(cx: &mut TestAppContext) -> (Entity<TableState<RowsDelegate>>, &mut VisualTestContext) {
    cx.update(gpui_component::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        table: cx.new(|cx| TableState::new(RowsDelegate, window, cx).cell_selectable(true)),
    });
    let table = view.read_with(cx, |harness, _| harness.table.clone());
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.draw(cx).clear(cx);
    });
    (table, cx)
}

fn record_selected_rows(
    table: &Entity<TableState<RowsDelegate>>,
    cx: &mut VisualTestContext,
) -> Rc<RefCell<Vec<usize>>> {
    let rows = Rc::new(RefCell::new(Vec::new()));
    cx.update({
        let rows = rows.clone();
        |_, cx| {
            cx.subscribe(table, move |_, event: &TableEvent, _| {
                if let TableEvent::SelectRow(row_ix) = event {
                    rows.borrow_mut().push(*row_ix);
                }
            })
            .detach()
        }
    });
    rows
}

#[gpui::test]
fn set_selected_row_in_cell_mouse_down_keeps_table_focus(cx: &mut TestAppContext) {
    let (table, cx) = harness(cx);
    let cell = cx.debug_bounds("cell-1-1").unwrap();

    cx.simulate_mouse_down(cell.center(), MouseButton::Left, Modifiers::default());

    assert_eq!(
        table.read_with(cx, |table, _| table.selected_row()),
        Some(1)
    );
    let focused = cx.update(|window, cx| table.focus_handle(cx).is_focused(window));
    assert!(focused);
}

#[gpui::test]
fn row_header_click_selects_row_once(cx: &mut TestAppContext) {
    let (table, cx) = harness(cx);
    let rows = record_selected_rows(&table, cx);
    let host = cx.debug_bounds("host").unwrap();
    let cell = cx.debug_bounds("cell-2-0").unwrap();

    // The row header is the narrow column on the left.
    cx.simulate_click(
        point(host.left() + px(6.), cell.center().y),
        Modifiers::default(),
    );

    assert_eq!(*rows.borrow(), [2]);
    assert_eq!(
        table.read_with(cx, |table, _| table.selected_row()),
        Some(2)
    );
}
