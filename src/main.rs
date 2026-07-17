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

    /* line::line(
        &mut framebuffer,
        Vertex::new(50, 50),
        Vertex::new(450, 350),
    ); */

    // =========================
    // Pentágono (ACTIVO)
    // =========================
    let pentagon = [
        Vertex::new(250, 50),
        Vertex::new(420, 170),
        Vertex::new(360, 380),
        Vertex::new(140, 380),
        Vertex::new(80, 170),
    ];

    polygon::draw_polygon(
        &mut framebuffer,
        &pentagon,
    );

    // =========================
    // Estrella
    // =========================
    /*
    let star = [
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

    polygon::draw_polygon(
        &mut framebuffer,
        &star,
    );
    */

    // =========================
    // Cuadrilátero
    // =========================
    /*
    let quad = [
        Vertex::new(321, 335),
        Vertex::new(288, 286),
        Vertex::new(339, 251),
        Vertex::new(374, 302),
    ];

    polygon::draw_polygon(
        &mut framebuffer,
        &quad,
    );
    */

    // =========================
    // Triángulo
    // =========================
    /*
    let triangle = [
        Vertex::new(377, 249),
        Vertex::new(411, 197),
        Vertex::new(436, 249),
    ];

    polygon::draw_polygon(
        &mut framebuffer,
        &triangle,
    );
    */

    // =========================
    // Polígono grande
    // =========================
    /*
    let polygon1 = [
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

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon1,
    );
    */

    // =========================
    // Cuadrilátero pequeño
    // =========================
    /*
    let polygon2 = [
        Vertex::new(682, 175),
        Vertex::new(708, 120),
        Vertex::new(735, 148),
        Vertex::new(739, 170),
    ];

    polygon::draw_polygon(
        &mut framebuffer,
        &polygon2,
    );
    */

    framebuffer.save("my_first_image.png");

    println!("Image Saved Successfully!");
}