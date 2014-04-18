use sdl2;
use sdl2_gfx;
use sdl2_ttf;
use sdl2::video;
use sdl2::{event, keycode};
use sdl2::pixels;
use sdl2_image::LoadSurface;
use sdl2_image::LoadTexture;
use sdl2_gfx::primitives::DrawRenderer;
use sdl2::pixels::{Color, RGB, RGBA};
use std::iter::range_step;
use std::cmp::max;
use rand::random;


static SCREEN_WIDTH : int = 800;
static SCREEN_HEIGHT : int = 600;


/* colors
bg #eee4da fg #f9f6f2

#f59563
#f67c5f
#f65e3b
#edcf72
#edcc61
#edc850
 #edc53f
#edc22e
#3c3a32
*/

// hadle the annoying Rect i32

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

// colors
static BG_COLOR: Color = RGB(0xee, 0xe4, 0xda);
static FG_COLOR: Color = RGB(0xf9, 0xf6, 0xf2);
static CHAR_COLOR: Color = RGB(0xee, 0x33, 0x66);
static CONTAINER_COLOR: Color = RGBA(0x77, 0x6e, 0x65, 200);
static CELL_COLORS: &'static [Color] = &'static [
    RGBA(0xee, 0xe4, 0xda, 120), RGB(0xed, 0xe0, 0xc8), RGB(0xf2, 0xb1, 0x79),
    RGB(0xf5, 0x95, 0x64), RGB(0xf6, 0x7c, 0x5f), RGB(0xf6, 0x5e, 0x3b),
    RGB(0xed, 0xcf, 0x72), RGB(0xed, 0xcc, 0x61), RGB(0xed, 0xc8, 0x50),
    RGB(0xed, 0xc5, 0x3f), RGB(0xed, 0xc2, 0x2e), RGB(0x3c, 0x3a, 0x32), ];


static WIDTH  : int = 4;
static HEIGHT : int = 4;

struct Game {
    pub grid: [[int, ..WIDTH], ..HEIGHT],
    pub score: int,
}

impl Game {
    pub fn new() -> Game {
        Game { grid: [[0, ..WIDTH], ..HEIGHT],
               score: 0,
        }
    }

        pub fn int_to_vec(dir: int) -> (int, int) /* x, y */
    {
        match dir
        {
            0 => (1, 0),    /* RIGHT */
            1 => (-1, 0),   /* LEFT */
            2 => (0, 1),    /* DOWN */
            3 => (0, -1),   /* UP */
            _ => (42, 42)   /* Wait what ERROR */
        }
    }

    /* 0022 -> 1 | 0222 -> 2 | 2222 -> 3 */
    pub fn get_lenght(self, vec: (int, int), i: int, j: int) -> int
    {
        let (x, y) = vec;
        let mut c = 1;

        while i+x*c >= 0 && j+y*c >= 0 && i+x*c < WIDTH && j+y*c < HEIGHT &&
              self.grid[i as uint][j as uint] == self.grid[(i+x*c) as uint][(j+y*c) as uint]
        {
            c+=1;
        }

        c
    }

    pub fn move_global(&mut self, vec: (int, int)) /* Move without merge */
    {
        let (x, y) = vec;

        /* Move enough times to move everything (soooo beautiful~) */
        for _ in range(0, max(WIDTH, HEIGHT)/2)
        {
            /* WIDTH-1 to 0 if x<0, 0 to WIDTH-1 if x>=0 */
            let mut w = if x < 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
            for i in w
            {
                /* HEIGHT-1 to 0 if x<0, 0 to HEIGHT-1 if x>=0 */
                let mut h = if y < 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
                for j in h
                {
                    /* If the current tile is full and the next is empty : swap */
                    if i+x >= 0 && j+y >= 0 && i+x < WIDTH && j+y < HEIGHT &&
                       self.grid[i as uint][j as uint] != 0 && self.grid[(i+x) as uint][(j+y) as uint] == 0
                    {
                        let tmp = self.grid[(i+x) as uint][(j+y) as uint];
                        self.grid[(i+x) as uint][(j+y) as uint] = self.grid[i as uint][j as uint];
                        self.grid[i as uint][j as uint] = tmp;
                    }
                }
            }
        }
    }

   pub fn merge_seq(&mut self, vec: (int, int), i: int, j: int)
    {
        let l = self.get_lenght(vec, i, j) - 1;
        let (x, y) = vec;

        /* 0022 -> ok (min), 0002 -> lolnope */
        if l >= 1
        {
            /* End of the sequence to the start+1 */
            for k in range_step(l, 0, -2)
            {
                /* If both tiles are equals */
                if self.grid[(i+x*k) as uint][(j+y*k) as uint] == self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint]
                {
                    // self.merged_nb+=1;
                }

                self.grid[(i+x*k) as uint][(j+y*k) as uint] += self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint];
                self.score += self.grid[(i+x*k) as uint][(j+y*k) as uint];
                self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint] = 0;

            }
        }
    }

    pub fn merge(&mut self, vec: (int, int))
    {
        let (x, y) = vec;

        /* WIDTH-1 to 0 if x<0, 0 to WIDTH-1 if x>=0 */
        let mut w = if x >= 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
        for i in w
        {
            /* HEIGHT-1 to 0 if x<0, 0 to HEIGHT-1 if x>=0 */
            let mut h = if y >= 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
            for j in h
            {
                if i+x >= 0 && j+y >= 0 && i+x < WIDTH && j+y < HEIGHT && self.grid[i as uint][j as uint] != 0
                {
                    self.merge_seq(vec, i, j);
                }
            }
        }
    }

    pub fn move(&mut self, vec: (int, int))
    {
        // self.move_nb+=1;
        self.move_global(vec);
        self.merge(vec);
        self.move_global(vec); /* Plug holes \o/ */
    }

    pub fn is_moved(g1: Game, g2: Game) -> bool
    {
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if g1.grid[i as uint][j as uint] != g2.grid[i as uint][j as uint]
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn list_move(self) -> ~[int]
    {
        let mut tmp: Game;
        let mut ret: ~[int] = ~[];

        /* Tries to move the grid in each direction, and sees if there have been any changes */
        for i in range(0, 4)
        {
            tmp = self;
            tmp.move(Game::int_to_vec(i));

            if Game::is_moved(self, tmp) == true
            {
                ret.push(i);
            }
        }

        ret
    }

    pub fn list_tile_empty(&mut self) -> ~[(int, int)]
    {
        let mut ret: ~[(int, int)] = ~[];

        /* List the position of all empty tiles */
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if self.grid[i as uint][j as uint] == 0
                {
                    ret.push((i, j));
                }
            }
        }

        ret
    }

    pub fn add_random_tile(&mut self)
    {
        let tab = self.list_tile_empty();

        /* If there is at least one empty tile */
        if tab.len() > 0
        {
            /* Chooses a random position and add the new tile */
            let (a, b) = tab[random::<uint>()%tab.len()];
            self.grid[a as uint][b as uint] = 2;
        }
    }

    pub fn draw_on(&self, ren: &sdl2::render::Renderer, font: &sdl2_ttf::Font,
                   (x,y,w,h): (int,int,int,int)) -> Result<(), ~str> {
        assert_eq!(w, h);
        // BEST in 500x500
        static CONTAINER_PADDING: int = 10;
        let cell_width = (w - CONTAINER_PADDING * 5) / 4;
        assert!(cell_width > 50); // Min width
        ren.box_(x as i16, y as i16, (x+w) as i16, (y+h) as i16, CONTAINER_COLOR);
        for i in range(0, HEIGHT) {
            for j in range(0, WIDTH) {
                let val = self.grid[i as uint][j as uint];
                let c = if val == 0 {
                    0
                } else {
                    (val as f64).log2() as uint
                };
                println!("c => {}", c);
                let bx = (x + CONTAINER_PADDING * (j + 1) + cell_width * j) as i16;
                let by = (y + CONTAINER_PADDING * (i + 1) + cell_width * i) as i16;
                ren.box_(bx, by, bx + cell_width as i16, by + cell_width as i16,
                         CELL_COLORS[c]);
                if val != 0 {
                    let (tex, w, h) = {
                        let wd = format!("{}", val);
                        let (w, h) = try!(font.size_of_str(wd));
                        let text = try!(font.render_str_blended(wd, RGB(0x77, 0x6e, 0x65)));
                        (try!(ren.create_texture_from_surface(text)), w, h)
                    };
                    if h > w {
                        ren.copy(tex, None, Some(rect!(bx as int + cell_width / 2 - w/2, by as int + cell_width / 2 - h/2,
                                                       w, h)));
                    } else {
                        ren.copy(tex, None, Some(rect!(bx as int + cell_width / 2 - w/2, by as int + cell_width / 2 - h/2,
                                                       w, h)));
                    }
                }
            }
        }
        Ok(())
    }
}



fn draw_title(ren: &sdl2::render::Renderer, font: &sdl2_ttf::Font) -> Result<(), ~str> {
    let (tex2, w, h) = {
        let wd = "Rust - 2048";
        //font.set_style([sdl2_ttf::StyleBold]);
        let (w, h) = try!(font.size_of_str(wd));
        let text = try!(font.render_str_blended(wd, RGB(0x77, 0x6e, 0x65)));
        (try!(ren.create_texture_from_surface(text)), w, h)
    };
    ren.copy(tex2, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2, 20, w, h)));
    Ok(())
}

pub fn run() -> Result<(), ~str> {
    let mut frames = 0;
    let win = try!(video::Window::new(
        "Rust - 2048", video::PosCentered, video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        [video::Shown]));

    let ren = try!(sdl2::render::Renderer::from_window(
        win, sdl2::render::DriverAuto, [sdl2::render::Accelerated]));

    let mut fpsm = sdl2_gfx::framerate::FPSManager::new();
    fpsm.set_framerate(50);

    let font = try!(sdl2_ttf::Font::from_file(&Path::new("./xiaonaipao.ttf"), 48));
    let mut game = Game::new();

    let mut playing = false;

    'main : loop {
        'event : loop {
            fpsm.framerate_delay();
            ren.set_draw_color(BG_COLOR);
            ren.clear();
            // main drawing
            draw_title(&*ren, &*font);
            ren.string(0i16, 0i16, format!("frames: {}", frames), CHAR_COLOR);

            ren.string(200, 90, format!("your score: {}", game.score), CHAR_COLOR);

            if !playing {
                ren.string(0, 20, "Press SPACE to start!", CHAR_COLOR);
            }

            game.draw_on(&*ren, &*font, (SCREEN_WIDTH / 2 - 400 / 2, 100, 400, 400));

            // main drawing ends
            ren.present();

            frames += 1;

            match event::poll_event() {
                event::QuitEvent(_) => break 'main,
                event::KeyDownEvent(_, _, keycode::LeftKey, _, _) if playing => {
                    game.move((-1, 0));
                    game.add_random_tile();
                }
                event::KeyDownEvent(_, _, keycode::RightKey, _, _) if playing => {
                    game.move((1, 0));
                    game.add_random_tile();
                }
                event::KeyDownEvent(_, _, keycode::UpKey, _, _) if playing => {
                    game.move((0, -1));
                    game.add_random_tile();
                }
                event::KeyDownEvent(_, _, keycode::DownKey, _, _) if playing => {
                    game.move((0, -0));
                    game.add_random_tile();
                }
                event::KeyDownEvent(_, _, key, _, _) => {
                    if key == keycode::EscapeKey {
                        break 'main
                    } else if key == keycode::SpaceKey {
                        if !playing {
                            playing = true;
                            game.add_random_tile();
                            game.add_random_tile();
                        }
                    }

                }
                event::MouseButtonDownEvent(_, _, _, _, x, y) => {
                    println!("mouse btn down at ({},{})", x, y);
                }
                event::MouseMotionEvent(_, _, _, _, x, y, dx, dy) => {
                         //println!("mouse btn move at ({},{}) d-> {} {}", x, y, dx, dy);

                }
                _ => {}
            }
        }
    }
    Ok(())
}