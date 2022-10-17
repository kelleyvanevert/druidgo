// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

#[macro_use]
extern crate enum_map;

mod game;

use crate::game::{Game, Pos};
use druid::kurbo::{Circle, Line};
use druid::piet::{LineCap, LineJoin, StrokeStyle};
use druid::widget::{CrossAxisAlignment, Flex, MainAxisAlignment};
use druid::{
    AppLauncher, Color, Data, Event, Lens, MouseButton, PlatformError, Point, RenderContext,
    Widget, WidgetExt, WindowDesc,
};
use game::Stone;

#[derive(Clone, Data, Lens)]
struct ViewModel {
    padding: f64,
    game: Game,
    hover: Option<Pos>,
}

impl ViewModel {
    fn project(&self, widget_size: f64, pos: Pos) -> Point {
        let board_size = widget_size - 2.0 * self.padding;
        let stone_size = board_size / (self.game.size as f64);
        Point {
            x: self.padding + (pos.0 as f64 + 0.5) * stone_size,
            y: self.padding + (pos.1 as f64 + 0.5) * stone_size,
        }
    }

    #[allow(dead_code)]
    fn valid_project(&self, widget_size: f64, pos: Pos) -> Option<Point> {
        pos.and_valid(self.game.size)
            .map(|p| self.project(widget_size, p))
    }

    fn unproject(&self, widget_size: f64, pt: Point) -> Pos {
        let board_size = widget_size - 2.0 * self.padding;
        Pos(
            ((pt.x - self.padding) / board_size * (self.game.size as f64)).floor() as i32,
            ((pt.y - self.padding) / board_size * (self.game.size as f64)).floor() as i32,
        )
    }

    fn unproject_valid(&self, widget_size: f64, pt: Point) -> Option<Pos> {
        self.unproject(widget_size, pt).and_valid(self.game.size)
    }
}

struct GoBoardWidget {}

impl GoBoardWidget {
    fn new() -> Self {
        Self {}
    }
}

impl Widget<ViewModel> for GoBoardWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        model: &mut ViewModel,
        _env: &druid::Env,
    ) {
        match event {
            Event::MouseMove(e) => {
                model.hover = model.unproject_valid(ctx.size().width, e.pos);
                ctx.request_paint();
            }
            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    if let Some(pos) = model.unproject_valid(ctx.size().width, e.pos) {
                        model.game.try_place_stone(pos);
                        ctx.request_paint();
                    }
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _model: &ViewModel,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_model: &ViewModel,
        _model: &ViewModel,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _model: &ViewModel,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.constrain_aspect_ratio(1., f64::INFINITY)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, model: &ViewModel, _env: &druid::Env) {
        let ViewModel { game, .. } = model;
        let widget_size = ctx.size().width;
        let board_size = widget_size - 2.0 * model.padding;
        let stone_size = board_size / (game.size as f64);
        let line_stroke_style = StrokeStyle::new()
            .line_cap(LineCap::Round)
            .line_join(LineJoin::Round);

        for x in 0..game.size {
            for y in 0..game.size {
                ctx.stroke_styled(
                    Line::new(
                        model.project(widget_size, (x, 0).into()),
                        model.project(widget_size, (x, game.size - 1).into()),
                    ),
                    &Color::BLACK,
                    board_size / 500.0,
                    &line_stroke_style,
                );
                ctx.stroke_styled(
                    Line::new(
                        model.project(widget_size, (0, y).into()),
                        model.project(widget_size, (game.size - 1, y).into()),
                    ),
                    &Color::BLACK,
                    board_size / 500.0,
                    &line_stroke_style,
                );
            }
        }

        let stone_stroke_width = board_size / 250.0;
        for x in 0..game.size {
            for y in 0..game.size {
                match game.stone_at(Pos(x as i32, y as i32)) {
                    Some(color) => {
                        let shape = Circle::new(
                            model.project(widget_size, (x, y).into()),
                            stone_size / 2.0 - stone_stroke_width / 3.0,
                        );
                        ctx.fill(
                            shape,
                            match color {
                                Stone::Black => &Color::BLACK,
                                Stone::White => &Color::WHITE,
                            },
                        );
                        ctx.stroke_styled(
                            shape,
                            &Color::BLACK,
                            stone_stroke_width,
                            &line_stroke_style,
                        );
                    }
                    None => {}
                }
            }
        }

        if let Some(p) = model.hover {
            let scale = 1.15;
            if !game.has_stone_at(p) {
                let shape = Circle::new(
                    model.project(widget_size, p),
                    stone_size / 2.0 * scale - stone_stroke_width / 3.0,
                );
                ctx.fill(
                    shape,
                    match model.game.turn {
                        Stone::Black => &Color::BLACK,
                        Stone::White => &Color::WHITE,
                    },
                );
                ctx.stroke_styled(
                    shape,
                    &Color::BLACK,
                    stone_stroke_width * scale,
                    &line_stroke_style,
                );
            }
        }
    }
}

fn build_calc() -> impl Widget<ViewModel> {
    let board = GoBoardWidget::new();

    Flex::column()
        .with_flex_child(board, 1.)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .background(Color::WHITE)
}

pub fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_calc())
        .window_size((800., 600.))
        .resizable(true)
        .title("Go");

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(ViewModel {
            padding: 24.0,
            game: Game::new(13),
            hover: None,
        })
}
