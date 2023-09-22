/*use druid::kurbo::{BezPath, Line, Point};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, Data, Env, Lens, Widget, WidgetExt, WindowDesc};

const HEX_RADIUS: f64 = 40.0;
const HEX_WIDTH: f64 = HEX_RADIUS * 2.0;
const HEX_HEIGHT: f64 = (3.0_f64).sqrt() * HEX_RADIUS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HexCell {
    player: usize,
}

impl HexCell {
    fn color(&self) -> Color {
        match self.player {
            0 => Color::TRANSPARENT,
            1 => Color::WHITE,
            2 => Color::BLACK,
            _ => Color::TRANSPARENT,
        }
    }
}

impl Data for HexCell {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Data, Lens)]
struct AppState {
    // Define your hex grid state here
    grid: Vec<Vec<HexCell>>,
}

impl AppState {
    fn new(rows: usize, cols: usize) -> Self {
        let mut grid = vec![vec![HexCell::Empty; cols]; rows];
        // Initialize your grid as needed
        AppState { grid }
    }
}

struct HexGrid {
    rows: usize,
    cols: usize,
}

impl HexGrid {
    fn new(rows: usize, cols: usize) -> Self {
        HexGrid { rows, cols }
    }
}

impl Widget<AppState> for HexGrid {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &AppState,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        Size::new(
            HEX_WIDTH * (self.cols as f64 + 0.5),
            HEX_HEIGHT * (self.rows as f64 + 0.5),
        )
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let x = col as f64 * HEX_WIDTH * 1.5 + HEX_WIDTH / 2.0;
                let y = row as f64 * HEX_HEIGHT + HEX_HEIGHT / 2.0;

                let hex_points = [
                    Point::new(x, y - HEX_RADIUS),
                    Point::new(x + HEX_WIDTH / 2.0, y - HEX_HEIGHT / 2.0),
                    Point::new(x + HEX_WIDTH / 2.0, y + HEX_HEIGHT / 2.0),
                    Point::new(x, y + HEX_RADIUS),
                    Point::new(x - HEX_WIDTH / 2.0, y + HEX_HEIGHT / 2.0),
                    Point::new(x - HEX_WIDTH / 2.0, y - HEX_HEIGHT / 2.0),
                ];

                let hex_path = BezPath::from_vec(hex_points.to_vec());
                ctx.fill(hex_path, &data.grid[row][col].color());
                ctx.stroke(hex_path, &Color::BLACK, 1.0);
            }
        }
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Hex Grid")
        .window_size((800.0, 600.0));

    let initial_state = AppState::new(5, 5); // Define the number of rows and columns

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    HexGrid::new(5, 5) // Define the number of rows and columns for the hex grid
        .center()
        .padding(20.0)
}
*/