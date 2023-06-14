use raylib::prelude::*;
use crate::renderer::Render;
use crate::Color as VColor;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2d {
    pub x:f32,
    pub y:f32,
}
impl Vector2d {
    fn to_vec2(&self) -> Vector2 {
        Vector2 { x: self.x, y:self.y }
    }
}
impl Into<Vector2> for Vector2d {
    fn into(self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub pos:Vector2d,
    pub size:Vector2d,
    pub fill:VColor
}
impl Rectangle {
    pub fn new(pos:Vector2d,size:Vector2d,fill:VColor) -> Self {
        Self { pos,size,fill }
    }
}
impl Render for Rectangle {
    fn draw(&self,draw_tool:&mut RaylibDrawHandle) {
        draw_tool.draw_rectangle_v( self.pos.to_vec2(), self.size.to_vec2(),self.fill.to_color());
    }
}
#[derive(Debug, Clone)]
pub struct Circle {
    pub pos:Vector2d,
    pub radius:f32,
    pub fill:VColor
}
impl Circle {
    pub fn new(pos:Vector2d,radius:f32,fill:VColor) -> Self {
        Self { pos,radius,fill }
    }
}
impl Render for Circle {
    fn draw(&self,draw_tool:&mut RaylibDrawHandle) {
        draw_tool.draw_circle_v(self.pos.to_vec2(), self.radius, self.fill.to_color())
    }
}
#[derive(Debug, Clone)]
pub struct Polygon {
    pub pos:Vector2d,
    pub radius:f32,
    pub sides: u16,
    pub rotation:f32,
    pub fill:VColor
}
impl Polygon {
    pub fn new(pos:Vector2d,radius:f32,sides:u16,rotation:f32,fill:VColor) -> Self {
        // makes sure there is atleast 3 sides
        assert!(sides > 2);
        // makes sure the radius is greater that 0
        assert!(radius > 0.);
        Self { pos,radius,sides,rotation,fill }
    }
}
impl Render for Polygon {
    fn draw(&self,draw_tool:&mut RaylibDrawHandle) {
        draw_tool.draw_poly(self.pos.to_vec2(), self.sides as i32, self.radius, self.rotation, self.fill.to_color());
    }
}
#[derive(Debug, Clone)]
pub struct Line {
    p1:Vector2d,
    p2:Vector2d,
    thickness:f32,
    fill:VColor
}
impl Line {
    pub fn new(pos1:Vector2d,pos2:Vector2d,thickness:f32,fill:VColor) -> Self {
        Self {p1:pos1,p2:pos2, thickness, fill}
    }
}
impl Render for Line {
    fn draw(&self,draw_tool:&mut RaylibDrawHandle) {
        draw_tool.draw_line_ex(self.p1.to_vec2(), self.p2.to_vec2(), self.thickness, self.fill.to_color());
    }
}
#[derive(Debug, Clone)]
pub struct LinedCircle  {
    pos:Vector2d,
    radius:f32,
    thickness:i32,
    fill:VColor
}
impl LinedCircle {
    pub fn new(pos:Vector2d,radius:f32,thickness:i32,fill:VColor) -> Self {
        Self { pos, radius, thickness, fill }
    }
}
impl Render for LinedCircle {
    fn draw(&self,draw_tool:&mut RaylibDrawHandle) {
        for i in 0..self.thickness {
            draw_tool.draw_circle_lines(self.pos.x.floor() as i32, self.pos.y.floor() as i32, self.radius - i as f32, self.fill.to_color());
        }
    }
}
