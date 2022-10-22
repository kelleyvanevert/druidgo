use std::ops::Sub;

use druid::{
    widget::Axis, BoxConstraints, Color, Data, Insets, Point, Rect, RenderContext, Size, Widget,
    WidgetPod,
};

pub trait SumBy<T> {
    fn sum_by(&self, get: fn(&T) -> f64) -> f64;
}

impl<T> SumBy<T> for Vec<T> {
    fn sum_by(&self, get: fn(&T) -> f64) -> f64 {
        self.iter().map(|el| get(el)).sum()
    }
}

pub trait Transpose {
    fn transpose(self) -> Self;

    fn transpose_if(self, condition: bool) -> Self
    where
        Self: Sized,
    {
        if condition {
            self.transpose()
        } else {
            self
        }
    }
}

impl Transpose for (f64, f64) {
    fn transpose(self) -> Self {
        (self.1, self.0)
    }
}

impl Transpose for Size {
    fn transpose(self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}

impl Transpose for Point {
    fn transpose(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl Transpose for Rect {
    fn transpose(self) -> Self {
        Self {
            x0: self.y0,
            y0: self.x0,
            x1: self.y1,
            y1: self.x1,
        }
    }
}

impl Transpose for Insets {
    fn transpose(self) -> Self {
        Self {
            x0: self.y0,
            y0: self.x0,
            x1: self.y1,
            y1: self.x1,
        }
    }
}

#[allow(dead_code)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[allow(dead_code)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    // Baseline,
}

#[allow(dead_code)]
pub enum AlignContent {
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub struct FlexBox<'a, T> {
    pub debug_label: String,

    // flexbox container styles
    pub direction: Axis,
    pub reverse: bool,
    pub wrap: bool,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap: f64,
    pub row_gap: f64,
    pub column_gap: f64,

    // flexbox child styles
    pub grow: f64,
    pub shrink: f64,
    pub basis: Option<f64>, // None = "auto"

    // regular styles
    pub background: Option<&'a Color>,
    pub border: Insets,
    pub border_color: &'a Color,
    pub padding: Insets,

    // flex children
    pub children: Vec<WidgetPod<T, FlexBox<'a, T>>>,
    pub content: Option<WidgetPod<T, Box<dyn Widget<T>>>>,
}

#[allow(dead_code)]
impl<'a, T: Data> FlexBox<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacer(grow: Option<f64>) -> Self {
        Self::new().grow(grow.unwrap_or(1.0))
    }

    pub fn direction(mut self, axis: Axis) -> Self {
        self.direction = axis;
        self
    }

    pub fn basis(mut self, basis: f64) -> Self {
        self.basis = Some(basis);
        self
    }

    pub fn debug_label(mut self, debug_label: &str) -> Self {
        self.debug_label = debug_label.into();
        self
    }

    pub fn grow(mut self, grow: f64) -> Self {
        self.grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: f64) -> Self {
        self.shrink = shrink;
        self
    }

    pub fn padding(mut self, p: impl Into<Insets>) -> Self {
        self.padding = p.into();
        self
    }

    pub fn background(mut self, color: &'a Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn no_background(mut self) -> Self {
        self.background = None;
        self
    }

    pub fn border(mut self, b: impl Into<Insets>) -> Self {
        self.border = b.into();
        self
    }

    pub fn add_child(&mut self, child: FlexBox<'a, T>) {
        self.children.push(WidgetPod::new(child));
    }

    pub fn with_child(mut self, child: FlexBox<'a, T>) -> Self {
        self.add_child(child);
        self
    }

    pub fn content(mut self, content: impl Widget<T> + 'static) -> Self {
        self.content = Some(WidgetPod::new(Box::new(content)));
        self
    }
}

impl<'a, T: Data> Default for FlexBox<'a, T> {
    fn default() -> Self {
        FlexBox {
            debug_label: "unknown".into(),

            direction: Axis::Horizontal,
            reverse: false,
            wrap: false,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Normal,
            gap: 0.0,
            row_gap: 0.0,
            column_gap: 0.0,

            grow: 0.0,
            shrink: 1.0,
            basis: None,

            background: None,
            border: Insets::ZERO,
            border_color: &Color::BLACK,
            padding: Insets::ZERO,

            children: vec![],
            content: None,
        }
    }
}

impl<'a, T: Data> Widget<T> for FlexBox<'a, T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        for child in &mut self.children {
            child.event(ctx, event, data, env);
        }

        if let Some(content) = &mut self.content {
            content.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        for child in &mut self.children {
            child.lifecycle(ctx, event, data, env);
        }

        if let Some(content) = &mut self.content {
            content.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        for child in &mut self.children {
            child.update(ctx, data, env);
        }

        if let Some(content) = &mut self.content {
            content.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        let size = bc.max();

        let available = size.sub(self.border.size()).sub(self.padding.size());

        let Size {
            width: available_main,
            height: available_cross,
        } = available.transpose_if(self.direction == Axis::Vertical);

        let (main_skip, cross_skip) = (
            self.border.x0 + self.padding.x0,
            self.border.y0 + self.padding.y0,
        )
            .transpose_if(self.direction == Axis::Vertical);

        // split children into flex lines
        let mut lines: Vec<Vec<&mut WidgetPod<T, FlexBox<'a, T>>>> = vec![];
        let mut curr_line: Vec<&mut WidgetPod<T, FlexBox<'a, T>>> = vec![];
        let mut curr_line_at: f64 = main_skip;
        for child in &mut self.children {
            let child_basis_size = child.widget().basis.unwrap_or(0.0);
            if curr_line_at + child_basis_size > available_main && curr_line.len() > 0 {
                lines.push(curr_line);
                curr_line = vec![];
                curr_line_at = main_skip;
            }

            curr_line.push(child);
            curr_line_at += child_basis_size;
        }
        lines.push(curr_line);

        let num_lines = lines.len();
        let line_cross_size = available_cross / (num_lines as f64);

        // arrange children within each flex line
        let mut total_height_acc: f64 = cross_skip;
        for line in lines {
            let space_used = line.sum_by(|c| c.widget().basis.unwrap_or(0.0));
            let space_left: f64 = available_main - space_used;

            let mut curr_line_at: f64 = main_skip;

            let total_grow = line.sum_by(|c| c.widget().grow);
            let total_shrink = line.sum_by(|c| c.widget().shrink);

            for child in line {
                let diff = if space_left >= 0.0 {
                    space_left * (child.widget().grow / total_grow)
                } else {
                    space_left * (child.widget().shrink / total_shrink)
                };

                let child_main_space = child.widget().basis.unwrap_or(0.0) + diff;
                let child_size = Size::new(child_main_space, line_cross_size)
                    .transpose_if(self.direction == Axis::Vertical);

                child.layout(ctx, &BoxConstraints::tight(child_size), &data, &env);

                let child_origin = Point::new(curr_line_at, total_height_acc)
                    .transpose_if(self.direction == Axis::Vertical);

                child.set_origin(ctx, data, env, child_origin);

                curr_line_at += child_main_space;
            }

            total_height_acc += line_cross_size;
        }

        if let Some(content) = &mut self.content {
            content.layout(ctx, &BoxConstraints::tight(available), data, env);
            content.set_origin(
                ctx,
                data,
                env,
                Point::new(
                    self.border.x0 + self.padding.x0,
                    self.border.y0 + self.padding.y0,
                ),
            );
        }

        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let rect = ctx.size().to_rect();
        if let Some(color) = self.background {
            ctx.fill(rect, color);
        }

        if self.border.y0 > 0.0 {
            ctx.fill(
                Rect::from_origin_size((0.0, 0.0), (rect.width(), self.border.y0)),
                self.border_color,
            );
        }
        if self.border.x0 > 0.0 {
            ctx.fill(
                Rect::from_origin_size((0.0, 0.0), (self.border.x0, rect.height())),
                self.border_color,
            );
        }
        if self.border.y1 > 0.0 {
            ctx.fill(
                Rect::from_origin_size(
                    (0.0, rect.height() - self.border.y1),
                    (rect.width(), self.border.y1),
                ),
                self.border_color,
            );
        }
        if self.border.x1 > 0.0 {
            ctx.fill(
                Rect::from_origin_size(
                    (rect.width() - self.border.x1, 0.0),
                    (self.border.x1, rect.height()),
                ),
                self.border_color,
            );
        }

        for child in &mut self.children {
            child.paint(ctx, data, env);
        }

        if let Some(content) = &mut self.content {
            content.paint(ctx, data, env);
        }
    }
}
