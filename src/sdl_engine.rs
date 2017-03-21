use sdl2;

pub struct SdlEngine<'a> {
    pub sdl: Option<sdl2::Sdl>,
    pub video: Option<sdl2::VideoSubsystem>,
    pub renderer: Option<sdl2::render::Renderer<'a>>,
    pub event_pump: Option<sdl2::EventPump>,
}

pub fn new<'a>(window_title: &str, window_width: u32, window_height: u32) -> SdlEngine<'a> {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();

    let window = sdl_video
        .window(window_title, window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let renderer = window
        .renderer()
        .accelerated()
        .build()
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    SdlEngine {
        sdl: Some(sdl_context),
        video: Some(sdl_video),
        renderer: Some(renderer),
        event_pump: Some(event_pump),
    }
}

impl<'a> SdlEngine<'a> {
    pub fn renderer(&mut self) -> &mut sdl2::render::Renderer<'a> {
        self.renderer.as_mut().unwrap()
    }

    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        self.event_pump.as_mut().unwrap()
    }
}
