use gpui::{
    App, Context, Entity, Focusable as _, IntoElement, Render, Styled, TestAppContext,
    VisualTestContext, Window, div, prelude::*, px,
};
use gpui_component::table::{Column, DataTable, TableDelegate, TableState};

const ROWS: usize = 20;
const COLUMNS: usize = 12;

struct WideDelegate;

impl TableDelegate for WideDelegate {
    fn columns_count(&self, _: &App) -> usize {
        COLUMNS
    }

    fn rows_count(&self, _: &App) -> usize {
        ROWS
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        let name = format!("column-{col_ix}");
        Column::new(name.clone(), name).width(px(120.))
    }

    fn render_td(
        &mut self,
        _: usize,
        _: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        div()
    }
}

struct Harness {
    table: Entity<TableState<WideDelegate>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(400.))
            .h(px(300.))
            .child(DataTable::new(&self.table))
    }
}

fn harness(
    cx: &mut TestAppContext,
    col_selectable: bool,
) -> (Entity<TableState<WideDelegate>>, &mut VisualTestContext) {
    cx.update(gpui_component::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        table: cx.new(|cx| {
            TableState::new(WideDelegate, window, cx)
                .row_selectable(true)
                .col_selectable(col_selectable)
        }),
    });
    let table = view.read_with(cx, |harness, _| harness.table.clone());
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        table.focus_handle(cx).focus(window, cx);
    });
    (table, cx)
}

#[gpui::test]
fn home_and_end_select_the_first_and_last_row(cx: &mut TestAppContext) {
    let (table, cx) = harness(cx, false);
    table.update(cx, |table, cx| table.set_selected_row(5, cx));

    cx.simulate_keystrokes("end");
    assert_eq!(
        table.read_with(cx, |table, _| table.selected_row()),
        Some(ROWS - 1)
    );

    cx.simulate_keystrokes("home");
    assert_eq!(
        table.read_with(cx, |table, _| table.selected_row()),
        Some(0)
    );
}

#[gpui::test]
fn home_and_end_still_move_a_selected_column(cx: &mut TestAppContext) {
    let (table, cx) = harness(cx, true);
    table.update(cx, |table, cx| table.set_selected_col(5, cx));

    cx.simulate_keystrokes("end");
    assert_eq!(
        table.read_with(cx, |table, _| table.selected_col()),
        Some(COLUMNS - 1)
    );

    cx.simulate_keystrokes("home");
    assert_eq!(
        table.read_with(cx, |table, _| table.selected_col()),
        Some(0)
    );
}
