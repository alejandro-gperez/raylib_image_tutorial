use crate::framebuffer::Framebuffer;
use crate::line;
use crate::vertex::Vertex;

pub fn draw_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
) {
    // Un polígono necesita al menos 3 vértices
    if vertices.len() < 3 {
        return;
    }

    // Dibujar cada lado
    for i in 0..vertices.len() - 1 {
        line::line(
            framebuffer,
            vertices[i],
            vertices[i + 1],
        );
    }

    // Cerrar el polígono
    line::line(
        framebuffer,
        vertices[vertices.len() - 1],
        vertices[0],
    );
}