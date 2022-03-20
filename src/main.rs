use macroquad::prelude::*;

fn config() -> Conf {
    Conf {
        window_title: "Template".to_string(),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

enum Scene {
    PLAY,
}

#[macroquad::main(config)]
async fn main() {
    /*
        clockwise rotation
        y = sin
        x = cos
        
        counter clockwise rotation
        x = sin
        y = cos
    */

    let mut current_scene = Scene::PLAY;

    // line
    let line = Vec2::new(screen_width()/2., 0.);

    // bob
    let grav = 1.;
    let mut force;
    let mut bob = Vec2::new(0., 0.);
    let mut angle = std::f32::consts::PI / 4.;
    let mut angle_v = 0.;
    let mut angle_a = 0.;

    loop {
        clear_background(BLACK);

        match current_scene {
            Scene::PLAY => {

                force = grav * f32::sin(angle) / screen_width()/2.;

                angle_a = -1. * force;

                angle_v += angle_a;
                angle += angle_v;


                // 300 adalah offset
                bob.x = 300. * f32::sin(angle) + screen_width()/2.;
                bob.y = 300. * f32::cos(angle);

                // draw line
                draw_line(screen_width()/2., 0., bob.x, bob.y, 3., WHITE);

                // draw circle
                draw_circle(bob.x, bob.y, 20., WHITE);

                
            },
        }

        next_frame().await
    }
}