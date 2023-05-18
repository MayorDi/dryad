use dryad::app::App;


fn main() {
    let app = &mut App::new() as *mut App;

    jarylo::App::new()
        .set_title("Dryad".to_string())
        .set_event_handler(|_, _|{

        })
        .set_update(move ||{
            dryad::update::update(app);
        })
        .set_render(move |painter| {
            painter.set_color(dryad::colors::BACKGROUND);
            painter.clear();

            painter.draw_rect(0.0, 0.0, 0.0, 0.0);
            
            dryad::render::render(app, painter);
        }).build().run();
}