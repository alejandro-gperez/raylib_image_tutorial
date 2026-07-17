mod framebuffer;
mod line;
mod vertex;
mod polygon;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use vertex::Vertex;

fn main() {
    let mut framebuffer = Framebuffer::new(
        1000,
        1000,
        Color::PURPLE,
    );

    framebuffer.set_color(Color::YELLOW);

    // =========================
    // Polígono 1
    // =========================
    let polygon1 = [
        Vertex::new(165, 380),
        Vertex::new(185, 360),
        Vertex::new(180, 330),
        Vertex::new(207, 345),
        Vertex::new(233, 330),
        Vertex::new(230, 360),
        Vertex::new(250, 380),
        Vertex::new(220, 385),
        Vertex::new(205, 410),
        Vertex::new(193, 383),
    ];

    polygon::fill_polygon(
        &mut framebuffer,
        &polygon1,
    );

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon1,
    );

    // =========================
    // Polígono 2
    // =========================
    let polygon2 = [
        Vertex::new(321, 335),
        Vertex::new(288, 286),
        Vertex::new(339, 251),
        Vertex::new(374, 302),
    ];

    polygon::fill_polygon(
        &mut framebuffer,
        &polygon2,
    );

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon2,
    );

    // =========================
    // Polígono 3
    // =========================
    let polygon3 = [
        Vertex::new(377, 249),
        Vertex::new(411, 197),
        Vertex::new(436, 249),
    ];

    polygon::fill_polygon(
        &mut framebuffer,
        &polygon3,
    );

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon3,
    );

    // =========================
    // Polígono 4
    // =========================
    let polygon4 = [
        Vertex::new(413, 177),
        Vertex::new(448, 159),
        Vertex::new(502, 88),
        Vertex::new(553, 53),
        Vertex::new(535, 36),
        Vertex::new(676, 37),
        Vertex::new(660, 52),
        Vertex::new(750, 145),
        Vertex::new(761, 179),
        Vertex::new(672, 192),
        Vertex::new(659, 214),
        Vertex::new(615, 214),
        Vertex::new(632, 230),
        Vertex::new(580, 230),
        Vertex::new(597, 215),
        Vertex::new(552, 214),
        Vertex::new(517, 144),
        Vertex::new(466, 180),
    ];

    // =========================
    // Polígono 5
    // =========================
    let polygon5 = [
        Vertex::new(682, 175),
        Vertex::new(708, 120),
        Vertex::new(735, 148),
        Vertex::new(739, 170),
    ];

    polygon::fill_polygon_with_holes(
        &mut framebuffer,
        &polygon4,
        &[&polygon5],
    );

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon4,
    );

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon5,
    );

    framebuffer.save("out.png");

    println!("Image Saved Successfully!");
}
