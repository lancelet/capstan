extern crate svg;

use nalgebra::Vector2;
use svg::node::element::path;
use svg::node::element::Circle;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::Node;
use svg::Document;

type Curve = capstan::curve::Curve<f32, Vector2<f32>>;

fn main() {
    println!("Plotting some examples");

    circle_example(&String::from("circle.svg"));
    cubic_bezier_example(&String::from("cubic-bezier.svg"));
}

fn circle_example(filename: &str) {
    let radius = 130.0;

    let mut nurbs_circle = unit_circle();
    nurbs_circle.uniform_scale(radius);
    let nurbs_group =
        curve_and_control_polygon(&nurbs_circle, 256).set("transform", "translate(150, 150)");

    let circle = style_regular(Circle::new().set("cx", 0).set("cy", 0).set("r", radius))
        .set("transform", "translate(450, 150)");

    let document = Document::new()
        .set("width", 600)
        .set("height", 300)
        .add(nurbs_group)
        .add(circle);

    svg::save(filename, &document).unwrap();
}

fn cubic_bezier_example(filename: &str) {
    let bezier = style_regular(
        Path::new().set(
            "d",
            path::Data::new()
                .move_to((80, 20))
                .cubic_curve_to((280, 280, 20, 280, 220, 20)),
        ),
    )
    .set("transform", "translate(300, 0)");

    let nurb_bezier = curve_and_control_polygon(
        &Curve::new(
            3,
            vec![
                Vector2::new(80.0, 20.0),
                Vector2::new(280.0, 280.0),
                Vector2::new(20.0, 280.0),
                Vector2::new(220.0, 20.0),
            ],
            vec![1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        )
        .unwrap(),
        256,
    );

    let document = Document::new()
        .set("width", 600)
        .set("height", 300)
        .add(nurb_bezier)
        .add(bezier);

    svg::save(filename, &document).unwrap();
}

fn style_regular<T>(node: T) -> Group
where
    T: Node,
{
    Group::new()
        .add(node)
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", "2px")
        .set("vector-effect", "non-scaling-stroke")
}

fn curve_and_control_polygon(curve: &Curve, n_divisions: usize) -> Group {
    Group::new()
        .add(curve_path(curve, n_divisions))
        .add(curve_polygon(curve))
}

fn curve_path(curve: &Curve, n_divisions: usize) -> Path {
    Path::new()
        .set("d", curve_path_data(curve, n_divisions))
        .set("fill", "none")
        .set("stroke", "#711081")
        .set("stroke-width", "2px")
        .set("vector-effect", "non-scaling-stroke")
}

fn curve_path_data(curve: &Curve, n_divisions: usize) -> path::Data {
    let min_u = curve.knots().min_u();
    let max_u = curve.knots().max_u();
    let u_range = max_u - min_u;
    let range_denom = n_divisions as f32;

    let mut commands = Vec::with_capacity(n_divisions + 1);
    commands.push(path::Command::Move(
        path::Position::Absolute,
        path::Parameters::from(eval_curve_2d(&curve, min_u)),
    ));
    for i in 1..(n_divisions + 1) {
        let u = min_u + (i as f32) * u_range / range_denom;
        commands.push(path::Command::Line(
            path::Position::Absolute,
            path::Parameters::from(eval_curve_2d(&curve, u)),
        ))
    }

    path::Data::from(commands)
}

fn curve_polygon(curve: &Curve) -> Group {
    // control points
    let cps = curve.control_points();

    // a group for the control points
    let mut control_points_group = Group::new();
    for cp in cps {
        let cp_circle = control_point(cp.x, cp.y, 3.5);
        control_points_group = control_points_group.add(cp_circle);
    }

    // the control polygon lines
    let mut commands = Vec::with_capacity(cps.len());
    commands.push(path::Command::Move(
        path::Position::Absolute,
        path::Parameters::from((cps[0].x, cps[0].y)),
    ));
    for cp in cps.iter().skip(1) {
        commands.push(path::Command::Line(
            path::Position::Absolute,
            path::Parameters::from((cp.x, cp.y)),
        ));
    }
    let path_data = path::Data::from(commands);
    let path = Path::new()
        .set("d", path_data)
        .set("fill", "none")
        .set("stroke", "#101010")
        .set("stroke-width", "1px")
        .set("stroke-dasharray", "4 3")
        .set("vector-effect", "non-scaling-stroke");

    Group::new().add(path).add(control_points_group)
}

fn control_point(x: f32, y: f32, radius: f32) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", radius)
        .set("fill", "#AAAAAA")
        .set("stroke", "#000000")
        .set("stroke-width", "1px")
        .set("vector-effect", "non-scaling-stroke")
}

fn eval_curve_2d(curve: &Curve, u: f32) -> (f32, f32) {
    let pt_3d = curve.de_boor(u);
    (pt_3d.x, pt_3d.y)
}

fn unit_circle() -> Curve {
    let r = f32::sqrt(2.0) / 2.0;
    let degree = 2;
    let control_points = vec![
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 1.0),
        Vector2::new(-1.0, 0.0),
        Vector2::new(-1.0, -1.0),
        Vector2::new(0.0, -1.0),
        Vector2::new(1.0, -1.0),
        Vector2::new(1.0, 0.0),
    ];
    let weights = vec![1.0, r, 1.0, r, 1.0, r, 1.0, r, 1.0];
    let knots = vec![0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0];
    Curve::new(degree, control_points, weights, knots).unwrap()
}
