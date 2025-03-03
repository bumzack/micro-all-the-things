use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;
use warp::ws::WebSocket;

use crate::index_html::INDEX_HTML;

mod index_html;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let get_scenes = warp::get()
        .and(warp::path("scenes"))
        // Only accept bodies smaller than 16kb...
        .map(|| {
            // let scenes = json!(get_scenes_dtos()).to_string();
            warp::reply::json(&"all good".to_string())
        });

    // GET /chat -> websocket upgrade
    let render = warp::path("openwebsocket")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| render_scene(socket))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(render).or(get_scenes);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}

async fn render_scene(_ws: WebSocket) {
    // let scenes = get_scenes_dtos();
    // let (mut websocket_tx, mut websocket_rx) = ws.split();
    //
    // // wait for a message, which contains infos about the scene
    // let w = websocket_rx.next().await.unwrap();
    //
    // match w {
    //     Ok(world_tmp) => {
    //         info!("got a message  {:?}", world_tmp.to_str());
    //         let scene_data: SceneConfig =
    //             serde_json::from_str(world_tmp.to_str().unwrap()).unwrap();
    //
    //         info!("worldscene {:?}", &scene_data);
    //
    //         let width = scene_data.get_width();
    //         let height = scene_data.get_height();
    //         let wi = scene_data.get_width();
    //         let h = scene_data.get_height();
    //
    //         let id = scene_data.get_id();
    //         let scene = scenes
    //             .get_scenes()
    //             .iter()
    //             .find(|s| s.get_id() == id)
    //             .unwrap();
    //         let (mut w, mut c) = (scene.get_world().clone(), scene.get_camera().clone());
    //         c.set_from(Tuple4D::new_point(
    //             scene_data.get_from().x,
    //             scene_data.get_from().y,
    //             scene_data.get_from().z,
    //         ));
    //         c.set_to(Tuple4D::new_point(
    //             scene_data.get_to().x,
    //             scene_data.get_to().y,
    //             scene_data.get_to().z,
    //         ));
    //         c.set_up(Tuple4D::new_vector(
    //             scene_data.get_up().x,
    //             scene_data.get_up().y,
    //             scene_data.get_up().z,
    //         ));
    //         c.set_field_of_view(scene_data.get_fov());
    //         c.set_width(scene_data.get_width());
    //         c.set_height(scene_data.get_height());
    //         c.set_antialiasing(scene_data.get_antialiasing() > 0);
    //         c.calc_pixel_size();
    //
    //         w.get_light_mut().iter_mut().for_each(|l| {
    //             match l {
    //                 Light::AreaLight(ref mut area_light) => {
    //                     area_light.set_usteps(scene_data.get_size_area_light());
    //                     area_light.set_vsteps(scene_data.get_size_area_light())
    //                 }
    //                 _ => (),
    //             };
    //         });
    //
    //         let (s, recv_web_sockets) = unbounded::<TileData>();
    //         let recv_canvas = recv_web_sockets.clone();
    //
    //         tokio::task::spawn(async move {
    //             let start = Instant::now();
    //             Camera::render_multi_core_tile_producer(&c, &w, 15, 15, s);
    //             let dur = Instant::now() - start;
    //             info!("async render_scene  multi core duration: {:?}", dur);
    //         });
    //
    //         tokio::task::spawn(async move {
    //             let mut cnt = 1;
    //             let mut canvas = Canvas::new(wi, h);
    //
    //             loop {
    //                 let td = recv_web_sockets.recv();
    //                 match td {
    //                     Ok(tile_data) => {
    //                         info!("warp backend got a tile idx {}", tile_data.get_idx());
    //
    //                         tile_data.get_points().iter().for_each(|p| {
    //                             canvas.write_pixel(p.get_x(), p.get_y(), p.get_color());
    //                         });
    //                         let start = Instant::now();
    //                         let tile_data_json = json!(tile_data).to_string();
    //                         let dur = Instant::now() - start;
    //                         info!("serialization took: {:?}", dur);
    //                         let start = Instant::now();
    //                         let msg = Message::text(tile_data_json);
    //                         let dur = Instant::now() - start;
    //                         info!("wrapping in message took: {:?}", dur);
    //                         websocket_tx
    //                             .send(msg)
    //                             .unwrap_or_else(|e| {
    //                                 error!("websocket send error: {}", e);
    //                             })
    //                             .await;
    //                         cnt += 1;
    //                     }
    //                     Err(e) => {
    //                         info!("no more tiles available: {}", e);
    //                         break;
    //                     }
    //                 }
    //             }
    //
    //             let filename = &format!(
    //                 "./webui_test_soft_shadow_multiple_lights_{}x{}.png",
    //                 width, height
    //             );
    //             canvas.write_png(filename).expect("write file");
    //         });
    //
    //         // tokio::task::spawn(async move {
    //         //     let start = Instant::now();
    //         //     let canvas = Camera::collect_tiles_to_canvas(recv_canvas , width, height);
    //         //     let dur = Instant::now() - start;
    //         //     info!("async collect_tiles_to_canvas      duration: {:?}", dur);
    //         //     let filename = &format!(
    //         //         "./webui_test_soft_shadow_multiple_lights_{}x{}.png",
    //         //         width,
    //         //         height,
    //         //     );
    //         //     canvas.write_png(filename).expect("write file");
    //         // });
    //     }
    //     _ => {}
    // }
}
