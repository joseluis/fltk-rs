use fltk::{app, frame::*, image::*, window::*};

const PXM: &[&str] = &[
    "50 34 4 1",
    "  c #000000",
    "o c #ff9900",
    "@ c #ffffff",
    "# c None",
    "##################################################",
    "###      ##############################       ####",
    "### ooooo  ###########################  ooooo ####",
    "### oo  oo  #########################  oo  oo ####",
    "### oo   oo  #######################  oo   oo ####",
    "### oo    oo  #####################  oo    oo ####",
    "### oo     oo  ###################  oo     oo ####",
    "### oo      oo                     oo      oo ####",
    "### oo       oo  ooooooooooooooo  oo       oo ####",
    "### oo        ooooooooooooooooooooo        oo ####",
    "### oo     ooooooooooooooooooooooooooo    ooo ####",
    "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
    "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
    "##### oo oooooooo ooooooooooooo oooooooo oo ######",
    "#####  o ooooooooooooooooooooooooooooooo o  ######",
    "###### ooooooooooooooooooooooooooooooooooo #######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "###### oooooooooooooo       oooooooooooooo #######",
    "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
    "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
    "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
    "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
    "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
    "########### @@@@@@@     @     @@@@@@@ ############",
    "############  @@@@@@@@@@@@@@@@@@@@@  #############",
    "##############  @@@@@@@@@@@@@@@@@  ###############",
    "################    @@@@@@@@@    #################",
    "####################         #####################",
    "##################################################",
];

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("pxm test")
        .with_size(720, 486)
        .center_screen();
    let mut frame = Frame::new(-30, 200, 30, 30, "");
    let mut pxm = unsafe { Pixmap::new(PXM).unwrap() };
    pxm.scale(200, 200, true, true);
    frame.set_image(Some(pxm));
    wind.set_color(Color::White);
    wind.end();
    wind.show_with_env_args();

    app::add_idle(move || {
        let x = frame.x();
        let y = frame.y();
        let w = frame.width();
        let h = frame.height();
        if x > wind.width() + w + 30 {
            app.quit();
        }
        frame.resize(x + 5, y, w, h);
        wind.redraw();
        app::sleep(0.016);
    });

    app.run().unwrap();
}

// // Another way is using app::add_timeout and app::repeat_timout

// fn move_image(mut frm: Frame) {
//     let (x, y) = (frm.x(), frm.y());
//     frm.set_pos(x + 5, y);
//     redraw();
//     app::repeat_timeout(0.016, move || {
//         let frm = frm.clone();
//         move_image(frm);
//     });
// }

// fn main() {
//     let app = app::App::default();
//     let mut wind = Window::default()
//         .with_label("pxm test")
//         .with_size(720, 486)
//         .center_screen();
//     let mut frame = Frame::new(-30, 200, 30, 30, "");
//     let mut pxm = unsafe { Pixmap::new(PXM).unwrap() };
//     pxm.scale(200, 200, true, true);
//     frame.set_image(Some(pxm));
//     wind.set_color(Color::White);
//     wind.end();
//     wind.show_with_env_args();

//     app::add_timeout(0.016, move || {
//         let frame = frame.clone();
//         move_image(frame);
//     });
//     app.run().unwrap();
// }
