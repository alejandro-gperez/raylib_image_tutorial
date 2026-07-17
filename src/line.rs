use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vertex,
    end: Vertex,
) {

    // Convertimos los floats de Vector2 a enteros
    let mut x0 = start.x;
    let mut y0 = start.y;

    let x1 = end.x;
    let y1 = end.y;

    // Diferencias absolutas
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    // Dirección de avance
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Error inicial
    let mut err = dx - dy;

    loop {

        // Dibujar el píxel actual
        framebuffer.point(x0, y0);

        // Si llegamos al destino terminamos
        if x0 == x1 && y0 == y1 {
            break;
        }

        // Copia del error multiplicada por 2
        let e2 = 2 * err;

        // Avanzar en X
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        // Avanzar en Y
        if e2 < dx {
            err += dx;
            y0 += sy;
        }

    }

}