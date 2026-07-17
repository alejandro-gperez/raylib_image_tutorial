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

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
) {
    fill_polygon_with_holes(
        framebuffer,
        vertices,
        &[],
    );
}

pub fn fill_polygon_with_holes(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
    holes: &[&[Vertex]],
) {
    // Un polígono necesita al menos 3 vértices
    if vertices.len() < 3 {
        return;
    }

    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;

    for vertex in vertices {
        if vertex.y < min_y {
            min_y = vertex.y;
        }

        if vertex.y > max_y {
            max_y = vertex.y;
        }
    }

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        add_intersections(
            &mut intersections,
            vertices,
            y,
        );

        for hole in holes {
            add_intersections(
                &mut intersections,
                hole,
                y,
            );
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;

        while i + 1 < intersections.len() {
            let start_x = intersections[i].ceil() as i32;
            let end_x = intersections[i + 1].floor() as i32;

            for x in start_x..=end_x {
                framebuffer.point(x, y);
            }

            i += 2;
        }
    }
}

fn add_intersections(
    intersections: &mut Vec<f32>,
    vertices: &[Vertex],
    y: i32,
) {
    if vertices.len() < 3 {
        return;
    }

    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];

        // Las aristas horizontales no agregan una entrada o salida nueva.
        if start.y == end.y {
            continue;
        }

        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        // Intervalo semiabierto para no contar dos veces los vértices compartidos.
        if y >= min_y && y < max_y {
            let x = start.x as f32
                + (y - start.y) as f32
                * (end.x - start.x) as f32
                / (end.y - start.y) as f32;

            intersections.push(x);
        }
    }
}
