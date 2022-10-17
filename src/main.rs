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
    AppLauncher, Color, Data, Event, Lens, LifeCycle, MouseButton, PlatformError, Rect,
    RenderContext, Size, Widget, WidgetExt, WindowDesc,
};
use game::Stone;

#[derive(Clone, Data, Lens)]
struct ViewModel {
    game: Game,
    hover: Option<Pos>,
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
                let Size { width, height } = ctx.size();
                let pos = Pos(
                    (e.pos.x / width * (model.game.size as f64)).floor() as i32,
                    (e.pos.y / height * (model.game.size as f64)).floor() as i32,
                );
                model.hover = pos.and_valid(model.game.size);
                ctx.request_paint();
            }
            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let Size { width, height } = ctx.size();
                    let x = (e.pos.x / width * (model.game.size as f64)).floor() as i32;
                    let y = (e.pos.y / height * (model.game.size as f64)).floor() as i32;
                    model.game.try_place_stone(Pos(x, y));
                    ctx.request_paint();
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
        let board_size = ctx.size().width;
        let stone_size = board_size / (game.size as f64);
        let line_stroke_style = StrokeStyle::new()
            .line_cap(LineCap::Round)
            .line_join(LineJoin::Round);

        // ctx.fill(
        //     Rect::new(0.0, 0.0, board_size, board_size),
        //     &Color::rgb(0.9, 0.9, 0.9),
        // );

        for x in 0..game.size {
            for y in 0..game.size {
                ctx.stroke_styled(
                    Line::new(
                        ((x as f64 + 0.5) * stone_size, 0.5 * stone_size),
                        (
                            (x as f64 + 0.5) * stone_size,
                            (game.size as f64 - 0.5) * stone_size,
                        ),
                    ),
                    &Color::BLACK,
                    board_size / 500.0,
                    &line_stroke_style,
                );
                ctx.stroke_styled(
                    Line::new(
                        (0.5 * stone_size, (y as f64 + 0.5) * stone_size),
                        (
                            (game.size as f64 - 0.5) * stone_size,
                            (y as f64 + 0.5) * stone_size,
                        ),
                    ),
                    &Color::BLACK,
                    board_size / 500.0,
                    &line_stroke_style,
                );
            }
        }

        for x in 0..game.size {
            for y in 0..game.size {
                match game.stone_at(Pos(x as i32, y as i32)) {
                    Some(color) => {
                        let shape = Circle::new(
                            ((x as f64 + 0.5) * stone_size, (y as f64 + 0.5) * stone_size),
                            stone_size / 2.0,
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
                            board_size / 250.0,
                            &line_stroke_style,
                        );
                    }
                    None => {}
                }
            }
        }

        if let Some(p) = model.hover {
            if !game.has_stone_at(p) {
                let shape = Circle::new(
                    (
                        (p.0 as f64 + 0.5) * stone_size,
                        (p.1 as f64 + 0.5) * stone_size,
                    ),
                    stone_size / 2.0 * 1.1,
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
                    board_size / 250.0 * 1.1,
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
        .padding(24.0)
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
            game: Game::new(13),
            hover: None,
        })
}
